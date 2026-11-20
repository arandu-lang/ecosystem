use anyhow::{Result, anyhow};
use clang_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::path::Path;

use crate::ast::*;
use crate::types::*;

pub struct ParserOptions {
    pub c_flags: Vec<String>,
    pub main_file_only: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            c_flags: vec!["-std=c11".to_string()],
            main_file_only: true,
        }
    }
}

pub unsafe fn get_children(cursor: CXCursor) -> Vec<CXCursor> {
    let mut children = Vec::new();
    extern "C" fn visitor(
        child: CXCursor,
        _parent: CXCursor,
        data: *mut c_void,
    ) -> CXChildVisitResult {
        unsafe {
            let list = &mut *(data as *mut Vec<CXCursor>);
            list.push(child);
            CXChildVisit_Continue
        }
    }
    clang_visitChildren(cursor, visitor, &mut children as *mut _ as *mut c_void);
    children
}

pub unsafe fn try_parse_offsetof(cursor: CXCursor) -> Option<i64> {
    let mut cur = cursor;
    for _ in 0..5 {
        if cur.kind == CXCursor_UnexposedExpr || cur.kind == CXCursor_ParenExpr {
            let ch = get_children(cur);
            if ch.len() == 1 {
                cur = ch[0];
            } else if ch.len() >= 2 {
                break;
            } else {
                return None;
            }
        } else {
            break;
        }
    }
    let children = get_children(cur);
    if children.len() >= 2 {
        let f_kind = children[0].kind;
        let s_kind = children[1].kind;
        if (f_kind == CXCursor_TypeRef || f_kind == CXCursor_StructDecl)
            && (s_kind == CXCursor_MemberRef || s_kind == CXCursor_FieldDecl)
        {
            let ty = clang_getCursorType(children[0]);
            let can_ty = clang_getCanonicalType(ty);
            let member_name = get_cursor_spelling(children[1]);
            if let Ok(c_member) = std::ffi::CString::new(member_name.as_str()) {
                let mut offset = clang_Type_getOffsetOf(can_ty, c_member.as_ptr());
                if offset < 0 {
                    offset = clang_Type_getOffsetOf(ty, c_member.as_ptr());
                }
                if offset >= 0 {
                    return Some(offset / 8);
                }
            }
        }
    }
    None
}

pub unsafe fn parse_expr(cursor: CXCursor) -> CExpr {
    if cursor.kind == CXCursor_TypeRef {
        let ty = c_type_to_arandu(clang_getCursorType(cursor));
        if ty == "f64" || ty == "f32" {
            return CExpr::FloatLiteral(0.0);
        } else if ty.starts_with("ptr[") {
            return CExpr::Ident("nil".to_string());
        } else {
            return CExpr::IntLiteral(0);
        }
    }

    if cursor.kind == CXCursor_UnaryExpr {
        let eval = clang_Cursor_Evaluate(cursor);
        if !eval.is_null() {
            let kind = clang_EvalResult_getKind(eval);
            if kind == CXEval_Int {
                let val = clang_EvalResult_getAsInt(eval);
                clang_EvalResult_dispose(eval);
                return CExpr::IntLiteral(val as i64);
            }
            clang_EvalResult_dispose(eval);
        }
    }

    match cursor.kind {
        CXCursor_IntegerLiteral => {
            let eval = clang_Cursor_Evaluate(cursor);
            if !eval.is_null() {
                let kind = clang_EvalResult_getKind(eval);
                if kind == CXEval_Int {
                    let v = clang_EvalResult_getAsInt(eval);
                    clang_EvalResult_dispose(eval);
                    return CExpr::IntLiteral(v as i64);
                }
                clang_EvalResult_dispose(eval);
            }
            let tokens = get_cursor_tokens(cursor);
            let s = tokens.first().cloned().unwrap_or_default();
            let clean = s.trim_end_matches(['u', 'U', 'l', 'L']);
            let val = if clean.starts_with("0x") || clean.starts_with("0X") {
                i64::from_str_radix(&clean[2..], 16).unwrap_or(0)
            } else {
                clean.parse::<i64>().unwrap_or(0)
            };
            CExpr::IntLiteral(val)
        }
        CXCursor_CharacterLiteral => {
            let eval = clang_Cursor_Evaluate(cursor);
            let val = if !eval.is_null() && clang_EvalResult_getKind(eval) == CXEval_Int {
                let v = clang_EvalResult_getAsInt(eval);
                clang_EvalResult_dispose(eval);
                v as i64
            } else {
                let s = get_cursor_token_string(cursor);
                if s.starts_with('\'') && s.ends_with('\'') && s.len() == 3 {
                    s.chars().nth(1).map(|c| c as i64).unwrap_or(0)
                } else if s == "'\\v'" {
                    11
                } else if s == "'\\f'" {
                    12
                } else if s == "'\\n'" {
                    10
                } else if s == "'\\t'" {
                    9
                } else if s == "'\\r'" {
                    13
                } else if s == "'\\0'" {
                    0
                } else {
                    0
                }
            };
            CExpr::IntLiteral(val)
        }
        CXCursor_FloatingLiteral => {
            let eval = clang_Cursor_Evaluate(cursor);
            if !eval.is_null() {
                let kind = clang_EvalResult_getKind(eval);
                if kind == CXEval_Float {
                    let v = clang_EvalResult_getAsDouble(eval);
                    clang_EvalResult_dispose(eval);
                    return CExpr::FloatLiteral(v);
                }
                clang_EvalResult_dispose(eval);
            }
            let tokens = get_cursor_tokens(cursor);
            let s = tokens.first().cloned().unwrap_or_default();
            let clean = s.trim_end_matches(['f', 'F', 'l', 'L']);
            let val = clean.parse::<f64>().unwrap_or(0.0);
            CExpr::FloatLiteral(val)
        }
        CXCursor_StringLiteral => {
            let spelling = get_cursor_spelling(cursor);
            let s = if !spelling.is_empty() {
                spelling
            } else {
                get_cursor_token_string(cursor)
            };
            CExpr::StringLiteral(s)
        }
        CXCursor_AddrLabelExpr => CExpr::Ident("nil".to_string()),
        CXCursor_GNUNullExpr => CExpr::Ident("nil".to_string()),
        CXCursor_UnaryExpr => {
            let eval = clang_Cursor_Evaluate(cursor);
            if !eval.is_null() {
                let kind = clang_EvalResult_getKind(eval);
                if kind == CXEval_Int {
                    let v = clang_EvalResult_getAsInt(eval);
                    clang_EvalResult_dispose(eval);
                    return CExpr::IntLiteral(v as i64);
                }
                clang_EvalResult_dispose(eval);
            }
            let tokens = get_cursor_tokens(cursor);
            if tokens.iter().any(|t| t == "sizeof" || t == "offsetof") {
                return CExpr::IntLiteral(0);
            }
            CExpr::Raw(get_cursor_token_string(cursor))
        }
        CXCursor_DeclRefExpr => {
            let name = get_cursor_spelling(cursor);
            if name == "true" {
                CExpr::BoolLiteral(true)
            } else if name == "false" {
                CExpr::BoolLiteral(false)
            } else if name == "l_uacInt" || name == "luacInt" {
                CExpr::IntLiteral(0)
            } else if name == "l_uacNumber" || name == "luacNumber" {
                CExpr::FloatLiteral(0.0)
            } else {
                CExpr::Ident(name)
            }
        }
        CXCursor_UnexposedExpr => {
            if let Some(offset) = try_parse_offsetof(cursor) {
                return CExpr::IntLiteral(offset);
            }
            let children = get_children(cursor);
            if let Some(first) = children.first() {
                let parent_ty = c_type_to_arandu(clang_getCursorType(cursor));
                let child_ty = c_type_to_arandu(clang_getCursorType(*first));
                let sub_expr = parse_expr(*first);
                // Se for conversão implícita entre tipos escalares ou ponteiros:
                if !parent_ty.is_empty() && !child_ty.is_empty() && parent_ty != child_ty && !parent_ty.contains("/*") && !child_ty.contains("/*") {
                    let is_scalar = |t: &str| {
                        !t.is_empty() && (
                            [
                                "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize", "isize", "int", "uint", "f32", "f64",
                                "LuByte", "LuaInteger", "LuaUnsigned", "LuMem", "LMem", "IdxT", "Instruction", "TimeT", "Utfint", "BinOpr", "OpCode", "UnOpr"
                            ].contains(&t) || t.starts_with("ptr[")
                        )
                    };
                    if is_scalar(&parent_ty) && is_scalar(&child_ty) {
                        return CExpr::Cast {
                            target_ty: parent_ty,
                            expr: Box::new(sub_expr),
                        };
                    }
                }
                sub_expr
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            }
        }
        CXCursor_ParenExpr => {
            let children = get_children(cursor);
            if let Some(first) = children.first() {
                parse_expr(*first)
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            }
        }
        CXCursor_CStyleCastExpr => {
            let children = get_children(cursor);
            let target_ty = c_type_to_arandu(clang_getCursorType(cursor));
            if let Some(sub) = children.last() {
                let sub_ty = clang_getCursorType(*sub);
                let is_array = sub_ty.kind == CXType_ConstantArray || sub_ty.kind == CXType_IncompleteArray;
                let parsed_sub = parse_expr(*sub);
                let final_sub = if is_array && target_ty.starts_with("ptr[") {
                    CExpr::UnaryOp { op: "&".to_string(), operand: Box::new(parsed_sub), is_postfix: false }
                } else {
                    parsed_sub
                };
                CExpr::Cast {
                    target_ty,
                    expr: Box::new(final_sub),
                }
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            }
        }
        CXCursor_MemberRefExpr => {
            let children = get_children(cursor);
            let member = get_cursor_spelling(cursor);
            let tokens = get_cursor_tokens(cursor);
            let is_arrow = tokens.iter().any(|t| t == "->");
            if let Some(base) = children.first() {
                CExpr::MemberAccess {
                    base: Box::new(parse_expr(*base)),
                    member,
                    is_arrow,
                }
            } else {
                CExpr::Ident(member)
            }
        }
        CXCursor_ArraySubscriptExpr => {
            let children = get_children(cursor);
            if children.len() >= 2 {
                let base = parse_expr(children[0]);
                let index = parse_expr(children[1]);
                CExpr::Index {
                    base: Box::new(base),
                    index: Box::new(index),
                }
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            }
        }
        CXCursor_ConditionalOperator => {
            let children = get_children(cursor);
            if children.len() >= 3 {
                let cond = parse_expr(children[0]);
                let then_expr = parse_expr(children[1]);
                let else_expr = parse_expr(children[2]);
                CExpr::Ternary {
                    cond: Box::new(cond),
                    then_expr: Box::new(then_expr),
                    else_expr: Box::new(else_expr),
                }
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            }
        }
        CXCursor_CallExpr => {
            let children = get_children(cursor);
            if !children.is_empty() {
                let callee = parse_expr(children[0]);
                let decl = clang_getCursorReferenced(cursor);
                let num_params = if clang_Cursor_isNull(decl) == 0 {
                    clang_Cursor_getNumArguments(decl) as usize
                } else {
                    0
                };

                let mut args = Vec::new();
                for (i, &child) in children[1..].iter().enumerate() {
                    let mut arg_expr = parse_expr(child);
                    if i < num_params {
                        let param_cursor = clang_Cursor_getArgument(decl, i as std::os::raw::c_uint);
                        let expected_ty = c_type_to_arandu(clang_getCursorType(param_cursor));
                        let actual_ty = c_type_to_arandu(clang_getCursorType(child));

                        if !expected_ty.is_empty() && !actual_ty.is_empty() && expected_ty != actual_ty && !expected_ty.contains("/*") {
                            if expected_ty.starts_with("ptr[") {
                                if actual_ty.starts_with("ptr[") || actual_ty.starts_with('[') || actual_ty == "usize" || actual_ty == "u64" || actual_ty == "int" || actual_ty.contains("func(") {
                                    arg_expr = CExpr::Cast {
                                        target_ty: expected_ty,
                                        expr: Box::new(arg_expr),
                                    };
                                } else if let CExpr::IntLiteral(0) = arg_expr {
                                    arg_expr = CExpr::Ident("nil".to_string());
                                }
                            } else if ["usize", "u64", "u32", "uint", "int", "i64", "i32", "f64"].contains(&expected_ty.as_str()) {
                                if !actual_ty.starts_with("ptr[") {
                                    arg_expr = CExpr::Cast {
                                        target_ty: expected_ty,
                                        expr: Box::new(arg_expr),
                                    };
                                }
                            }
                        }
                    }
                    args.push(arg_expr);
                }

                CExpr::Call {
                    callee: Box::new(callee),
                    args,
                }
            } else {
                let name = get_cursor_spelling(cursor);
                CExpr::Call {
                    callee: Box::new(CExpr::Ident(name)),
                    args: Vec::new(),
                }
            }
        }
        CXCursor_UnaryOperator => {
            let children = get_children(cursor);
            if let Some(operand) = children.first() {
                let tu = clang_Cursor_getTranslationUnit(cursor);
                let cur_range = clang_getCursorExtent(cursor);
                let op_range = clang_getCursorExtent(*operand);

                // Em C, operadores unários postfix são ESTRITAMENTE "++" e "--".
                let post_range = clang_getRange(clang_getRangeEnd(op_range), clang_getRangeEnd(cur_range));
                let post_tokens = crate::types::get_range_tokens(tu, post_range);
                if let Some(p_op) = post_tokens.into_iter().find(|t| t == "++" || t == "--") {
                    return CExpr::UnaryOp {
                        op: p_op,
                        operand: Box::new(parse_expr(*operand)),
                        is_postfix: true,
                    };
                }

                // Caso contrário, é prefix ("++", "--", "~", "!", "&", "*", "-", "+")
                let pre_range = clang_getRange(clang_getRangeStart(cur_range), clang_getRangeStart(op_range));
                let pre_tokens = crate::types::get_range_tokens(tu, pre_range);
                const PREFIX_OPS: &[&str] = &["++", "--", "~", "!", "&", "*", "-", "+"];
                let op = pre_tokens
                    .into_iter()
                    .find(|t| PREFIX_OPS.contains(&t.as_str()))
                    .or_else(|| {
                        let all = crate::types::get_cursor_tokens(cursor);
                        all.into_iter().find(|t| PREFIX_OPS.contains(&t.as_str()))
                    })
                    .unwrap_or_else(|| {
                        let mut op_cursor = *operand;
                        while clang_getCursorKind(op_cursor) == CXCursor_ParenExpr || clang_getCursorKind(op_cursor) == CXCursor_UnexposedExpr {
                            let ch = get_children(op_cursor);
                            if ch.is_empty() { break; }
                            op_cursor = ch[0];
                        }
                        let op_kind = clang_getCursorKind(op_cursor);
                        if op_kind == CXCursor_ConditionalOperator || op_kind == CXCursor_BinaryOperator || op_kind == CXCursor_CallExpr {
                            "!".to_string()
                        } else {
                            let ty_str = c_type_to_arandu(clang_getCursorType(*operand));
                            if ty_str == "usize" || ty_str == "u64" || ty_str == "uint" {
                                "--".to_string()
                            } else {
                                "-".to_string()
                            }
                        }
                    });

                if op == "!" {
                    let operand_ty = c_type_to_arandu(clang_getCursorType(*operand));
                    if operand_ty.starts_with("ptr[") {
                        return CExpr::BinaryOp {
                            op: "==".to_string(),
                            left: Box::new(parse_expr(*operand)),
                            right: Box::new(CExpr::Ident("nil".to_string())),
                        };
                    }
                }

                if op == "&" {
                    let target_ty = c_type_to_arandu(clang_getCursorType(cursor));
                    if target_ty.starts_with("ptr[") {
                        return CExpr::Cast {
                            target_ty,
                            expr: Box::new(CExpr::UnaryOp {
                                op,
                                operand: Box::new(parse_expr(*operand)),
                                is_postfix: false,
                            }),
                        };
                    }
                }

                CExpr::UnaryOp {
                    op,
                    operand: Box::new(parse_expr(*operand)),
                    is_postfix: false,
                }
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            }
        }
        CXCursor_BinaryOperator | CXCursor_CompoundAssignOperator => {
            let children = get_children(cursor);
            if children.len() >= 2 {
                let mut left = parse_expr(children[0]);
                let mut right = parse_expr(children[1]);

                let left_ty = clang_getCursorType(children[0]);
                let right_ty = clang_getCursorType(children[1]);
                let left_aru_ty = c_type_to_arandu(left_ty);
                let right_aru_ty = c_type_to_arandu(right_ty);
                let expr_aru_ty = c_type_to_arandu(clang_getCursorType(cursor));

                let spelling = get_cursor_spelling(cursor);
                const OPS: &[&str] = &[
                    "==", "!=", "<=", ">=", "&&", "||", "<<=", ">>=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=",
                    "<<", ">>", "+", "-", "*", "/", "%", "<", ">", "&", "|", "^", ",", "="
                ];

                let found_op = if OPS.contains(&spelling.as_str()) {
                    spelling
                } else {
                    let tu = clang_Cursor_getTranslationUnit(cursor);
                    let c0_end = clang_getRangeEnd(clang_getCursorExtent(children[0]));
                    let c1_start = clang_getRangeStart(clang_getCursorExtent(children[1]));
                    let between_range = clang_getRange(c0_end, c1_start);
                    let between_tokens = crate::types::get_range_tokens(tu, between_range);
                    between_tokens
                        .into_iter()
                        .find(|t| OPS.contains(&t.as_str()))
                        .unwrap_or_else(|| "=".to_string())
                };

                let left_can_ty = clang_getCanonicalType(left_ty);
                let right_can_ty = clang_getCanonicalType(right_ty);
                let left_is_ptr = left_aru_ty.starts_with("ptr[") || left_ty.kind == CXType_Pointer || left_can_ty.kind == CXType_Pointer;
                let right_is_ptr = right_aru_ty.starts_with("ptr[") || right_ty.kind == CXType_Pointer || right_can_ty.kind == CXType_Pointer;

                fn is_zero_val(e: &CExpr) -> bool {
                    match e {
                        CExpr::IntLiteral(0) => true,
                        CExpr::Cast { expr, .. } | CExpr::Paren(expr) => is_zero_val(expr),
                        _ => false,
                    }
                }

                // Normalizações e coerções de tipos para conformidade estrita com Arandu:
                if found_op == "==" || found_op == "!=" {
                    if left_is_ptr && is_zero_val(&right) {
                        right = CExpr::Ident("nil".to_string());
                    } else if right_is_ptr && is_zero_val(&left) {
                        left = CExpr::Ident("nil".to_string());
                    } else if left_aru_ty != right_aru_ty && !left_aru_ty.is_empty() && !right_aru_ty.is_empty() {
                        if (left_aru_ty == "int" && right_aru_ty == "uint") || (left_aru_ty == "uint" && right_aru_ty == "int") {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if (left_aru_ty == "u64" && right_aru_ty == "usize") || (left_aru_ty == "usize" && right_aru_ty == "u64") {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if left_aru_ty == "usize" || left_aru_ty == "u64" || left_aru_ty == "int" || left_aru_ty == "i64" {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if right_aru_ty == "usize" || right_aru_ty == "u64" || right_aru_ty == "int" || right_aru_ty == "i64" {
                            left = CExpr::Cast { target_ty: right_aru_ty.clone(), expr: Box::new(left) };
                        }
                    }
                } else if ["<", "<=", ">", ">="].contains(&found_op.as_str()) {
                    if left_is_ptr && right_is_ptr {
                        left = CExpr::Cast { target_ty: "usize".to_string(), expr: Box::new(left) };
                        right = CExpr::Cast { target_ty: "usize".to_string(), expr: Box::new(right) };
                    } else if left_is_ptr && is_zero_val(&right) {
                        right = CExpr::Ident("nil".to_string());
                    } else if right_is_ptr && is_zero_val(&left) {
                        left = CExpr::Ident("nil".to_string());
                    } else if left_aru_ty != right_aru_ty && !left_aru_ty.is_empty() && !right_aru_ty.is_empty() {
                        if (left_aru_ty == "int" && right_aru_ty == "uint") || (left_aru_ty == "uint" && right_aru_ty == "int") {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if (left_aru_ty == "u64" && right_aru_ty == "usize") || (left_aru_ty == "usize" && right_aru_ty == "u64") {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if right_aru_ty == "usize" || right_aru_ty == "u64" || right_aru_ty == "i64" || right_aru_ty == "int" {
                            left = CExpr::Cast { target_ty: right_aru_ty.clone(), expr: Box::new(left) };
                        } else if left_aru_ty == "usize" || left_aru_ty == "u64" || left_aru_ty == "i64" || left_aru_ty == "int" {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if right_aru_ty == "u32" || right_aru_ty == "uint" {
                            left = CExpr::Cast { target_ty: right_aru_ty.clone(), expr: Box::new(left) };
                        } else if left_aru_ty == "u32" || left_aru_ty == "uint" {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        }
                    }
                } else if ["+", "-", "*", "/", "%", "&", "|", "^"].contains(&found_op.as_str()) {
                    if left_aru_ty != right_aru_ty && !left_aru_ty.is_empty() && !right_aru_ty.is_empty() {
                        if (left_aru_ty == "int" && right_aru_ty == "uint") || (left_aru_ty == "uint" && right_aru_ty == "int") {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if (left_aru_ty == "u64" && right_aru_ty == "usize") || (left_aru_ty == "usize" && right_aru_ty == "u64") {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if ["usize", "u64", "i64", "int"].contains(&left_aru_ty.as_str()) && ["u8", "i8", "u16", "i16", "LuByte"].contains(&right_aru_ty.as_str()) {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        } else if ["usize", "u64", "i64", "int"].contains(&right_aru_ty.as_str()) && ["u8", "i8", "u16", "i16", "LuByte"].contains(&left_aru_ty.as_str()) {
                            left = CExpr::Cast { target_ty: right_aru_ty.clone(), expr: Box::new(left) };
                        }
                    }
                } else if found_op.ends_with('=') && found_op != "==" && found_op != "!=" && found_op != "<=" && found_op != ">=" {
                    if left_is_ptr && (found_op == "+=" || found_op == "-=") {
                        let math_op = if found_op == "+=" { "+" } else { "-" };
                        return CExpr::BinaryOp {
                            op: "=".to_string(),
                            left: Box::new(left.clone()),
                            right: Box::new(CExpr::BinaryOp {
                                op: math_op.to_string(),
                                left: Box::new(left),
                                right: Box::new(CExpr::Cast { target_ty: "usize".to_string(), expr: Box::new(right) }),
                            }),
                        };
                    } else if found_op == "=" {
                        if left_is_ptr && matches!(right, CExpr::IntLiteral(0)) {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(CExpr::Ident("nil".to_string())) };
                        } else if left_aru_ty != right_aru_ty && !left_aru_ty.is_empty() && !right_aru_ty.is_empty() {
                            right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                        }
                    } else if left_aru_ty != right_aru_ty && !left_aru_ty.is_empty() && !right_aru_ty.is_empty() {
                        right = CExpr::Cast { target_ty: left_aru_ty.clone(), expr: Box::new(right) };
                    }
                } else if ["+", "-", "*", "/", "%", "&", "|", "^"].contains(&found_op.as_str()) {
                    if left_is_ptr && (found_op == "+" || found_op == "-") {
                        if found_op == "-" && right_is_ptr {
                            return CExpr::BinaryOp {
                                op: "-".to_string(),
                                left: Box::new(CExpr::Cast { target_ty: "isize".to_string(), expr: Box::new(left) }),
                                right: Box::new(CExpr::Cast { target_ty: "isize".to_string(), expr: Box::new(right) }),
                            };
                        }
                        // Aritmética de ponteiro: ptr + int ou ptr - int (nunca converte o inteiro para o tipo do ponteiro)
                    } else if !expr_aru_ty.is_empty() && !expr_aru_ty.contains("/*") {
                        if left_aru_ty != expr_aru_ty && !left_aru_ty.is_empty() && !left_aru_ty.contains("/*") {
                            left = CExpr::Cast { target_ty: expr_aru_ty.clone(), expr: Box::new(left) };
                        }
                        if right_aru_ty != expr_aru_ty && !right_aru_ty.is_empty() && !right_aru_ty.contains("/*") {
                            right = CExpr::Cast { target_ty: expr_aru_ty.clone(), expr: Box::new(right) };
                        }
                    }
                }

                CExpr::BinaryOp {
                    op: found_op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            }
        }
        CXCursor_InitListExpr => {
            let ty = clang_getCursorType(cursor);
            let canon_ty = clang_getCanonicalType(ty);
            let children = get_children(cursor);
            if ty.kind == CXType_Record || canon_ty.kind == CXType_Record {
                let ty_name = c_type_to_arandu(ty);
                let mut fields_info: Vec<(String, CXType)> = Vec::new();
                extern "C" fn field_visitor(
                    f_cur: CXCursor,
                    data: *mut c_void,
                ) -> CXVisitorResult {
                    unsafe {
                        let fields = &mut *(data as *mut Vec<(String, CXType)>);
                        let name = get_cursor_spelling(f_cur);
                        let ty = clang_getCursorType(f_cur);
                        fields.push((name, ty));
                        CXVisit_Continue
                    }
                }
                let target_ty = if ty.kind == CXType_Record { ty } else { canon_ty };
                clang_Type_visitFields(target_ty, field_visitor, &mut fields_info as *mut _ as *mut c_void);
                let mut inits = Vec::new();
                for (i, child) in children.iter().enumerate() {
                    let val_expr = parse_expr(*child);
                    let mut val_str = crate::codegen::generate_expr(&val_expr);
                    if let Some((f_name, f_ty)) = fields_info.get(i) {
                        let f_aru_ty = c_type_to_arandu(*f_ty);
                        if (f_aru_ty.starts_with("ptr[func(") || f_aru_ty.contains("Func") || f_aru_ty.contains("func"))
                            && val_str != "nil" && val_str != "0" && !val_str.starts_with('&')
                        {
                            val_str = format!("(&{val_str} as {f_aru_ty})");
                        } else if f_aru_ty.starts_with("ptr[") && val_str == "0" {
                            val_str = "nil".to_string();
                        } else if f_aru_ty == "ptr[i8]" && (val_str.starts_with("c\"") || val_str.starts_with('"')) {
                            val_str = format!("({val_str} as ptr[i8])");
                        } else if f_aru_ty == "ptr[u8]" && (val_str.starts_with("c\"") || val_str.starts_with('"')) {
                            val_str = format!("({val_str} as ptr[u8])");
                        }
                        let clean_f = crate::codegen::sanitize_ident(f_name);
                        inits.push(format!("{clean_f}: {val_str}"));
                    } else {
                        inits.push(val_str);
                    }
                }
                CExpr::Raw(format!("{ty_name} {{ {} }}", inits.join(", ")))
            } else {
                let elem_ty = clang_getArrayElementType(if ty.kind == CXType_ConstantArray { ty } else { canon_ty });
                let elem_aru_ty = c_type_to_arandu(elem_ty);
                let is_fn_ptr = elem_aru_ty.starts_with("ptr[func(") || elem_aru_ty.contains("Func") || elem_aru_ty.contains("func");
                let items: Vec<String> = children
                    .iter()
                    .map(|&c| {
                        let parsed = parse_expr(c);
                        let mut s = crate::codegen::generate_expr(&parsed);
                        if is_fn_ptr {
                            if s == "nil" || s == "0" || s == "(0)" {
                                s = format!("(nil as {elem_aru_ty})");
                            } else {
                                let bare = s.trim_start_matches('&').trim_start_matches('(').trim_end_matches(')');
                                s = format!("((&{bare}) as {elem_aru_ty})");
                            }
                        } else if elem_aru_ty == "LuByte" || elem_aru_ty == "u8" || elem_aru_ty == "i8" || elem_aru_ty == "LsByte" {
                            s = format!("(({s}) as {elem_aru_ty})");
                        }
                        s
                    })
                    .collect();
                CExpr::Raw(format!("[{}]", items.join(", ")))
            }
        }
        _ => CExpr::Raw(get_cursor_token_string(cursor)),
    }
}

pub unsafe fn parse_stmt(cursor: CXCursor) -> Vec<CStmt> {
    match cursor.kind {
        CXCursor_CompoundStmt => {
            let children = get_children(cursor);
            let mut stmts = Vec::new();
            for child in children {
                stmts.extend(parse_stmt(child));
            }
            stmts
        }
        CXCursor_DeclStmt => {
            let children = get_children(cursor);
            let mut stmts = Vec::new();
            for child in children {
                if child.kind == CXCursor_VarDecl {
                    let name = get_cursor_spelling(child);
                    let ty = c_type_to_arandu(clang_getCursorType(child));
                    let init_cursor = clang_Cursor_getVarDeclInitializer(child);
                    let init = if clang_Cursor_isNull(init_cursor) == 0 && clang_isInvalid(init_cursor.kind) == 0 {
                        Some(parse_expr(init_cursor))
                    } else {
                        None
                    };
                    stmts.push(CStmt::VarDecl {
                        name,
                        ty,
                        init,
                        is_mut: true,
                    });
                }
            }
            stmts
        }
        CXCursor_ReturnStmt => {
            let children = get_children(cursor);
            let expr = children.first().map(|&c| parse_expr(c));
            vec![CStmt::Return(expr)]
        }
        CXCursor_IfStmt => {
            let children = get_children(cursor);
            if !children.is_empty() {
                let cond = parse_expr(children[0]);
                let then_stmts = if children.len() > 1 {
                    parse_stmt(children[1])
                } else {
                    Vec::new()
                };
                let else_stmts = if children.len() > 2 {
                    Some(parse_stmt(children[2]))
                } else {
                    None
                };
                vec![CStmt::If {
                    cond,
                    then_block: then_stmts,
                    else_block: else_stmts,
                }]
            } else {
                Vec::new()
            }
        }
        CXCursor_WhileStmt => {
            let children = get_children(cursor);
            if children.len() >= 2 {
                let cond = parse_expr(children[0]);
                let body = parse_stmt(children[1]);
                vec![CStmt::While { cond, body }]
            } else {
                Vec::new()
            }
        }
        CXCursor_DoStmt => {
            let children = get_children(cursor);
            if children.len() >= 2 {
                let body = parse_stmt(children[0]);
                let cond = parse_expr(children[1]);
                vec![CStmt::DoWhile { body, cond }]
            } else {
                Vec::new()
            }
        }
        CXCursor_ForStmt => {
            let children = get_children(cursor);
            // No Clang ForStmt: filhos podem ser [init, cond, inc, body]
            let body = children.last().map(|&c| parse_stmt(c)).unwrap_or_default();
            let mut init = None;
            let mut cond = None;
            let mut step = None;

            if children.len() == 4 {
                let init_stmts = parse_stmt(children[0]);
                init = init_stmts.into_iter().next().map(Box::new);
                cond = Some(parse_expr(children[1]));
                step = Some(parse_expr(children[2]));
            } else if children.len() > 1 {
                let tokens = get_cursor_tokens(cursor);
                let semi1 = tokens.iter().position(|t| t == ";");
                let semi2 = semi1.and_then(|idx1| tokens[idx1 + 1..].iter().position(|t| t == ";").map(|p| p + idx1 + 1));
                let lparen = tokens.iter().position(|t| t == "(");
                let rparen = semi2.and_then(|idx2| tokens[idx2 + 1..].iter().position(|t| t == ")").map(|p| p + idx2 + 1));

                let has_init = match (lparen, semi1) {
                    (Some(lp), Some(s1)) => s1 > lp + 1,
                    _ => false,
                };
                let has_cond = match (semi1, semi2) {
                    (Some(s1), Some(s2)) => s2 > s1 + 1,
                    _ => false,
                };
                let has_step = match (semi2, rparen) {
                    (Some(s2), Some(rp)) => rp > s2 + 1,
                    _ => false,
                };

                let mut child_idx = 0;
                let num_header_children = children.len() - 1;

                if has_init && child_idx < num_header_children {
                    let init_stmts = parse_stmt(children[child_idx]);
                    init = init_stmts.into_iter().next().map(Box::new);
                    child_idx += 1;
                }
                if has_cond && child_idx < num_header_children {
                    cond = Some(parse_expr(children[child_idx]));
                    child_idx += 1;
                }
                if has_step && child_idx < num_header_children {
                    step = Some(parse_expr(children[child_idx]));
                }
            }

            vec![CStmt::For {
                init,
                cond,
                step,
                body,
            }]
        }
        CXCursor_NullStmt => Vec::new(),
        CXCursor_BreakStmt => vec![CStmt::Break],
        CXCursor_ContinueStmt => vec![CStmt::Continue],
        CXCursor_IndirectGotoStmt => {
            let children = get_children(cursor);
            let target = if let Some(first) = children.first() {
                parse_expr(*first)
            } else {
                CExpr::Raw(get_cursor_token_string(cursor))
            };
            vec![CStmt::Expr(CExpr::Raw(format!("/* computed goto: {} */", crate::codegen::generate_expr(&target))))]
        }
        CXCursor_GotoStmt => {
            let children = get_children(cursor);
            let label = if let Some(first) = children.first() {
                get_cursor_spelling(*first)
            } else {
                let tokens = get_cursor_tokens(cursor);
                tokens.get(1).cloned().unwrap_or_default()
            };
            vec![CStmt::Goto(label)]
        }
        CXCursor_LabelStmt => {
            let name = get_cursor_spelling(cursor);
            let children = get_children(cursor);
            let mut stmts = vec![CStmt::Label(name)];
            for child in children {
                stmts.extend(parse_stmt(child));
            }
            stmts
        }
        CXCursor_SwitchStmt => {
            let children = get_children(cursor);
            if !children.is_empty() {
                let expr = parse_expr(children[0]);
                let mut arms = Vec::new();
                if children.len() > 1 {
                    let switch_body = children[1];

                    enum SwitchControl {
                        Label(String),
                        Stmt(CStmt),
                        Break,
                    }

                    unsafe fn collect_controls(cur: CXCursor, out: &mut Vec<SwitchControl>) {
                        match cur.kind {
                            CXCursor_CaseStmt => {
                                let c_children = get_children(cur);
                                if let Some(first) = c_children.first() {
                                    let eval = clang_Cursor_Evaluate(*first);
                                    let pat_str = if !eval.is_null() {
                                        let kind = clang_EvalResult_getKind(eval);
                                        if kind == CXEval_Int {
                                            let val = clang_EvalResult_getAsInt(eval);
                                            clang_EvalResult_dispose(eval);
                                            val.to_string()
                                        } else {
                                            clang_EvalResult_dispose(eval);
                                            crate::codegen::generate_expr(&parse_expr(*first))
                                        }
                                    } else {
                                        crate::codegen::generate_expr(&parse_expr(*first))
                                    };
                                    out.push(SwitchControl::Label(pat_str));
                                }
                                for sub in &c_children[1..] {
                                    collect_controls(*sub, out);
                                }
                            }
                            CXCursor_DefaultStmt => {
                                out.push(SwitchControl::Label("_".to_string()));
                                for sub in get_children(cur) {
                                    collect_controls(sub, out);
                                }
                            }
                            CXCursor_BreakStmt => {
                                out.push(SwitchControl::Break);
                            }
                            CXCursor_CompoundStmt => {
                                for sub in get_children(cur) {
                                    collect_controls(sub, out);
                                }
                            }
                            _ => {
                                let stmts = parse_stmt(cur);
                                for s in stmts {
                                    if matches!(s, CStmt::Break) {
                                        out.push(SwitchControl::Break);
                                    } else {
                                        out.push(SwitchControl::Stmt(s));
                                    }
                                }
                            }
                        }
                    }

                    let mut controls = Vec::new();
                    unsafe {
                        collect_controls(switch_body, &mut controls);
                    }

                    #[derive(Clone, Debug)]
                    struct CaseBlock {
                        patterns: Vec<String>,
                        stmts: Vec<CStmt>,
                        terminated: bool,
                    }

                    let mut blocks: Vec<CaseBlock> = Vec::new();
                    let mut current_patterns = Vec::new();
                    let mut current_stmts = Vec::new();
                    let mut current_terminated = false;

                    for ctrl in controls {
                        match ctrl {
                            SwitchControl::Label(pat) => {
                                if !current_stmts.is_empty() {
                                    blocks.push(CaseBlock {
                                        patterns: std::mem::take(&mut current_patterns),
                                        stmts: std::mem::take(&mut current_stmts),
                                        terminated: current_terminated,
                                    });
                                    current_terminated = false;
                                }
                                current_patterns.push(pat);
                            }
                            SwitchControl::Stmt(s) => {
                                if matches!(s, CStmt::Return(_) | CStmt::Goto(_)) {
                                    current_terminated = true;
                                }
                                current_stmts.push(s);
                            }
                            SwitchControl::Break => {
                                current_terminated = true;
                            }
                        }
                    }

                    if !current_patterns.is_empty() || !current_stmts.is_empty() {
                        blocks.push(CaseBlock {
                            patterns: current_patterns,
                            stmts: current_stmts,
                            terminated: current_terminated,
                        });
                    }

                    for i in 0..blocks.len() {
                        let pat_str = blocks[i].patterns.join(" | ");
                        let mut body = blocks[i].stmts.clone();

                        // Se o case não terminou com break/return/goto, sintetiza fallthrough
                        // anexando os statements dos blocos seguintes até o primeiro que terminar.
                        if !blocks[i].terminated {
                            for next_block in &blocks[i + 1..] {
                                body.extend(next_block.stmts.clone());
                                if next_block.terminated {
                                    break;
                                }
                            }
                        }

                        let final_pat = if pat_str.is_empty() {
                            "_".to_string()
                        } else {
                            pat_str
                        };

                        arms.push(CMatchArm {
                            pattern: final_pat,
                            body,
                        });
                    }
                }

                if !arms.iter().any(|a| a.pattern == "_") {
                    arms.push(CMatchArm {
                        pattern: "_".to_string(),
                        body: Vec::new(),
                    });
                }

                vec![CStmt::Switch { expr, arms }]
            } else {
                Vec::new()
            }
        }
        CXCursor_BinaryOperator | CXCursor_CompoundAssignOperator | _ => {
            let expr = parse_expr(cursor);
            let mut stmts = Vec::new();

            fn flatten_expr_to_stmts(e: CExpr, out: &mut Vec<CStmt>) {
                let mut curr = e;
                while let CExpr::Paren(inner) = curr {
                    curr = *inner;
                }

                if let CExpr::Ternary { cond, then_expr, else_expr } = curr {
                    let mut then_stmts = Vec::new();
                    flatten_expr_to_stmts(*then_expr, &mut then_stmts);
                    let mut else_stmts = Vec::new();
                    flatten_expr_to_stmts(*else_expr, &mut else_stmts);

                    let is_noop_list = |stmts: &[CStmt]| -> bool {
                        stmts.is_empty() || (stmts.len() == 1 && match &stmts[0] {
                            CStmt::Expr(e) => {
                                let mut curr = e;
                                while let CExpr::Paren(inner) = curr {
                                    curr = inner;
                                }
                                matches!(curr, CExpr::IntLiteral(0))
                                    || matches!(curr, CExpr::Ident(id) if id == "NULL" || id == "nil")
                            }
                            _ => false,
                        })
                    };

                    let else_opt = if is_noop_list(&else_stmts) {
                        None
                    } else {
                        Some(else_stmts)
                    };

                    out.push(CStmt::If {
                        cond: *cond,
                        then_block: then_stmts,
                        else_block: else_opt,
                    });
                    return;
                }

                if let CExpr::BinaryOp { op, left, right } = curr {
                    if op == "," {
                        flatten_expr_to_stmts(*left, out);
                        flatten_expr_to_stmts(*right, out);
                        return;
                    }
                    if [
                        "=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="
                    ].contains(&op.as_str()) {
                        fn unroll_nested_assign(target: CExpr, op: String, value: CExpr, out: &mut Vec<CStmt>) {
                            let mut unwrapped_val = value;
                            while let CExpr::Paren(inner) = unwrapped_val {
                                unwrapped_val = *inner;
                            }
                            if let CExpr::BinaryOp { op: ref r_op, left: ref r_left, right: ref r_right } = unwrapped_val {
                                if r_op == "=" {
                                    unroll_nested_assign((**r_left).clone(), r_op.clone(), (**r_right).clone(), out);
                                    out.push(CStmt::Assign {
                                        target,
                                        op,
                                        value: (**r_left).clone(),
                                    });
                                    return;
                                }
                            }
                            out.push(CStmt::Assign {
                                target,
                                op,
                                value: unwrapped_val,
                            });
                        }
                        unroll_nested_assign(*left, op, *right, out);
                        return;
                    }
                    out.push(CStmt::Expr(CExpr::BinaryOp { op, left, right }));
                    return;
                }
                out.push(CStmt::Expr(curr));
            }

            flatten_expr_to_stmts(expr, &mut stmts);
            stmts
        }
    }
}

pub fn rename_vars_simultaneous_in_expr(expr: &mut CExpr, map: &std::collections::HashMap<String, String>) {
    match expr {
        CExpr::Ident(name) => {
            if let Some(to) = map.get(name) {
                *name = to.clone();
            }
        }
        CExpr::BinaryOp { left, right, .. } => {
            rename_vars_simultaneous_in_expr(left, map);
            rename_vars_simultaneous_in_expr(right, map);
        }
        CExpr::UnaryOp { operand, .. } => {
            rename_vars_simultaneous_in_expr(operand, map);
        }
        CExpr::MemberAccess { base, .. } => {
            rename_vars_simultaneous_in_expr(base, map);
        }
        CExpr::Index { base, index } => {
            rename_vars_simultaneous_in_expr(base, map);
            rename_vars_simultaneous_in_expr(index, map);
        }
        CExpr::Call { callee, args } => {
            rename_vars_simultaneous_in_expr(callee, map);
            for a in args {
                rename_vars_simultaneous_in_expr(a, map);
            }
        }
        CExpr::Cast { expr, .. } => rename_vars_simultaneous_in_expr(expr, map),
        CExpr::Paren(inner) => rename_vars_simultaneous_in_expr(inner, map),
        CExpr::Ternary { cond, then_expr, else_expr } => {
            rename_vars_simultaneous_in_expr(cond, map);
            rename_vars_simultaneous_in_expr(then_expr, map);
            rename_vars_simultaneous_in_expr(else_expr, map);
        }
        _ => {}
    }
}

pub fn rename_vars_simultaneous_in_stmts(stmts: &mut [CStmt], map: &std::collections::HashMap<String, String>) {
    for stmt in stmts {
        match stmt {
            CStmt::VarDecl { init, .. } => {
                if let Some(init_expr) = init {
                    rename_vars_simultaneous_in_expr(init_expr, map);
                }
            }
            CStmt::Assign { target, value, .. } => {
                rename_vars_simultaneous_in_expr(target, map);
                rename_vars_simultaneous_in_expr(value, map);
            }
            CStmt::Expr(e) | CStmt::Return(Some(e)) => rename_vars_simultaneous_in_expr(e, map),
            CStmt::If { cond, then_block, else_block } => {
                rename_vars_simultaneous_in_expr(cond, map);
                rename_vars_simultaneous_in_stmts(then_block, map);
                if let Some(eb) = else_block {
                    rename_vars_simultaneous_in_stmts(eb, map);
                }
            }
            CStmt::While { cond, body } | CStmt::DoWhile { cond, body } => {
                rename_vars_simultaneous_in_expr(cond, map);
                rename_vars_simultaneous_in_stmts(body, map);
            }
            CStmt::For { init, cond, step, body } => {
                if let Some(i) = init {
                    rename_vars_simultaneous_in_stmts(std::slice::from_mut(i), map);
                }
                if let Some(c) = cond {
                    rename_vars_simultaneous_in_expr(c, map);
                }
                if let Some(s) = step {
                    rename_vars_simultaneous_in_expr(s, map);
                }
                rename_vars_simultaneous_in_stmts(body, map);
            }
            CStmt::Switch { expr, arms } => {
                rename_vars_simultaneous_in_expr(expr, map);
                for arm in arms {
                    rename_vars_simultaneous_in_stmts(&mut arm.body, map);
                }
            }
            CStmt::Compound(inner) => rename_vars_simultaneous_in_stmts(inner, map),
            _ => {}
        }
    }
}

pub fn rename_var_in_expr(expr: &mut CExpr, from: &str, to: &str) {
    match expr {
        CExpr::Ident(name) => {
            if name == from {
                *name = to.to_string();
            }
        }
        CExpr::BinaryOp { left, right, .. } => {
            rename_var_in_expr(left, from, to);
            rename_var_in_expr(right, from, to);
        }
        CExpr::UnaryOp { operand, .. } => {
            rename_var_in_expr(operand, from, to);
        }
        CExpr::MemberAccess { base, .. } => {
            rename_var_in_expr(base, from, to);
        }
        CExpr::Index { base, index } => {
            rename_var_in_expr(base, from, to);
            rename_var_in_expr(index, from, to);
        }
        CExpr::Call { callee, args } => {
            rename_var_in_expr(callee, from, to);
            for a in args {
                rename_var_in_expr(a, from, to);
            }
        }
        CExpr::Cast { expr, .. } => rename_var_in_expr(expr, from, to),
        CExpr::Paren(inner) => rename_var_in_expr(inner, from, to),
        CExpr::Ternary { cond, then_expr, else_expr } => {
            rename_var_in_expr(cond, from, to);
            rename_var_in_expr(then_expr, from, to);
            rename_var_in_expr(else_expr, from, to);
        }
        _ => {}
    }
}

pub fn rename_var_in_stmts(stmts: &mut [CStmt], from: &str, to: &str) {
    for stmt in stmts {
        match stmt {
            CStmt::VarDecl { init, .. } => {
                if let Some(init_expr) = init {
                    rename_var_in_expr(init_expr, from, to);
                }
            }
            CStmt::Assign { target, value, .. } => {
                rename_var_in_expr(target, from, to);
                rename_var_in_expr(value, from, to);
            }
            CStmt::Expr(e) | CStmt::Return(Some(e)) => rename_var_in_expr(e, from, to),
            CStmt::If { cond, then_block, else_block } => {
                rename_var_in_expr(cond, from, to);
                rename_var_in_stmts(then_block, from, to);
                if let Some(eb) = else_block {
                    rename_var_in_stmts(eb, from, to);
                }
            }
            CStmt::While { cond, body } | CStmt::DoWhile { cond, body } => {
                rename_var_in_expr(cond, from, to);
                rename_var_in_stmts(body, from, to);
            }
            CStmt::For { init, cond, step, body } => {
                if let Some(i) = init {
                    rename_var_in_stmts(std::slice::from_mut(i), from, to);
                }
                if let Some(c) = cond {
                    rename_var_in_expr(c, from, to);
                }
                if let Some(s) = step {
                    rename_var_in_expr(s, from, to);
                }
                rename_var_in_stmts(body, from, to);
            }
            CStmt::Switch { expr, arms } => {
                rename_var_in_expr(expr, from, to);
                for arm in arms {
                    rename_var_in_stmts(&mut arm.body, from, to);
                }
            }
            CStmt::Compound(inner) => rename_var_in_stmts(inner, from, to),
            _ => {}
        }
    }
}

fn collect_mutated_vars(stmts: &[CStmt], vars: &mut std::collections::HashSet<String>) {
    fn scan_expr(e: &CExpr, vars: &mut std::collections::HashSet<String>) {
        match e {
            CExpr::UnaryOp { op, operand, .. } if op == "++" || op == "--" => {
                if let CExpr::Ident(name) = &**operand {
                    vars.insert(name.clone());
                }
                scan_expr(operand, vars);
            }
            CExpr::BinaryOp { op, left, right } => {
                if [
                    "=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="
                ].contains(&op.as_str()) {
                    if let CExpr::Ident(name) = &**left {
                        vars.insert(name.clone());
                    }
                }
                scan_expr(left, vars);
                scan_expr(right, vars);
            }
            CExpr::Ternary { cond, then_expr, else_expr } => {
                scan_expr(cond, vars);
                scan_expr(then_expr, vars);
                scan_expr(else_expr, vars);
            }
            CExpr::Call { callee, args } => {
                scan_expr(callee, vars);
                for arg in args {
                    scan_expr(arg, vars);
                }
            }
            CExpr::Paren(inner) => scan_expr(inner, vars),
            _ => {}
        }
    }

    for s in stmts {
        match s {
            CStmt::Assign { target, value, .. } => {
                if let CExpr::Ident(name) = target {
                    vars.insert(name.clone());
                }
                scan_expr(target, vars);
                scan_expr(value, vars);
            }
            CStmt::Expr(expr) => {
                scan_expr(expr, vars);
            }
            CStmt::If { cond, then_block, else_block } => {
                scan_expr(cond, vars);
                collect_mutated_vars(then_block, vars);
                if let Some(eb) = else_block {
                    collect_mutated_vars(eb, vars);
                }
            }
            CStmt::While { cond, body } => {
                scan_expr(cond, vars);
                collect_mutated_vars(body, vars);
            }
            CStmt::DoWhile { body, cond } => {
                collect_mutated_vars(body, vars);
                scan_expr(cond, vars);
            }
            CStmt::For { init, cond, step, body } => {
                if let Some(i) = init {
                    collect_mutated_vars(std::slice::from_ref(i), vars);
                }
                if let Some(c) = cond {
                    scan_expr(c, vars);
                }
                if let Some(st) = step {
                    scan_expr(st, vars);
                }
                collect_mutated_vars(body, vars);
            }
            CStmt::Switch { expr, arms } => {
                scan_expr(expr, vars);
                for arm in arms {
                    collect_mutated_vars(&arm.body, vars);
                }
            }
            CStmt::Compound(inner) => collect_mutated_vars(inner, vars),
            CStmt::Return(Some(expr)) => scan_expr(expr, vars),
            _ => {}
        }
    }
}

pub fn parse_c_file(path: &Path, options: &ParserOptions) -> Result<CProgram> {
    unsafe {
        let index = clang_createIndex(0, 0);
        if index.is_null() {
            return Err(anyhow!("Falha ao inicializar o índice do Clang"));
        }

        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow!("Caminho de arquivo inválido: {:?}", path))?;
        let c_path = CString::new(path_str)?;

        let mut c_args: Vec<CString> = Vec::new();
        for flag in &options.c_flags {
            c_args.push(CString::new(flag.as_str())?);
        }
        let c_args_ptrs: Vec<*const i8> = c_args.iter().map(|arg| arg.as_ptr()).collect();

        eprintln!("⏱️ [{:?}] Iniciando clang_parseTranslationUnit...", std::time::Instant::now());
        let tu = clang_parseTranslationUnit(
            index,
            c_path.as_ptr(),
            c_args_ptrs.as_ptr(),
            c_args_ptrs.len() as i32,
            std::ptr::null_mut(),
            0,
            CXTranslationUnit_None,
        );
        eprintln!("⏱️ Concluído clang_parseTranslationUnit.");

        if tu.is_null() {
            clang_disposeIndex(index);
            return Err(anyhow!("Não foi possível parsear o arquivo C: {}", path_str));
        }

        // Checar erros fatais de compilação C
        let num_diags = clang_getNumDiagnostics(tu);
        for i in 0..num_diags {
            let diag = clang_getDiagnostic(tu, i);
            let severity = clang_getDiagnosticSeverity(diag);
            if severity >= CXDiagnostic_Error {
                let diag_str = clang_formatDiagnostic(diag, clang_defaultDiagnosticDisplayOptions());
                let c_str = clang_getCString(diag_str);
                let msg = if !c_str.is_null() {
                    CStr::from_ptr(c_str).to_string_lossy().into_owned()
                } else {
                    "Erro desconhecido do Clang".to_string()
                };
                clang_disposeString(diag_str);
                clang_disposeDiagnostic(diag);
                clang_disposeTranslationUnit(tu);
                clang_disposeIndex(index);
                return Err(anyhow!("Erro no código C de entrada: {}", msg));
            }
            clang_disposeDiagnostic(diag);
        }

        let mut program = CProgram::default();
        let root_cursor = clang_getTranslationUnitCursor(tu);

        struct VisitorState<'a> {
            program: &'a mut CProgram,
            #[allow(dead_code)]
            main_file_only: bool,
            fn_indices: std::collections::HashMap<String, usize>,
            struct_indices: std::collections::HashMap<String, usize>,
            typedef_set: std::collections::HashSet<String>,
            global_set: std::collections::HashSet<String>,
        }

        let mut state = VisitorState {
            program: &mut program,
            main_file_only: options.main_file_only,
            fn_indices: std::collections::HashMap::new(),
            struct_indices: std::collections::HashMap::new(),
            typedef_set: std::collections::HashSet::new(),
            global_set: std::collections::HashSet::new(),
        };

        unsafe fn ensure_record_registered(
            ty: CXType,
            state: &mut VisitorState,
        ) {
            if ty.kind != CXType_Record {
                return;
            }

            let r_name = c_type_to_arandu(ty);
            if r_name.is_empty() || state.struct_indices.contains_key(&r_name) {
                return;
            }

            let decl = clang_getTypeDeclaration(ty);
            let is_union = decl.kind == CXCursor_UnionDecl;

            // Insere o placeholder ANTES de visitar os filhos para quebrar ciclos recursivos
            let struct_idx = state.program.structs.len();
            state.struct_indices.insert(r_name.clone(), struct_idx);
            state.program.structs.push(CStruct {
                name: r_name,
                fields: Vec::new(),
                is_union,
            });

            let mut fields: Vec<CField> = Vec::new();

            struct RegContext<'f, 's> {
                fields: &'f mut Vec<CField>,
                state: *mut VisitorState<'s>,
            }

            let mut reg_ctx = RegContext {
                fields: &mut fields,
                state: state as *mut _,
            };

            extern "C" fn reg_visitor(
                cur: CXCursor,
                _parent: CXCursor,
                client_data: *mut c_void,
            ) -> CXChildVisitResult {
                unsafe {
                    let ctx = &mut *(client_data as *mut RegContext<'_, '_>);
                    if cur.kind == CXCursor_FieldDecl {
                        let fname = get_cursor_spelling(cur);
                        let fty = clang_getCursorType(cur);
                        let ty_str = c_type_to_arandu(fty);
                        let is_bitfield = clang_Cursor_isBitField(cur) != 0;
                        let bit_width = if is_bitfield {
                            let bw = clang_getFieldDeclBitWidth(cur);
                            if bw >= 0 { Some(bw as u32) } else { None }
                        } else {
                            None
                        };
                        ctx.fields.push(CField {
                            name: fname,
                            ty: ty_str,
                            bit_width,
                        });
                        ensure_record_registered(fty, &mut *ctx.state);
                    } else if cur.kind == CXCursor_StructDecl || cur.kind == CXCursor_UnionDecl {
                        let sub_ty = clang_getCursorType(cur);
                        ensure_record_registered(sub_ty, &mut *ctx.state);
                    }
                    CXChildVisit_Continue
                }
            }

            clang_visitChildren(decl, reg_visitor, &mut reg_ctx as *mut _ as *mut c_void);

            state.program.structs[struct_idx].fields = fields;
        }

        extern "C" fn visit_top_level(
            cursor: CXCursor,
            _parent: CXCursor,
            client_data: *mut c_void,
        ) -> CXChildVisitResult {
            unsafe {
                let state = &mut *(client_data as *mut VisitorState);

                let loc = clang_getCursorLocation(cursor);
                if clang_Location_isInSystemHeader(loc) != 0 {
                    return CXChildVisit_Continue;
                }

                match cursor.kind {
                    CXCursor_StructDecl | CXCursor_UnionDecl => {
                        let is_union = cursor.kind == CXCursor_UnionDecl;
                        let raw_name = get_cursor_spelling(cursor);
                        if raw_name.is_empty() || raw_name.contains("unnamed") || raw_name.contains('(') {
                            return CXChildVisit_Continue;
                        }
                        let name = crate::types::to_pascal_case(&raw_name);
                            struct FieldContext<'a> {
                                fields: Vec<CField>,
                                state: &'a mut VisitorState<'a>,
                            }

                            let mut field_ctx = FieldContext {
                                fields: Vec::new(),
                                state,
                            };

                            extern "C" fn visit_fields(
                                field_cur: CXCursor,
                                _parent: CXCursor,
                                client_data: *mut c_void,
                            ) -> CXChildVisitResult {
                                unsafe {
                                    let ctx = &mut *(client_data as *mut FieldContext);
                                    if field_cur.kind == CXCursor_FieldDecl {
                                        let field_name = get_cursor_spelling(field_cur);
                                        let field_ty = clang_getCursorType(field_cur);
                                        let ty_str = c_type_to_arandu(field_ty);
                                        let is_bitfield = clang_Cursor_isBitField(field_cur) != 0;
                                        let bit_width = if is_bitfield {
                                            let bw = clang_getFieldDeclBitWidth(field_cur);
                                            if bw >= 0 { Some(bw as u32) } else { None }
                                        } else {
                                            None
                                        };
                                        ctx.fields.push(CField {
                                            name: field_name,
                                            ty: ty_str,
                                            bit_width,
                                        });
                                        ensure_record_registered(field_ty, ctx.state);
                                    } else if field_cur.kind == CXCursor_StructDecl || field_cur.kind == CXCursor_UnionDecl {
                                        let sub_ty = clang_getCursorType(field_cur);
                                        ensure_record_registered(sub_ty, ctx.state);
                                    }
                                    CXChildVisit_Continue
                                }
                            }

                            clang_visitChildren(
                                cursor,
                                visit_fields,
                                &mut field_ctx as *mut _ as *mut c_void,
                            );

                            let fields = field_ctx.fields;
                            let state = field_ctx.state;

                            if let Some(&idx) = state.struct_indices.get(&name) {
                                if !fields.is_empty() && state.program.structs[idx].fields.is_empty() {
                                    state.program.structs[idx].fields = fields;
                                }
                                state.program.structs[idx].is_union |= is_union;
                            } else {
                                state.struct_indices.insert(name.clone(), state.program.structs.len());
                                state.program.structs.push(CStruct { name, fields, is_union });
                            }
                    }
                    CXCursor_EnumDecl => {
                        let raw_name = get_cursor_spelling(cursor);
                        let name = if raw_name.is_empty()
                            || raw_name.contains("unnamed")
                            || raw_name.contains('(')
                        {
                            String::new()
                        } else {
                            crate::types::to_pascal_case(&raw_name)
                        };
                        let mut variants = Vec::new();

                        extern "C" fn visit_variants(
                            v_cur: CXCursor,
                            _parent: CXCursor,
                            client_data: *mut c_void,
                        ) -> CXChildVisitResult {
                            unsafe {
                                if v_cur.kind == CXCursor_EnumConstantDecl {
                                    let variants = &mut *(client_data as *mut Vec<CEnumVariant>);
                                    let v_name = get_cursor_spelling(v_cur);
                                    let v_val = clang_getEnumConstantDeclValue(v_cur);
                                    variants.push(CEnumVariant {
                                        name: v_name,
                                        value: v_val,
                                    });
                                }
                                CXChildVisit_Continue
                            }
                        }

                        clang_visitChildren(
                            cursor,
                            visit_variants,
                            &mut variants as *mut _ as *mut c_void,
                        );

                        let is_dup = if name.is_empty() {
                            variants.first().map(|v: &CEnumVariant| {
                                state.program.enums.iter().any(|e| e.variants.iter().any(|ev| ev.name == v.name))
                            }).unwrap_or(false)
                        } else {
                            state.program.enums.iter().any(|e| e.name == name)
                        };

                        if !variants.is_empty() && !is_dup {
                            state.program.enums.push(CEnum { name, variants });
                        }
                    }
                    CXCursor_TypedefDecl => {
                        let raw_name = get_cursor_spelling(cursor);
                        let name = crate::types::to_pascal_case(&raw_name);
                        let underlying = clang_getTypedefDeclUnderlyingType(cursor);
                        let underlying_str = c_type_to_arandu(underlying);
                        if !name.is_empty() && state.typedef_set.insert(name.clone()) {
                            state.program.typedefs.push(CTypedef {
                                name,
                                underlying_ty: underlying_str,
                            });
                        }
                    }
                    CXCursor_VarDecl => {
                        let name = get_cursor_spelling(cursor);
                        if !name.is_empty() {
                            let c_ty = clang_getCursorType(cursor);
                            ensure_record_registered(c_ty, state);
                            let ty_str = c_type_to_arandu(c_ty);
                            let is_const = clang_isConstQualifiedType(c_ty) != 0;
                            let init_cursor = clang_Cursor_getVarDeclInitializer(cursor);
                            let init = if clang_Cursor_isNull(init_cursor) == 0 && clang_isInvalid(init_cursor.kind) == 0 {
                                Some(parse_expr(init_cursor))
                            } else {
                                None
                            };
                            if state.global_set.insert(name.clone()) {
                                state.program.globals.push(CGlobal {
                                    name,
                                    ty: ty_str,
                                    init,
                                    is_const,
                                });
                            } else if init.is_some() {
                                if let Some(existing) = state.program.globals.iter_mut().find(|g| g.name == name) {
                                    if existing.init.is_none() {
                                        existing.init = init;
                                        existing.ty = ty_str;
                                        existing.is_const = is_const;
                                    }
                                }
                            }
                        }
                    }
                    CXCursor_FunctionDecl => {
                        let name = get_cursor_spelling(cursor);
                        static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                        let count = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                        if count % 500 == 0 {
                            eprintln!("⏱️ Processada função #{count}: {name}");
                        }
                        let fn_type = clang_getCursorType(cursor);
                        let ret_type = clang_getResultType(fn_type);
                        let ret_str = c_type_to_arandu(ret_type);

                        let num_args = clang_Cursor_getNumArguments(cursor);
                        let mut params = Vec::new();
                        for i in 0..num_args {
                            let arg_cur = clang_Cursor_getArgument(cursor, i as u32);
                            let mut arg_name = get_cursor_spelling(arg_cur);
                            if arg_name.is_empty() {
                                arg_name = format!("arg{i}");
                            }
                            let arg_type = clang_getCursorType(arg_cur);
                            let arg_str = if arg_type.kind == CXType_ConstantArray
                                || arg_type.kind == CXType_IncompleteArray
                                || arg_type.kind == CXType_VariableArray
                            {
                                let elem = clang_getArrayElementType(arg_type);
                                format!("ptr[{}]", c_type_to_arandu(elem))
                            } else {
                                c_type_to_arandu(arg_type)
                            };
                            params.push(CParam {
                                name: arg_name,
                                ty: arg_str,
                                is_mut: false,
                            });
                        }

                        let is_def = clang_isCursorDefinition(cursor) != 0;
                        let is_variadic = clang_isFunctionTypeVariadic(fn_type) != 0;

                        // Se for uma definição, extrair o corpo (CompoundStmt)
                        let body = if is_def {
                            let children = get_children(cursor);
                            let mut b = children
                                .into_iter()
                                .find(|c| c.kind == CXCursor_CompoundStmt)
                                .map(|c| parse_stmt(c));

                            if let Some(ref mut stmts) = b {
                                let mut mutated = std::collections::HashSet::new();
                                collect_mutated_vars(stmts, &mut mutated);
                                for p in &mut params {
                                    if mutated.contains(&p.name) {
                                        p.is_mut = true;
                                        let local_name = format!("local_{}", p.name);
                                        rename_var_in_stmts(stmts, &p.name, &local_name);
                                    }
                                }
                            }
                            b
                        } else {
                            None
                        };

                        if !name.is_empty() {
                            if let Some(&pos) = state.fn_indices.get(&name) {
                                if !state.program.functions[pos].is_definition && is_def {
                                    state.program.functions[pos] = CFunction {
                                        name,
                                        return_ty: ret_str,
                                        params,
                                        body,
                                        is_definition: is_def,
                                        is_variadic,
                                    };
                                }
                            } else {
                                state.fn_indices.insert(name.clone(), state.program.functions.len());
                                state.program.functions.push(CFunction {
                                    name,
                                    return_ty: ret_str,
                                    params,
                                    body,
                                    is_definition: is_def,
                                    is_variadic,
                                });
                            }
                        }
                    }
                    _ => {}
                }

                CXChildVisit_Continue
            }
        }

        eprintln!("⏱️ Iniciando clang_visitChildren...");
        clang_visitChildren(
            root_cursor,
            visit_top_level,
            &mut state as *mut _ as *mut c_void,
        );
        eprintln!("⏱️ Concluído clang_visitChildren.");

        clang_disposeTranslationUnit(tu);
        clang_disposeIndex(index);

        Ok(program)
    }
}
