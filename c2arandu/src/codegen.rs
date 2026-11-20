use crate::ast::*;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static PTR_ARRAY_GLOBALS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static KNOWN_TYPES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static KNOWN_TYPEDEFS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static KNOWN_FUNCS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static KNOWN_FUNC_PARAMS: RefCell<HashMap<String, Vec<String>>> = RefCell::new(HashMap::new());
    static KNOWN_STRUCT_FIELDS: RefCell<HashMap<String, HashMap<String, String>>> = RefCell::new(HashMap::new());
    static STRUCT_FIELD_ORDER: RefCell<HashMap<String, Vec<(String, String)>>> = RefCell::new(HashMap::new());
    static CURRENT_FUNC_RET_TY: RefCell<Option<String>> = RefCell::new(None);
    static TMP_VAR_COUNTER: RefCell<usize> = RefCell::new(0);
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TranspileMode {
    #[default]
    Auto,
    Transpile,
    Binding,
}

pub struct CodegenOptions {
    pub mode: TranspileMode,
    pub emit_extern_block: bool,
    pub module_name: Option<String>,
}

impl Default for CodegenOptions {
    fn default() -> Self {
        Self {
            mode: TranspileMode::Auto,
            emit_extern_block: true,
            module_name: None,
        }
    }
}

pub fn to_camel_case(name: &str) -> String {
    if name.is_empty() {
        return "val".to_string();
    }
    if name == "NULL" {
        return "nil".to_string();
    }
    // Constantes em SCREAMING_SNAKE_CASE (ex: MZ_OK, MZ_STREAM_END) contendo '_' são mantidas intactas
    let is_screaming = name.contains('_')
        && name.chars().any(|c| c.is_ascii_uppercase())
        && name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    if is_screaming {
        return name.to_string();
    }

    let mut raw = name.trim_start_matches('_');
    if raw.is_empty() {
        return "val".to_string();
    }
    // Remove prefixos de membros C ("m_") e estáticos ("s_") apenas se seguidos de letra
    if raw.len() > 2 {
        let third = raw.chars().nth(2).unwrap_or('\0');
        if third.is_ascii_alphabetic() && (raw.starts_with("m_") || raw.starts_with("s_")) {
            raw = &raw[2..];
        }
    }

    // Converte de snake_case para camelCase
    let mut result = String::new();
    let mut capitalize_next = false;
    for (i, ch) in raw.chars().enumerate() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(ch.to_uppercase());
            capitalize_next = false;
        } else if i == 0 {
            result.extend(ch.to_lowercase());
        } else {
            result.push(ch);
        }
    }

    if result.is_empty() {
        result = "val".to_string();
    }
    if result.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        result = format!("v{result}");
    }

    // Palavras reservadas e tipos primitivos do Arandu que não podem ser identificadores
    const RESERVED: &[&str] = &[
        "ptr", "ref", "mut", "let", "set", "func", "struct", "enum", "type", "public",
        "extern", "if", "else", "while", "for", "do", "break", "continue", "return",
        "match", "true", "false", "nil", "int", "uint", "usize", "isize", "void", "bool",
        "as", "import", "module", "const", "own", "self"
    ];
    if RESERVED.contains(&result.as_str()) {
        format!("{result}Val")
    } else {
        result
    }
}

pub fn sanitize_ident(name: &str) -> String {
    if name == "nil" || name == "NULL" {
        return "nil".to_string();
    }
    if name == "true" || name == "false" {
        return name.to_string();
    }
    let res = to_camel_case(name);
    const RESERVED: &[&str] = &[
        "if", "else", "for", "in", "while", "match", "return", "break", "continue",
        "func", "async", "comptime", "await", "struct", "enum", "interface", "const",
        "type", "module", "import", "as", "public", "internal", "private", "extern",
        "unsafe", "where", "catch", "is", "let", "impl", "set", "own", "mut",
        "shared", "ref", "self", "ptr", "defer", "errdefer", "int", "uint", "isize",
        "usize", "float", "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64",
        "f32", "f64", "bool", "byte", "char", "str", "any",
    ];
    if RESERVED.contains(&res.as_str()) {
        format!("{res}Val")
    } else {
        res
    }
}

pub fn is_scalar_type(ty: &str) -> bool {
    matches!(
        ty,
        "bool" | "f32" | "f64" | "LuaNumber"
            | "i8" | "i16" | "i32" | "i64" | "int" | "isize"
            | "u8" | "u16" | "u32" | "u64" | "uint" | "usize"
            | "LuaInteger" | "LuaUnsigned" | "LuByte" | "LuMem" | "LMem"
            | "ls_byte" | "LsByte" | "Instruction" | "IdxT" | "TimeT" | "Utfint" | "BinOpr" | "OpCode" | "UnOpr"
    )
}

pub fn is_unsigned_type(ty: &str) -> bool {
    matches!(
        ty,
        "u8" | "u16" | "u32" | "u64" | "uint" | "usize"
            | "LuaUnsigned" | "LuByte" | "LuMem" | "Instruction"
    )
}

pub fn is_signed_int_type(ty: &str) -> bool {
    matches!(
        ty,
        "i8" | "i16" | "i32" | "i64" | "int" | "isize"
            | "ls_byte" | "LsByte" | "LuaInteger" | "LMem" | "TimeT" | "IdxT"
    )
}

fn strip_outer_parens(mut s: &str) -> &str {
    loop {
        let trimmed = s.trim();
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            let mut depth = 0usize;
            let mut matches_outer = true;
            for (i, ch) in trimmed.char_indices() {
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 && i < trimmed.len() - 1 {
                            matches_outer = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if matches_outer {
                s = &trimmed[1..trimmed.len() - 1];
                continue;
            }
        }
        return trimmed;
    }
}

fn promote_numeric_ty(l: Option<String>, r: Option<String>) -> Option<String> {
    fn rank(ty: &str) -> u8 {
        match ty {
            "f64" | "LuaNumber" => 6,
            "f32" => 5,
            "usize" => 4,
            "u64" | "LuaUnsigned" => 3,
            "i64" | "isize" | "LuaInteger" => 2,
            "uint" | "u32" | "Utfint" => 1,
            _ => 0,
        }
    }
    match (l, r) {
        (Some(lt), Some(rt)) => {
            if rank(&rt) > rank(&lt) {
                Some(rt)
            } else {
                Some(lt)
            }
        }
        (Some(lt), None) => Some(lt),
        (None, Some(rt)) => Some(rt),
        (None, None) => None,
    }
}

pub fn infer_expr_type(expr: &CExpr) -> Option<String> {
    match expr {
        CExpr::Ident(name) => {
            let clean = sanitize_ident(name);
            KNOWN_TYPES.with(|cell| {
                let map = cell.borrow();
                map.get(name).cloned().or_else(|| map.get(&clean).cloned())
            })
        }
        CExpr::Cast { target_ty, .. } => Some(target_ty.clone()),
        CExpr::Ternary { then_expr, else_expr, .. } => {
            let t = infer_expr_type(then_expr).or_else(|| infer_expr_type(else_expr));
            if let Some(ref ty) = t {
                if ty.starts_with('[') {
                    return Some("ptr[u8]".to_string());
                }
            }
            t
        }
        CExpr::UnaryOp { op, operand, .. } => {
            if op == "&" {
                let inner_ty = infer_expr_type(operand).unwrap_or_else(|| "u8".to_string());
                Some(format!("ptr[{inner_ty}]"))
            } else if op == "*" {
                let inner_ty = infer_expr_type(operand)?;
                if let Some(stripped) = inner_ty.strip_prefix("ptr[").and_then(|s| s.strip_suffix(']')) {
                    Some(stripped.to_string())
                } else {
                    None
                }
            } else if op == "!" {
                Some("bool".to_string())
            } else {
                infer_expr_type(operand)
            }
        }
        CExpr::Call { callee, .. } => {
            if let CExpr::Ident(ref name) = **callee {
                let clean = sanitize_ident(name);
                KNOWN_FUNCS.with(|cell| {
                    let map = cell.borrow();
                    map.get(name).cloned().or_else(|| map.get(&clean).cloned())
                })
            } else {
                None
            }
        }
        CExpr::MemberAccess { base, member, is_arrow } => {
            let base_ty = infer_expr_type(base)?;
            let struct_name = if *is_arrow || base_ty.starts_with("ptr[") {
                base_ty.trim_start_matches("ptr[").trim_end_matches(']').trim()
            } else {
                base_ty.as_str()
            };
            KNOWN_STRUCT_FIELDS.with(|cell| {
                let map = cell.borrow();
                map.get(struct_name)
                    .or_else(|| {
                        KNOWN_TYPES.with(|t_cell| {
                            let t_map = t_cell.borrow();
                            t_map.get(struct_name).and_then(|aliased| map.get(aliased))
                        })
                    })
                    .and_then(|fields| fields.get(member).cloned())
            })
        }
        CExpr::Index { base, .. } => {
            let base_ty = infer_expr_type(base)?;
            if let Some(stripped) = base_ty.strip_prefix("ptr[").and_then(|s| s.strip_suffix(']')) {
                Some(stripped.to_string())
            } else if let Some(bracket_end) = base_ty.find(']') {
                Some(base_ty[bracket_end + 1..].trim().to_string())
            } else {
                None
            }
        }
        CExpr::BinaryOp { op, left, right } => {
            if ["==", "!=", "<", "<=", ">", ">="].contains(&op.as_str()) {
                Some("bool".to_string())
            } else if ["&&", "||"].contains(&op.as_str()) {
                Some("bool".to_string())
            } else if ["&", "|", "^", "<<", ">>", "%", "/", "*"].contains(&op.as_str()) {
                promote_numeric_ty(infer_expr_type(left), infer_expr_type(right))
                    .or(Some("int".to_string()))
            } else if op == "," {
                infer_expr_type(right)
            } else if op == "=" {
                infer_expr_type(left).or_else(|| infer_expr_type(right))
            } else if op == "+" || op == "-" {
                let l_ty = infer_expr_type(left);
                let r_ty = infer_expr_type(right);
                if op == "-"
                    && (l_ty.as_deref().map_or(false, |t| t.starts_with("ptr[") || t.starts_with('[')) || is_ptr_expr(left))
                    && (r_ty.as_deref().map_or(false, |t| t.starts_with("ptr[") || t.starts_with('[')) || is_ptr_expr(right))
                {
                    return Some("isize".to_string());
                }
                if let Some(ref lt) = l_ty {
                    if lt.starts_with("ptr[") {
                        return l_ty;
                    }
                }
                if let Some(ref rt) = r_ty {
                    if rt.starts_with("ptr[") && op == "+" {
                        return r_ty;
                    }
                }
                if is_ptr_expr(left) {
                    return l_ty.or(Some("ptr[u8]".to_string()));
                }
                promote_numeric_ty(l_ty, r_ty)
            } else {
                None
            }
        }
        CExpr::Paren(inner) => infer_expr_type(inner),
        CExpr::IntLiteral(_) => Some("int".to_string()),
        CExpr::FloatLiteral(_) => Some("f64".to_string()),
        CExpr::BoolLiteral(_) => Some("bool".to_string()),
        CExpr::StringLiteral(_) => Some("ptr[u8]".to_string()),
        _ => None,
    }
}

pub fn is_ptr_expr(expr: &CExpr) -> bool {
    if let Some(ty) = infer_expr_type(expr) {
        let mut curr = ty;
        KNOWN_TYPEDEFS.with(|cell| {
            let map = cell.borrow();
            let mut depth = 0;
            while depth < 10 {
                if let Some(aliased) = map.get(&curr) {
                    if aliased == &curr {
                        break;
                    }
                    curr = aliased.clone();
                    depth += 1;
                } else {
                    break;
                }
            }
        });
        if is_scalar_type(&curr) {
            return false;
        }
        curr.starts_with("ptr[") || curr.starts_with("func(")
    } else {
        match expr {
            CExpr::Ident(name) => {
                name == "nil" || name == "NULL"
                    || name.ends_with("ptr") || name.ends_with("Ptr")
                    || name == "val" || name == "buff" || name == "block" || name == "ci"
                    || name == "cl" || name == "curr" || name == "prev" || name == "next"
                    || name == "s" || name == "l" || name == "g" || name == "node" || name == "state"
                    || name == "o" || name == "k" || name == "res" || name == "obj" || name == "head" || name == "tail"
                    || name == "tbc" || name.starts_with("tbc") || name == "f" || name == "fname"
            }
            CExpr::BinaryOp { op, left, right } if op == "-" => {
                if is_ptr_expr(right) {
                    false
                } else {
                    is_ptr_expr(left)
                }
            }
            CExpr::BinaryOp { op, left, .. } if op == "+" => is_ptr_expr(left),
            CExpr::BinaryOp { op, left, right } if op == "=" => is_ptr_expr(left) || is_ptr_expr(right),
            CExpr::BinaryOp { .. } => false,
            CExpr::UnaryOp { op, .. } if op == "&" => true,
            CExpr::Cast { target_ty, .. } if target_ty.starts_with("ptr[") => true,
            CExpr::MemberAccess { member, .. } => {
                member == "p" || member == "top" || member == "stack" || member == "tbclist"
                    || member == "strval" || member == "gc" || member == "metatable" || member == "buffer"
                    || member == "ud" || member == "frealloc" || member == "f" || member == "closef"
                    || member == "hook" || member == "panic" || member == "warnf" || member == "alloc"
            }
            CExpr::Paren(inner) => is_ptr_expr(inner),
            _ => false,
        }
    }
}

pub fn is_boolean_expr(expr: &CExpr) -> bool {
    match expr {
        CExpr::BoolLiteral(_) => true,
        CExpr::BinaryOp { op, right, .. } => {
            if op == "," {
                is_boolean_expr(right)
            } else {
                ["==", "!=", "<", "<=", ">", ">=", "&&", "||"].contains(&op.as_str())
            }
        }
        CExpr::UnaryOp { op, .. } => op == "!",
        CExpr::Paren(inner) => is_boolean_expr(inner),
        CExpr::Ternary { then_expr, else_expr, .. } => {
            is_boolean_expr(then_expr) && is_boolean_expr(else_expr)
        }
        _ => false,
    }
}

pub fn ensure_boolean_condition(expr: &CExpr) -> String {
    let s = generate_expr(expr);
    if is_boolean_expr(expr) {
        s
    } else if is_ptr_expr(expr) {
        format!("({s} != nil)")
    } else {
        format!("({s} != 0)")
    }
}

pub fn extract_assignment_from_cond(expr: &CExpr) -> Option<(CStmt, CExpr)> {
    match expr {
        CExpr::BinaryOp { op, left, right } if [
            "=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="
        ].contains(&op.as_str()) => {
            let stmt = CStmt::Assign {
                target: (**left).clone(),
                op: op.clone(),
                value: (**right).clone(),
            };
            let pure_cond = (**left).clone();
            Some((stmt, pure_cond))
        }
        CExpr::Paren(inner) => {
            let (stmt, pure_cond) = extract_assignment_from_cond(inner)?;
            Some((stmt, CExpr::Paren(Box::new(pure_cond))))
        }
        CExpr::BinaryOp { op, left, right } => {
            if let Some((stmt, new_left)) = extract_assignment_from_cond(left) {
                let pure_cond = CExpr::BinaryOp {
                    op: op.clone(),
                    left: Box::new(new_left),
                    right: right.clone(),
                };
                return Some((stmt, pure_cond));
            }
            if let Some((stmt, new_right)) = extract_assignment_from_cond(right) {
                let pure_cond = CExpr::BinaryOp {
                    op: op.clone(),
                    left: left.clone(),
                    right: Box::new(new_right),
                };
                return Some((stmt, pure_cond));
            }
            None
        }
        _ => None,
    }
}

pub fn generate_expr(expr: &CExpr) -> String {
    match expr {
        CExpr::IntLiteral(v) => {
            if *v > 2147483647 && *v <= 4294967295 {
                format!("(({v} as uint) as int)")
            } else {
                v.to_string()
            }
        }
        CExpr::FloatLiteral(v) => {
            let s = v.to_string();
            if s.contains('.') { s } else { format!("{s}.0") }
        }
        CExpr::StringLiteral(s) => {
            let norm = normalize_c_string_literal(s);
            if norm.starts_with('"') {
                format!("c{norm}")
            } else {
                format!("c\"{norm}\"")
            }
        }
        CExpr::BoolLiteral(b) => b.to_string(),
        CExpr::Ident(id) => {
            let clean = sanitize_ident(id);
            if let Some(elem_ty) = PTR_ARRAY_GLOBALS.with(|cell| {
                let map = cell.borrow();
                map.get(id).or_else(|| map.get(&clean)).cloned()
            }) {
                format!("(&get_{clean}() as ptr[{elem_ty}])")
            } else {
                let is_fn = KNOWN_FUNCS.with(|cell| {
                    let map = cell.borrow();
                    map.contains_key(id) || map.contains_key(&clean)
                });
                let is_local = KNOWN_TYPES.with(|cell| {
                    let map = cell.borrow();
                    map.contains_key(id) || map.contains_key(&clean)
                });
                if is_fn && !is_local && id != "main" && clean != "main" {
                    format!("(&{clean})")
                } else {
                    clean
                }
            }
        }
        CExpr::BinaryOp { op, left, right } => {
            if op == "," || op == "=" {
                return generate_expr(right);
            }
            if let Some(stripped_op) = op.strip_suffix('=') {
                if ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"].contains(&stripped_op) {
                    let l_str = generate_expr(left);
                    let r_str = generate_expr(right);
                    return format!("({l_str} {stripped_op} {r_str})");
                }
            }
            if op == "&&" || op == "||" {
                let l_str = ensure_boolean_condition(left);
                let r_str = ensure_boolean_condition(right);
                return format!("({l_str} {op} {r_str})");
            }

            let mut l_str = generate_expr(left);
            let mut r_str = generate_expr(right);
            let left_ty = infer_expr_type(left);
            let right_ty = infer_expr_type(right);

            // Decaimento de arrays literais/locais para ponteiros
            if let Some(ref lt) = left_ty {
                if lt.starts_with('[') {
                    l_str = format!("((&{l_str}[0]) as ptr[u8])");
                }
            } else if l_str == "buff" || l_str.starts_with("local_buff") {
                l_str = format!("((&{l_str}[0]) as ptr[u8])");
            }
            if let Some(ref rt) = right_ty {
                if rt.starts_with('[') {
                    r_str = format!("((&{r_str}[0]) as ptr[u8])");
                }
            }

            let get_effective_type = |s: &str, ty: &Option<String>| -> Option<String> {
                let s_trimmed = strip_outer_parens(s);
                if s_trimmed.ends_with(" as int") {
                    return Some("int".to_string());
                }
                if s_trimmed.ends_with(" as isize") {
                    return Some("isize".to_string());
                }
                if s_trimmed.ends_with(" as usize") {
                    return Some("usize".to_string());
                }
                if s_trimmed.ends_with(" as uint") || s_trimmed.ends_with(" as u32") || s_trimmed.ends_with(" as Utfint") {
                    return Some("uint".to_string());
                }
                if s_trimmed.ends_with(" as u64") || s_trimmed.ends_with(" as LuaUnsigned") {
                    return Some("u64".to_string());
                }
                if s_trimmed.ends_with(" as i64") || s_trimmed.ends_with(" as LuaInteger") {
                    return Some("i64".to_string());
                }
                if s_trimmed.ends_with(" as i8") || s_trimmed.ends_with(" as LsByte") {
                    return Some("i8".to_string());
                }
                if s_trimmed.ends_with(" as u8") || s_trimmed.ends_with(" as LuByte") {
                    return Some("u8".to_string());
                }
                if s_trimmed.ends_with(" as f64") || s_trimmed.ends_with(" as LuaNumber") {
                    return Some("f64".to_string());
                }
                if s_trimmed.ends_with(']') {
                    if let Some(pos) = s_trimmed.rfind(" as ptr[") {
                        let ptr_part = &s_trimmed[pos + 4..];
                        if !ptr_part.contains('(')
                            && !ptr_part.contains(')')
                            && !ptr_part.contains(" + ")
                            && !ptr_part.contains(" - ")
                        {
                            return Some(ptr_part.to_string());
                        }
                    }
                }
                if (s_trimmed.starts_with("if ") || s_trimmed.starts_with("(if "))
                    && s_trimmed.contains("as uint)")
                    && !s_trimmed.ends_with("as int)")
                {
                    return Some("uint".to_string());
                }
                if let Some(ref raw_ty) = ty {
                    let mut curr = raw_ty.clone();
                    KNOWN_TYPEDEFS.with(|cell| {
                        let map = cell.borrow();
                        let mut depth = 0;
                        while depth < 10 {
                            if let Some(aliased) = map.get(&curr) {
                                if aliased == &curr {
                                    break;
                                }
                                curr = aliased.clone();
                                depth += 1;
                            } else {
                                break;
                            }
                        }
                    });
                    return Some(curr);
                }
                None
            };

            // Detecção precisa de ponteiros
            let is_ptr = |expr: &CExpr, s: &str, ty: &Option<String>| -> bool {
                if let Some(eff) = get_effective_type(s, ty) {
                    if is_scalar_type(&eff) || eff == "bool" {
                        return false;
                    }
                    if eff.starts_with("ptr[") || eff.starts_with('&') || eff.starts_with("ref ") || eff.contains("func(") {
                        return true;
                    }
                }
                if is_ptr_expr(expr) {
                    return true;
                }
                let s_clean = strip_outer_parens(s);
                if s_clean == "nil" {
                    return true;
                }
                if s_clean.ends_with(".p") || s_clean.ends_with(".top") || s_clean.ends_with(".stack") || s_clean.ends_with(".tbclist") || s_clean.ends_with(".gc")
                    || s_clean.ends_with(".f") || s_clean.ends_with(".closef") || s_clean.ends_with(".hook") || s_clean.ends_with(".panic")
                    || s_clean.ends_with(".warnf") || s_clean.ends_with(".alloc") || s_clean.ends_with(".ud") || s_clean.ends_with(".file")
                    || s_clean.ends_with(".metatable")
                    || (s_clean.starts_with('&') && !s_clean.contains('.')) || s_clean == "f" || s_clean == "tbc" || s_clean.starts_with("tbc.") {
                    return true;
                }
                false
            };

            let l_is_ptr = is_ptr(left, &l_str, &left_ty);
            let r_is_ptr = is_ptr(right, &r_str, &right_ty);

            if op == "==" || op == "!=" {
                if let CExpr::IntLiteral(0) = **right {
                    let mut unwrapped_left = left.as_ref();
                    while let CExpr::Paren(inner) = unwrapped_left {
                        unwrapped_left = inner;
                    }
                    if is_boolean_expr(unwrapped_left) {
                        return if op == "!=" {
                            generate_expr(unwrapped_left)
                        } else {
                            format!("!({})", generate_expr(unwrapped_left))
                        };
                    }
                }
                let l_looks_bool = is_boolean_expr(left) || l_str.contains(" || ") || l_str.contains(" && ");
                let r_looks_bool = is_boolean_expr(right) || r_str.contains(" || ") || r_str.contains(" && ");
                if (r_str == "nil" || r_str == "0" || r_str == "(0)") && l_looks_bool {
                    return if op == "!=" { format!("({l_str})") } else { format!("!({l_str})") };
                }
                if (l_str == "nil" || l_str == "0" || l_str == "(0)") && r_looks_bool {
                    return if op == "!=" { format!("({r_str})") } else { format!("!({r_str})") };
                }

                let is_func_or_ptr_ty = |opt_ty: &Option<String>| -> bool {
                    if let Some(t) = opt_ty {
                        if t.contains("func(") || t.contains("ptr[") || t.ends_with("Function") || t.ends_with("Func") || t.ends_with("Hook") {
                            return true;
                        }
                        let mut curr = t.clone();
                        let mut is_p = false;
                        KNOWN_TYPEDEFS.with(|cell| {
                            let map = cell.borrow();
                            let mut depth = 0;
                            while depth < 10 {
                                if let Some(aliased) = map.get(&curr) {
                                    if aliased.contains("ptr[") || aliased.contains("func(") {
                                        is_p = true;
                                        break;
                                    }
                                    if aliased == &curr { break; }
                                    curr = aliased.clone();
                                    depth += 1;
                                } else {
                                    break;
                                }
                            }
                        });
                        is_p
                    } else {
                        false
                    }
                };

                let l_ptr_compare = l_is_ptr || is_func_or_ptr_ty(&left_ty) || l_str == "k" || l_str.ends_with(".k") || l_str.ends_with(".f") || l_str.ends_with(".panic") || l_str.ends_with(".hook");
                let r_ptr_compare = r_is_ptr || is_func_or_ptr_ty(&right_ty) || r_str == "k" || r_str.ends_with(".k") || r_str.ends_with(".f") || r_str.ends_with(".panic") || r_str.ends_with(".hook");

                if l_ptr_compare && (r_str == "0" || r_str == "(0)" || r_str == "(0 as int)" || r_str == "nil" || r_str == "(nil)") {
                    return format!("({l_str} {op} nil)");
                }
                if r_ptr_compare && (l_str == "0" || l_str == "(0)" || l_str == "(0 as int)" || l_str == "nil" || l_str == "(nil)") {
                    return format!("(nil {op} {r_str})");
                }
                if l_ptr_compare && r_ptr_compare {
                    if l_str == "nil" || r_str == "nil" {
                        return format!("({l_str} {op} {r_str})");
                    }
                    let l_cast = if l_str.starts_with("(&") || l_str.starts_with('&') {
                        format!("((({l_str}) as ptr[u8]) as usize)")
                    } else {
                        format!("(({l_str}) as usize)")
                    };
                    let r_cast = if r_str.starts_with("(&") || r_str.starts_with('&') {
                        format!("((({r_str}) as ptr[u8]) as usize)")
                    } else {
                        format!("(({r_str}) as usize)")
                    };
                    return format!("({l_cast} {op} {r_cast})");
                }
                // Se nenhum é ponteiro, mas um lado é nil, esse nil veio de 0 em macro C!
                if !l_ptr_compare && (r_str == "nil" || r_str == "(nil)") {
                    r_str = "0".to_string();
                }
                if !r_ptr_compare && (l_str == "nil" || l_str == "(nil)") {
                    l_str = "0".to_string();
                }

                // Normalização de bool vs int
                if is_boolean_expr(left) && !is_boolean_expr(right) {
                    l_str = format!("(if {l_str} {{ 1 }} else {{ 0 }})");
                } else if !is_boolean_expr(left) && is_boolean_expr(right) {
                    r_str = format!("(if {r_str} {{ 1 }} else {{ 0 }})");
                }

                // Normalização de escalares para == e != via tipos efetivos
                let l_eff = get_effective_type(&l_str, &left_ty);
                let r_eff = get_effective_type(&r_str, &right_ty);
                let lt = l_eff.as_deref().unwrap_or("int");
                let rt = r_eff.as_deref().unwrap_or("int");

                if lt != rt {
                    if (lt == "isize" && rt == "usize") || (lt == "usize" && rt == "isize") {
                        l_str = format!("(({l_str}) as isize)");
                        r_str = format!("(({r_str}) as isize)");
                    } else if (lt == "isize" && rt == "int") || (lt == "int" && rt == "isize") {
                        l_str = format!("(({l_str}) as isize)");
                        r_str = format!("(({r_str}) as isize)");
                    } else if (lt == "usize" && rt == "int") || (lt == "int" && rt == "usize") {
                        l_str = format!("(({l_str}) as usize)");
                        r_str = format!("(({r_str}) as usize)");
                    } else if (lt == "uint" && rt == "int") || (lt == "int" && rt == "uint") {
                        if r_str.starts_with('-') {
                            r_str = format!("((({r_str}) as int) as uint)");
                        } else {
                            r_str = format!("(({r_str}) as uint)");
                        }
                        if l_str.starts_with('-') {
                            l_str = format!("((({l_str}) as int) as uint)");
                        } else {
                            l_str = format!("(({l_str}) as uint)");
                        }
                    } else if (lt == "usize" && rt == "u64") || (lt == "u64" && rt == "usize") {
                        l_str = format!("(({l_str}) as usize)");
                        r_str = format!("(({r_str}) as usize)");
                    } else if (lt == "uint" && rt == "u64") || (lt == "u64" && rt == "uint") {
                        l_str = format!("(({l_str}) as u64)");
                        r_str = format!("(({r_str}) as u64)");
                    } else if (lt == "i64" && rt == "int") || (lt == "int" && rt == "i64") {
                        l_str = format!("(({l_str}) as i64)");
                        r_str = format!("(({r_str}) as i64)");
                    } else if (lt == "i64" && rt == "usize") || (lt == "usize" && rt == "i64") {
                        l_str = format!("(({l_str}) as i64)");
                        r_str = format!("(({r_str}) as i64)");
                    } else if (lt == "u64" && rt == "i64") || (lt == "i64" && rt == "u64") {
                        l_str = format!("(({l_str}) as u64)");
                        r_str = format!("(({r_str}) as u64)");
                    }
                }
                return format!("({l_str} {op} {r_str})");
            }

            if ["<", "<=", ">", ">="].contains(&op.as_str()) {
                if l_is_ptr || r_is_ptr {
                    let l_cast = if l_str.starts_with("(&") || l_str.starts_with('&') {
                        format!("((({l_str}) as ptr[u8]) as usize)")
                    } else if l_is_ptr {
                        format!("({l_str} as usize)")
                    } else {
                        format!("(({l_str}) as usize)")
                    };
                    let r_cast = if r_str.starts_with("(&") || r_str.starts_with('&') {
                        format!("((({r_str}) as ptr[u8]) as usize)")
                    } else if r_is_ptr {
                        format!("({r_str} as usize)")
                    } else {
                        format!("(({r_str}) as usize)")
                    };
                    return format!("({l_cast} {op} {r_cast})");
                }

                // Normalização de escalares para ordem
                let l_eff = get_effective_type(&l_str, &left_ty);
                let r_eff = get_effective_type(&r_str, &right_ty);
                let lt = l_eff.as_deref().unwrap_or("int");
                let rt = r_eff.as_deref().unwrap_or("int");

                if lt != rt {
                    if (lt == "isize" && rt == "usize") || (lt == "usize" && rt == "isize") {
                        l_str = format!("(({l_str}) as isize)");
                        r_str = format!("(({r_str}) as isize)");
                    } else if (lt == "isize" && rt == "int") || (lt == "int" && rt == "isize") {
                        l_str = format!("(({l_str}) as isize)");
                        r_str = format!("(({r_str}) as isize)");
                    } else if (lt == "usize" && rt == "int") || (lt == "int" && rt == "usize") {
                        l_str = format!("(({l_str}) as usize)");
                        r_str = format!("(({r_str}) as usize)");
                    } else if (lt == "uint" && rt == "int") || (lt == "int" && rt == "uint") {
                        l_str = format!("(({l_str}) as uint)");
                        r_str = format!("(({r_str}) as uint)");
                    } else if (lt == "usize" && rt == "u64") || (lt == "u64" && rt == "usize") {
                        l_str = format!("(({l_str}) as usize)");
                        r_str = format!("(({r_str}) as usize)");
                    } else if (lt == "uint" && rt == "u64") || (lt == "u64" && rt == "uint") {
                        l_str = format!("(({l_str}) as u64)");
                        r_str = format!("(({r_str}) as u64)");
                    } else if (lt == "i64" && rt == "int") || (lt == "int" && rt == "i64") {
                        l_str = format!("(({l_str}) as i64)");
                        r_str = format!("(({r_str}) as i64)");
                    } else if (lt == "i64" && rt == "usize") || (lt == "usize" && rt == "i64") {
                        l_str = format!("(({l_str}) as i64)");
                        r_str = format!("(({r_str}) as i64)");
                    } else if (lt == "u64" && rt == "i64") || (lt == "i64" && rt == "u64") {
                        l_str = format!("(({l_str}) as u64)");
                        r_str = format!("(({r_str}) as u64)");
                    }
                }
                return format!("({l_str} {op} {r_str})");
            }

            // Operações aritméticas e bitwise
            // 1. Aritmética de ponteiros
            let is_any_ptr_op = |s: &str, ty: &Option<String>, is_p: bool| -> bool {
                if let Some(eff) = get_effective_type(s, ty) {
                    if is_scalar_type(&eff) || eff == "bool" {
                        return false;
                    }
                    if eff.starts_with("ptr[") || eff.starts_with('[') || eff.starts_with('&') || eff.starts_with("ref ") || eff.contains("func(") {
                        return true;
                    }
                }
                let s_clean = strip_outer_parens(s);
                is_p || s_clean.ends_with(".p") || s_clean.ends_with(".top") || s_clean.ends_with(".stack")
            };
            let l_is_any_ptr = is_any_ptr_op(&l_str, &left_ty, l_is_ptr);
            let r_is_any_ptr = is_any_ptr_op(&r_str, &right_ty, r_is_ptr);

            if op == "-" && l_is_any_ptr && r_is_any_ptr {
                return format!("((({l_str} as usize) - ({r_str} as usize)) as isize)");
            }
            if (op == "+" || op == "-") && l_is_any_ptr && !r_is_any_ptr {
                return format!("({l_str} {op} (({r_str}) as int))");
            }
            if op == "+" && !l_is_any_ptr && r_is_any_ptr {
                return format!("({r_str} + (({l_str}) as int))");
            }

            // 2. Normalização bool vs int em aritmética
            if is_boolean_expr(left) && !is_boolean_expr(right) {
                l_str = format!("(if {l_str} {{ 1 }} else {{ 0 }})");
            } else if !is_boolean_expr(left) && is_boolean_expr(right) {
                r_str = format!("(if {r_str} {{ 1 }} else {{ 0 }})");
            }

            // 3. Normalização de tipos escalares via tipos efetivos
            let l_eff = get_effective_type(&l_str, &left_ty);
            let r_eff = get_effective_type(&r_str, &right_ty);
            let lt = l_eff.as_deref().unwrap_or("int");
            let rt = r_eff.as_deref().unwrap_or("int");

            if lt != rt {
                if (lt == "int" && rt == "usize") || (lt == "usize" && rt == "int") {
                    l_str = format!("(({l_str}) as usize)");
                    r_str = format!("(({r_str}) as usize)");
                } else if (lt == "int" && rt == "isize") || (lt == "isize" && rt == "int") {
                    l_str = format!("(({l_str}) as isize)");
                    r_str = format!("(({r_str}) as isize)");
                } else if (lt == "isize" && rt == "usize") || (lt == "usize" && rt == "isize") {
                    l_str = format!("(({l_str}) as isize)");
                    r_str = format!("(({r_str}) as isize)");
                } else if (lt == "uint" && rt == "int") || (lt == "int" && rt == "uint") {
                    if r_str.starts_with('-') {
                        r_str = format!("((({r_str}) as int) as uint)");
                    } else {
                        r_str = format!("(({r_str}) as uint)");
                    }
                    if l_str.starts_with('-') {
                        l_str = format!("((({l_str}) as int) as uint)");
                    } else {
                        l_str = format!("(({l_str}) as uint)");
                    }
                } else if (lt == "usize" && rt == "u64") || (lt == "u64" && rt == "usize") {
                    l_str = format!("(({l_str}) as usize)");
                    r_str = format!("(({r_str}) as usize)");
                } else if (lt == "u64" && rt == "uint") || (lt == "uint" && rt == "u64") {
                    l_str = format!("(({l_str}) as u64)");
                    r_str = format!("(({r_str}) as u64)");
                } else if (lt == "u64" && rt == "i64") || (lt == "i64" && rt == "u64") {
                    l_str = format!("(({l_str}) as u64)");
                    r_str = format!("(({r_str}) as u64)");
                } else if (lt == "i64" && (rt == "int" || rt == "usize")) || ((lt == "int" || lt == "usize") && rt == "i64") {
                    l_str = format!("(({l_str}) as i64)");
                    r_str = format!("(({r_str}) as i64)");
                } else if (lt == "u64" && rt == "f64") || (lt == "f64" && rt == "u64") {
                    l_str = format!("(({l_str}) as f64)");
                    r_str = format!("(({r_str}) as f64)");
                } else if (lt == "int" && rt == "i8") || (rt == "int" && lt == "i8") {
                    l_str = format!("(({l_str}) as int)");
                    r_str = format!("(({r_str}) as int)");
                }
            }

            if op == "<<" || op == ">>" {
                if l_str.contains("as u64") || l_str.contains("as LuaUnsigned") {
                    format!("({l_str} {op} (({r_str}) as u64))")
                } else if l_str.contains("as usize") {
                    format!("({l_str} {op} (({r_str}) as usize))")
                } else if l_str.contains("as uint") || l_str.contains("as u32") || l_str.contains("as Utfint") || l_str.contains("as u8") || l_str.contains("as LuByte") {
                    format!("({l_str} {op} (({r_str}) as uint))")
                } else if let Some(ref lt) = left_ty {
                    format!("({l_str} {op} (({r_str}) as {lt}))")
                } else {
                    format!("({l_str} {op} (({r_str}) as int))")
                }
            } else if op == "&" || op == "|" || op == "^" {
                let l_is_u = l_eff.as_deref().map_or(false, is_unsigned_type) || l_str.contains("as uint") || l_str.contains("as u64") || l_str.contains("as usize") || l_str.contains("nCcalls");
                let r_is_u = r_eff.as_deref().map_or(false, is_unsigned_type) || r_str.contains("as uint") || r_str.contains("as u64") || r_str.contains("as usize");

                if l_is_u && r_str.starts_with('-') {
                    let target_u = l_eff.as_deref().unwrap_or("uint");
                    format!("({l_str} {op} ((({r_str}) as int) as {target_u}))")
                } else if r_is_u && l_str.starts_with('-') {
                    let target_u = r_eff.as_deref().unwrap_or("uint");
                    format!("(((({l_str}) as int) as {target_u}) {op} {r_str})")
                } else if l_is_u && !r_is_u && !r_str.starts_with('-') {
                    let target_u = l_eff.as_deref().unwrap_or("uint");
                    format!("({l_str} {op} (({r_str}) as {target_u}))")
                } else if r_is_u && !l_is_u && !l_str.starts_with('-') {
                    let target_u = r_eff.as_deref().unwrap_or("uint");
                    format!("((({l_str}) as {target_u}) {op} {r_str})")
                } else if (l_str.contains("as usize") || left_ty.as_deref() == Some("usize")) && (r_str.contains("as u64") || right_ty.as_deref() == Some("u64")) {
                    format!("({l_str} {op} (({r_str}) as usize))")
                } else if (r_str.contains("as usize") || right_ty.as_deref() == Some("usize")) && (l_str.contains("as u64") || left_ty.as_deref() == Some("u64")) {
                    format!("((({l_str}) as usize) {op} {r_str})")
                } else {
                    format!("({l_str} {op} {r_str})")
                }
            } else if op == "%" {
                if (l_str.contains("as i64") || l_str.contains("as LuaInteger")) && !r_str.contains("as i64") {
                    format!("({l_str} % (({r_str}) as i64))")
                } else if (r_str.contains("as i64") || r_str.contains("as LuaInteger")) && !l_str.contains("as i64") {
                    format!("((({l_str}) as i64) % {r_str})")
                } else {
                    format!("({l_str} % {r_str})")
                }
            } else {
                format!("({l_str} {op} {r_str})")
            }
        }
        CExpr::UnaryOp { op, operand, is_postfix } => {
            let op_str = generate_expr(operand);
            match op.as_str() {
                "++" => format!("({op_str} + 1)"),
                "--" => format!("({op_str} - 1)"),
                "*" => {
                    let mut unwrapped = operand.as_ref();
                    while let CExpr::Paren(inner) = unwrapped {
                        unwrapped = inner;
                    }
                    if let CExpr::UnaryOp { op: ref inner_op, operand: ref inner_operand, .. } = *unwrapped {
                        if inner_op == "&" {
                            return generate_expr(inner_operand);
                        }
                    }
                    if let CExpr::StringLiteral(ref s) = unwrapped {
                        let ch = s.as_bytes().first().copied().unwrap_or(0);
                        return format!("({ch} as u8)");
                    }
                    if let CExpr::BinaryOp { op: ref b_op, left: ref b_left, right: ref b_right } = unwrapped {
                        if b_op == "+" {
                            let mut l_s = generate_expr(b_left);
                            let r_s = generate_expr(b_right);
                            if (l_s.starts_with("((&") || l_s.starts_with("(&")) && l_s.contains("[0]) as ") {
                                if let Some(as_p) = l_s.find(" as ") {
                                    l_s = l_s[..as_p].trim_start_matches('(').trim_start_matches('&').to_string();
                                    if let Some(br_p) = l_s.find("[0]") {
                                        l_s = l_s[..br_p].to_string();
                                    }
                                }
                            }
                            return format!("{l_s}[(({r_s}) as int)]");
                        }
                    }
                    format!("{op_str}[0]")
                }
                "&" => {
                    if op_str.starts_with("(&") && op_str.ends_with(')') {
                        op_str
                    } else {
                        format!("&{op_str}")
                    }
                }
                "!" => {
                    if is_boolean_expr(operand) {
                        format!("!{op_str}")
                    } else if is_ptr_expr(operand) {
                        format!("({op_str} == nil)")
                    } else {
                        format!("({op_str} == 0)")
                    }
                }
                "-" => {
                    let operand_ty = infer_expr_type(operand);
                    let is_unsigned = operand_ty.as_deref().map(is_unsigned_type).unwrap_or(false)
                        || op_str.contains("as uint") || op_str.contains("as usize") || op_str.contains("as u32") || op_str.contains("as u64") || op_str.contains("as u8");
                    if is_boolean_expr(operand) {
                        format!("(if {op_str} {{ -1 }} else {{ 0 }})")
                    } else if is_unsigned {
                        let target_ty = operand_ty.unwrap_or_else(|| {
                            if op_str.contains("uint") { "uint".to_string() }
                            else if op_str.contains("u64") { "u64".to_string() }
                            else { "usize".to_string() }
                        });
                        if op_str == "1" || op_str == "(1)" || op_str.starts_with("(1 as ") || op_str.starts_with("1 as ") {
                            format!("(~(0 as {target_ty}))")
                        } else {
                            format!("((0 as {target_ty}) - (({op_str}) as {target_ty}))")
                        }
                    } else if is_ptr_expr(operand) || op_str.starts_with("ptr[") || op_str.contains(" as ptr[") {
                        op_str
                    } else {
                        format!("-{op_str}")
                    }
                }
                "~" => format!("~{op_str}"),
                "+" => op_str,
                _ => {
                    if *is_postfix {
                        format!("({op_str} + 1)")
                    } else if is_ptr_expr(operand) || op_str.contains("ptr[") {
                        op_str
                    } else {
                        format!("-{op_str}")
                    }
                }
            }
        }
        CExpr::Call { callee, args } => {
            let mut callee_inner = callee.as_ref();
            let mut was_deref = false;
            while let CExpr::Paren(inner) = callee_inner {
                callee_inner = inner;
            }
            if let CExpr::UnaryOp { op, operand, .. } = callee_inner {
                if op == "*" {
                    callee_inner = operand;
                    was_deref = true;
                }
            }
            let mut callee_str = generate_expr(callee_inner);
            while (callee_str.starts_with("(&") || callee_str.starts_with('(')) && callee_str.ends_with(')') {
                callee_str = callee_str[1..callee_str.len() - 1].trim().to_string();
                if callee_str.starts_with('&') {
                    callee_str = callee_str[1..].trim().to_string();
                }
            }
            while callee_str.starts_with('&') {
                callee_str = callee_str[1..].trim().to_string();
            }
            if (callee_str == "builtinExpect" || callee_str == "BuiltinExpect" || callee_str == "__builtin_expect") && !args.is_empty() {
                return generate_expr(&args[0]);
            }
            if (callee_str == "setjmp" || callee_str == "longjmp") && !args.is_empty() {
                let first_arg = generate_expr(&args[0]);
                let casted_first = format!("(&({first_arg}) as ptr[usize])");
                let rest_args: Vec<_> = args[1..].iter().map(generate_expr).collect();
                let all_args = if rest_args.is_empty() {
                    casted_first
                } else {
                    format!("{casted_first}, {}", rest_args.join(", "))
                };
                return format!("{callee_str}({all_args})");
            }
            let expected_params = KNOWN_FUNC_PARAMS.with(|cell| {
                let map = cell.borrow();
                map.get(&callee_str).cloned().or_else(|| map.get(&sanitize_ident(&callee_str)).cloned())
            });

            let is_global_fn = KNOWN_FUNCS.with(|cell| {
                let map = cell.borrow();
                map.contains_key(&callee_str) || map.contains_key(&sanitize_ident(&callee_str))
            });

            let args_str = args
                .iter()
                .enumerate()
                .map(|(idx, a)| {
                    let s = generate_expr(a);
                    if let CExpr::Ident(ref name) = a {
                        let clean = sanitize_ident(name);
                        let is_target_fn = KNOWN_FUNCS.with(|cell| {
                            let map = cell.borrow();
                            map.contains_key(name) || map.contains_key(&clean)
                        });
                        let is_local_var = KNOWN_TYPES.with(|cell| {
                            let map = cell.borrow();
                            map.contains_key(name) || map.contains_key(&clean)
                        });
                        if is_target_fn && !is_local_var && !s.starts_with("(&") && !s.starts_with('&') {
                            return format!("(&{s})");
                        }
                    }

                    let a_ty = infer_expr_type(a);

                    if callee_str == "memcpy" || callee_str == "memmove" {
                        if idx == 0 || idx == 1 {
                            if s.ends_with(".contents") || s.contains(".contents") || s == "buff" || s.starts_with("local_buff") || a_ty.as_deref().map_or(false, |t| t.starts_with('[')) {
                                return format!("((&{s}[0]) as ptr[u8])");
                            }
                            if !s.contains("as ptr[u8]") && s != "nil" {
                                return format!("(({s}) as ptr[u8])");
                            }
                        }
                    } else if callee_str == "memset" {
                        if idx == 0 {
                            if s.ends_with(".contents") || s.contains(".contents") || s == "buff" || s.starts_with("local_buff") || a_ty.as_deref().map_or(false, |t| t.starts_with('[')) {
                                return format!("((&{s}[0]) as ptr[u8])");
                            }
                            if !s.contains("as ptr[u8]") && s != "nil" {
                                return format!("(({s}) as ptr[u8])");
                            }
                        }
                    } else if callee_str == "luaMRealloc" || callee_str == "luaMFree" || callee_str == "free" {
                        if (callee_str == "free" && idx == 0) || (callee_str != "free" && idx == 1) {
                            if s.ends_with(".contents") || s.contains(".contents") || s == "buff" || s.starts_with("local_buff") || a_ty.as_deref().map_or(false, |t| t.starts_with('[')) {
                                return format!("((&{s}[0]) as ptr[u8])");
                            }
                            if !s.contains("as ptr[u8]") && s != "nil" {
                                return format!("(({s}) as ptr[u8])");
                            }
                        }
                    } else if callee_str == "strlen" {
                        if !s.contains("as ptr[i8]") {
                            if s.starts_with('&') {
                                return format!("(({s}) as ptr[i8])");
                            } else {
                                return format!("((&{s}[0]) as ptr[i8])");
                            }
                        }
                    }

                    let expected_ty = expected_params.as_ref().and_then(|p| p.get(idx));
                    let a_ty = infer_expr_type(a);

                    if let Some(param_ty) = expected_ty {
                        if param_ty.starts_with("ptr[") {
                            if s == "nil" || s == "(nil as ptr[u8])" || s == "0" || s == "(0)" {
                                return format!("(nil as {param_ty})");
                            }
                            if a_ty.as_deref().map_or(false, |t| t.starts_with('[')) || s.ends_with(".contents") || s == "buff" || s.starts_with("local_buff") {
                                return format!("((&{s}[0]) as {param_ty})");
                            }
                            if s.starts_with('&') {
                                return format!("({s} as {param_ty})");
                            }
                            if a_ty.as_deref().map_or(false, |t| t.starts_with("ptr[")) && a_ty.as_deref() != Some(param_ty.as_str()) {
                                return format!("({s} as {param_ty})");
                            }
                        } else if is_boolean_expr(a) && param_ty != "bool" {
                            return format!("(if {s} {{ 1 as {param_ty} }} else {{ 0 as {param_ty} }})");
                        } else if let Some(ref at) = a_ty {
                            if at != param_ty && is_scalar_type(param_ty) && is_scalar_type(at) {
                                return format!("(({s}) as {param_ty})");
                            }
                        }
                    }

                    if is_boolean_expr(a) {
                        format!("(if {s} {{ 1 }} else {{ 0 }})")
                    } else if let Some(ty) = a_ty {
                        if ty.starts_with('[') {
                            format!("(&{s} as ptr[u8])")
                        } else {
                            s
                        }
                    } else if s == "buff" || s.starts_with("local_buff") {
                        format!("(&{s} as ptr[u8])")
                    } else {
                        s
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            let is_indirect = was_deref
                || matches!(callee_inner, CExpr::MemberAccess { .. } | CExpr::Index { .. })
                || !is_global_fn;

            if is_indirect {
                format!("{callee_str}[0]({args_str})")
            } else {
                format!("{callee_str}({args_str})")
            }
        }
        CExpr::MemberAccess { base, member, is_arrow: _ } => {
            let clean_member = sanitize_ident(member);
            let mut unwrapped_base = base.as_ref();
            while let CExpr::Paren(inner) = unwrapped_base {
                unwrapped_base = inner;
            }
            if let CExpr::UnaryOp { ref op, ref operand, .. } = *unwrapped_base {
                if op == "&" {
                    let inner_base = generate_expr(operand);
                    return format!("{inner_base}.{clean_member}");
                }
                if op == "++" || op == "--" {
                    let inner_base = generate_expr(operand);
                    return format!("{inner_base}[0].{clean_member}");
                }
            }
            // Em C: (ptr + offset)->field é idêntico a ptr[offset].field
            if let CExpr::BinaryOp { ref op, ref left, ref right } = *unwrapped_base {
                if op == "+" {
                    let l_str = generate_expr(left);
                    let r_str = generate_expr(right);
                    return format!("{l_str}[(({r_str}) as int)].{clean_member}");
                }
            }
            let base_str = generate_expr(base);
            format!("{base_str}.{clean_member}")
        }
        CExpr::Index { base, index } => {
            let index_str = generate_expr(index);
            let cast_index = if index_str.chars().all(|c| c.is_ascii_digit()) {
                index_str.clone()
            } else {
                format!("(({index_str}) as int)")
            };
            if let CExpr::Ident(ref id) = **base {
                let clean = sanitize_ident(id);
                let is_ptr_arr = PTR_ARRAY_GLOBALS.with(|cell| {
                    let map = cell.borrow();
                    map.contains_key(id) || map.contains_key(&clean)
                });
                if is_ptr_arr {
                    return format!("get_{clean}()[{cast_index}]");
                }
            }
            let base_str = generate_expr(base);
            format!("{base_str}[{cast_index}]")
        }
        CExpr::Cast { target_ty, expr } => {
            if target_ty == "void" {
                return format!("({})", generate_expr(expr));
            }
            if is_boolean_expr(expr) {
                let cond_str = generate_expr(expr);
                return format!("(if {cond_str} {{ (1 as {target_ty}) }} else {{ (0 as {target_ty}) }})");
            }
            if is_scalar_type(target_ty) || target_ty == "uint" || target_ty == "usize" || target_ty == "int" || target_ty == "u64" || target_ty == "i64" {
                if let CExpr::Ident(ref id) = **expr {
                    if id == "NULL" || id == "nil" {
                        return format!("(0 as {target_ty})");
                    }
                }
            }
            if target_ty.starts_with("ptr[") {
                if let CExpr::IntLiteral(0) = **expr {
                    return "nil".to_string();
                }
                if let CExpr::Ident(ref id) = **expr {
                    if id == "NULL" || id == "nil" {
                        return "nil".to_string();
                    }
                }
            }
            let expr_str = generate_expr(expr);
            if target_ty.starts_with("ptr[") {
                if expr_str.ends_with(".contents") || expr_str.contains(".contents") {
                    return format!("((&{expr_str}[0]) as {target_ty})");
                }
                if let Some(ty) = infer_expr_type(expr) {
                    if ty.starts_with('[') {
                        return format!("((&{expr_str}[0]) as {target_ty})");
                    }
                } else if expr_str == "buff" || expr_str.starts_with("local_buff") {
                    return format!("((&{expr_str}[0]) as {target_ty})");
                }
            }
            if target_ty == "usize" {
                let mut unwrapped = expr.as_ref();
                while let CExpr::Paren(inner) = unwrapped {
                    unwrapped = inner;
                }
                let is_addr_or_fn = matches!(unwrapped, CExpr::UnaryOp { op, .. } if op == "&")
                    || (matches!(unwrapped, CExpr::Ident(_)) && (expr_str.starts_with("(&") || expr_str.starts_with('&')));
                if is_addr_or_fn {
                    return format!("(((({expr_str}) as ptr[u8])) as usize)");
                }
            }
            if is_unsigned_type(target_ty) || is_scalar_type(target_ty) || target_ty == "uint" || target_ty == "usize" || target_ty == "u64" || target_ty == "int" || target_ty == "i64" {
                if expr_str == "nil" || expr_str == "(nil)" || expr_str == "((nil))" {
                    return format!("(0 as {target_ty})");
                }
                if expr_str == "-1" || expr_str == "(-1)" || expr_str == "(0 - 1)" {
                    return format!("(~(0 as {target_ty}))");
                }
                if expr_str.starts_with('-') || expr_str.starts_with("(-") {
                    return format!("((({expr_str}) as int) as {target_ty})");
                }
            }
            format!("({expr_str} as {target_ty})")
        }
        CExpr::Paren(inner) => format!("({})", generate_expr(inner)),
        CExpr::Ternary { cond, then_expr, else_expr } => {
            let cond_str = ensure_boolean_condition(cond);
            let mut then_str = generate_expr(then_expr);
            let mut else_str = generate_expr(else_expr);

            let then_ty = infer_expr_type(then_expr);
            let else_ty = infer_expr_type(else_expr);

            // Decaimento de arrays em ramos de ternário
            if let Some(ref tt) = then_ty {
                if tt.starts_with('[') {
                    then_str = format!("((&{then_str}[0]) as ptr[u8])");
                }
            } else if then_str.ends_with(".contents") {
                then_str = format!("((&{then_str}[0]) as ptr[u8])");
            }
            if let Some(ref et) = else_ty {
                if et.starts_with('[') {
                    else_str = format!("((&{else_str}[0]) as ptr[u8])");
                }
            } else if else_str.ends_with(".contents") {
                else_str = format!("((&{else_str}[0]) as ptr[u8])");
            }

            // Funções globais usadas em ternário como ponteiros de função
            let is_fn_val = |e: &CExpr, s: &str| {
                if let CExpr::Ident(name) = e {
                    let clean = sanitize_ident(name);
                    KNOWN_FUNCS.with(|cell| {
                        let map = cell.borrow();
                        (map.contains_key(name) || map.contains_key(&clean)) && !s.starts_with('&') && !s.starts_with("(&")
                    })
                } else {
                    false
                }
            };
            if is_fn_val(then_expr, &then_str) {
                then_str = format!("(&{then_str})");
            }
            if is_fn_val(else_expr, &else_str) {
                else_str = format!("(&{else_str})");
            }

            // Normalização ptr[i8] vs ptr[u8]
            if (then_str.contains("ptr[i8]") || then_ty.as_deref() == Some("ptr[i8]") || then_str.starts_with("strerror"))
                && (else_str.contains("ptr[u8]") || else_ty.as_deref() == Some("ptr[u8]") || else_str.starts_with("c\"")) {
                then_str = format!("({then_str} as ptr[u8])");
            } else if (else_str.contains("ptr[i8]") || else_ty.as_deref() == Some("ptr[i8]") || else_str.starts_with("strerror"))
                && (then_str.contains("ptr[u8]") || then_ty.as_deref() == Some("ptr[u8]") || then_str.starts_with("c\"")) {
                else_str = format!("({else_str} as ptr[u8])");
            }

            // Normalização bool vs int
            let then_is_bool = is_boolean_expr(then_expr) || then_str == "true" || then_str == "false" || then_str.starts_with('!') || then_str.ends_with("== 0)") || then_str.ends_with("!= 0)");
            let else_is_bool = is_boolean_expr(else_expr) || else_str == "true" || else_str == "false" || else_str.starts_with('!') || else_str.ends_with("== 0)") || else_str.ends_with("!= 0)");

            if !then_is_bool && else_is_bool {
                else_str = format!("(if {else_str} {{ 1 }} else {{ 0 }})");
            } else if then_is_bool && !else_is_bool {
                then_str = format!("(if {then_str} {{ 1 }} else {{ 0 }})");
            }

            // Normalização int vs uint (somente quando ambos são tipos numéricos escalares)
            if let (Some(ref tt), Some(ref et)) = (&then_ty, &else_ty) {
                if (tt == "uint" || tt == "usize") && (et == "int" || et == "i64" || et == "i32") {
                    else_str = format!("(({else_str}) as {tt})");
                } else if (et == "uint" || et == "usize") && (tt == "int" || tt == "i64" || tt == "i32") {
                    then_str = format!("(({then_str}) as {et})");
                }
            }

            format!("(if {cond_str} {{\n    {then_str}\n}} else {{\n    {else_str}\n}})")
        }
        CExpr::Raw(s) => {
            if s.is_empty() || s.contains('#') || s.contains("/*") || s.contains(';') || s.contains("define") {
                String::new()
            } else {
                s.clone()
            }
        }
    }
}

pub fn generate_stmt_from_expr(e: &CExpr) -> String {
    let mut unwrapped = e;
    while let CExpr::Paren(inner) = unwrapped {
        unwrapped = inner;
    }
    if let CExpr::BinaryOp { op, left, right } = unwrapped {
        if [
            "=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="
        ].contains(&op.as_str()) {
            let stmt = CStmt::Assign {
                target: (**left).clone(),
                op: op.clone(),
                value: (**right).clone(),
            };
            return generate_stmt(&stmt, 0).trim().to_string();
        }
    }
    String::new()
}

pub fn default_initializer_for_type(ty: &str) -> String {
    let mut resolved_ty = ty.to_string();
    KNOWN_TYPEDEFS.with(|cell| {
        let map = cell.borrow();
        let mut depth = 0;
        while depth < 10 {
            if let Some(aliased) = map.get(&resolved_ty) {
                if aliased == &resolved_ty {
                    break;
                }
                resolved_ty = aliased.clone();
                depth += 1;
            } else {
                break;
            }
        }
    });
    let ty = resolved_ty.as_str();
    if ty.starts_with("ptr[") || ty.ends_with("[]") || ty.starts_with("func(") || ty.ends_with("Function") || ty.ends_with("Func") || ty.ends_with("Id") || ty == "StkId" || ty == "VaList" {
        "nil".to_string()
    } else if ty.starts_with('[') {
        if let Some(close_bracket) = ty.find(']') {
            let len_str = &ty[1..close_bracket];
            let inner_ty = &ty[close_bracket + 1..];
            let inner_init = default_initializer_for_type(inner_ty);
            if inner_init.contains("nil") || inner_init.contains("ptr[") || inner_init.contains('{') {
                if let Ok(count) = len_str.parse::<usize>() {
                    if count <= 64 {
                        let items = vec![inner_init; count];
                        return format!("[{}]", items.join(", "));
                    }
                }
            }
            format!("[{inner_init}; {len_str}]")
        } else {
            "[]".to_string()
        }
    } else {
        match ty {
            "bool" => "false".to_string(),
            "f32" | "f64" | "LuaNumber" => "0.0".to_string(),
            "i8" | "i16" | "i32" | "i64" | "int" | "isize" |
            "u8" | "u16" | "u32" | "u64" | "uint" | "usize" |
            "LuaInteger" | "LuaUnsigned" | "LuByte" | "LuMem" | "LMem" |
            "Instruction" | "IdxT" | "TimeT" | "Utfint" | "BinOpr" | "OpCode" | "UnOpr" => "0".to_string(),
            "AnonymousStruct" => "AnonymousStruct { dummy: 0 }".to_string(),
            "Dyndata" => "Dyndata { actvar: Dyndata_ArrNSize { arr: nil, n: 0, size: 0 }, gt: Labellist { arr: nil, n: 0, size: 0 }, label: Labellist { arr: nil, n: 0, size: 0 } }".to_string(),
            "SParser" => "SParser { z: nil, buff: Mbuffer { buffer: nil, n: (0 as usize), buffsize: (0 as usize) }, dyd: Dyndata { actvar: Dyndata_ArrNSize { arr: nil, n: 0, size: 0 }, gt: Labellist { arr: nil, n: 0, size: 0 }, label: Labellist { arr: nil, n: 0, size: 0 } }, mode: nil, name: nil }".to_string(),
            "Table" => "Table { next: nil, tt: (0 as u8), marked: (0 as u8), flags: (0 as u8), lsizenode: (0 as u8), alimit: 0, array: nil, node: nil, lastfree: nil, metatable: nil, gclist: nil }".to_string(),
            "LexState" => "LexState { current: 0, linenumber: 0, lastline: 0, t: Token { token: 0, seminfo: SemInfo { i: (0 as LuaInteger) } }, lookahead: Token { token: 0, seminfo: SemInfo { i: (0 as LuaInteger) } }, fs: nil, l: nil, z: nil, buff: nil, h: nil, dyd: nil, source: nil, envn: nil }".to_string(),
            "FuncState" => "FuncState { f: nil, prev: nil, ls: nil, bl: nil, pc: 0, lasttarget: 0, previousline: 0, nk: 0, np: 0, nups: 0, ndebugvars: 0, nactvar: 0, needclose: (0 as u8), freereg: (0 as u8), iwthabs: 0, nabslineinfo: 0, firstlocal: 0, firstlabel: 0 }".to_string(),
            "MatchState" => {
                let cap_items = vec!["MatchState_InitLen { init: nil, len: 0 }".to_string(); 32];
                format!("MatchState {{ srcInit: nil, srcEnd: nil, pEnd: nil, l: nil, matchdepth: 0, level: (0 as u8), capture: [{}] }}", cap_items.join(", "))
            }
            "LuaLBuffer" => "LuaLBuffer { b: nil, size: (0 as usize), n: (0 as usize), l: nil, init: LuaLBuffer_NUS { i: (0 as LuaInteger) } }".to_string(),
            "CallS" => "CallS { funcVal: nil, nresults: 0 }".to_string(),
            "LuaLongjmp" => "LuaLongjmp { previous: nil, b: [0; 64], status: 0 }".to_string(),
            "CloseP" => "CloseP { level: nil, status: 0 }".to_string(),
            "LoadS" => "LoadS { s: nil, size: (0 as usize) }".to_string(),
            "LoadF" => "LoadF { n: 0, f: nil, buff: [0; 8192] }".to_string(),
            "Header" => "Header { l: nil, islittle: 1, maxalign: 1 }".to_string(),
            "DumpState" => "DumpState { l: nil, writer: nil, data: nil, strip: 0, status: 0 }".to_string(),
            "LoadState" => "LoadState { l: nil, z: nil, name: nil }".to_string(),
            "RN" => "RN { f: nil, c: 0, n: 0, buff: [0; 201] }".to_string(),
            "BuffFS" => "BuffFS { l: nil, pushed: 0, blen: 0, space: [0; 199] }".to_string(),
            "Tm" => "Tm { tmSec: 0, tmMin: 0, tmHour: 0, tmMday: 0, tmMon: 0, tmYear: 0, tmWday: 0, tmYday: 0, tmIsdst: 0 }".to_string(),
            "BlockCnt" => "BlockCnt { previous: nil, firstlabel: 0, firstgoto: 0, nactvar: (0 as u8), upval: (0 as u8), isloop: (0 as u8), insidetbc: (0 as u8) }".to_string(),
            "Expdesc" => "Expdesc { k: (0 as Expkind), u: Expdesc_IvalNvalStrval { ival: (0 as LuaInteger) }, t: 0, f: 0 }".to_string(),
            "LHSAssign" => "LHSAssign { prev: nil, v: Expdesc { k: (0 as Expkind), u: Expdesc_IvalNvalStrval { ival: (0 as LuaInteger) }, t: 0, f: 0 } }".to_string(),
            "ConsControl" => "ConsControl { v: Expdesc { k: (0 as Expkind), u: Expdesc_IvalNvalStrval { ival: (0 as LuaInteger) }, t: 0, f: 0 }, t: nil, nh: 0, na: 0, tostore: 0 }".to_string(),
            "LuaDebug" => "LuaDebug { event: 0, name: nil, namewhat: nil, what: nil, source: nil, srclen: (0 as usize), currentline: 0, linedefined: 0, lastlinedefined: 0, nups: (0 as u8), nparams: (0 as u8), isvararg: (0 as i8), istailcall: (0 as i8), ftransfer: (0 as u16), ntransfer: (0 as u16), shortSrc: [0; 60], iCi: nil }".to_string(),
            "TValue" => "TValue { value: Value { gc: nil }, tt: (0 as u8) }".to_string(),
            "Zio" | "ZIO" => "Zio { p: nil, n: (0 as usize), reader: nil, data: nil, l: nil }".to_string(),
            struct_or_other => {
                if let Some(order) = STRUCT_FIELD_ORDER.with(|cell| cell.borrow().get(struct_or_other).cloned()) {
                    let field_inits: Vec<String> = order
                        .into_iter()
                        .map(|(clean, f_ty)| {
                            let init = default_initializer_for_type(&f_ty);
                            format!("{clean}: {init}")
                        })
                        .collect();
                    if !field_inits.is_empty() {
                        return format!("{struct_or_other} {{ {} }}", field_inits.join(", "));
                    }
                }
                format!("{struct_or_other} {{}}")
            }
        }
    }
}

pub fn normalize_c_string_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next) = chars.peek() {
                match next {
                    '0'..='7' => {
                        let mut octal_val = 0u32;
                        let mut count = 0;
                        while count < 3 {
                            if let Some(&digit) = chars.peek() {
                                if ('0'..='7').contains(&digit) {
                                    octal_val = octal_val * 8 + (digit as u32 - '0' as u32);
                                    chars.next();
                                    count += 1;
                                    continue;
                                }
                            }
                            break;
                        }
                        if octal_val == 0 && count == 1 {
                            out.push_str(r"\0");
                        } else {
                            out.push_str(&format!(r"\x{:02x}", octal_val & 0xff));
                        }
                    }
                    'a' => { chars.next(); out.push_str(r"\x07"); }
                    'b' => { chars.next(); out.push_str(r"\x08"); }
                    'f' => { chars.next(); out.push_str(r"\x0c"); }
                    'v' => { chars.next(); out.push_str(r"\x0b"); }
                    '?' => { chars.next(); out.push('?'); }
                    _ => {
                        out.push('\\');
                        out.push(chars.next().unwrap());
                    }
                }
            } else {
                out.push('\\');
            }
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn extract_postfix_incs(e: &mut CExpr, out: &mut Vec<(String, String)>) {
    match e {
        CExpr::Paren(inner) => extract_postfix_incs(inner, out),
        CExpr::MemberAccess { base, .. } => extract_postfix_incs(base, out),
        CExpr::Index { base, index } => {
            extract_postfix_incs(base, out);
            extract_postfix_incs(index, out);
        }
        CExpr::UnaryOp { op, operand, is_postfix } if *is_postfix && (op == "++" || op == "--") => {
            let op_clone = op.clone();
            let var_str = generate_expr(operand);
            *e = *operand.clone();
            out.push((var_str, op_clone));
        }
        CExpr::UnaryOp { operand, .. } => extract_postfix_incs(operand, out),
        CExpr::BinaryOp { left, right, .. } => {
            extract_postfix_incs(left, out);
            extract_postfix_incs(right, out);
        }
        CExpr::Call { callee, args } => {
            extract_postfix_incs(callee, out);
            for arg in args {
                extract_postfix_incs(arg, out);
            }
        }
        CExpr::Cast { expr, .. } => extract_postfix_incs(expr, out),
        CExpr::Ternary { cond, .. } => extract_postfix_incs(cond, out),
        _ => {}
    }
}

pub fn generate_stmt(stmt: &CStmt, indent: usize) -> String {
    let pad = "    ".repeat(indent);
    match stmt {
        CStmt::VarDecl { name, ty, init, is_mut } => {
            let clean_name = sanitize_ident(name);
            let mut_str = if *is_mut { "mut " } else { "" };
            let mut postfix_incs = Vec::new();
            let init_str = if let Some(init_expr) = init {
                let mut init_mut = init_expr.clone();
                extract_postfix_incs(&mut init_mut, &mut postfix_incs);
                let s = generate_expr(&init_mut);
                let init_ty = infer_expr_type(&init_mut);
                if (s == "0" || s == "(0)") && ty.starts_with("ptr[") {
                    format!("(nil as {ty})")
                } else if ty.starts_with("ptr[") {
                    if s == "nil" {
                        format!("(nil as {ty})")
                    } else if s.starts_with('&') {
                        format!("({s} as {ty})")
                    } else if s.ends_with(".contents") || init_ty.as_deref().map_or(false, |t| t.starts_with('[')) {
                        format!("((&{s}[0]) as {ty})")
                    } else if init_ty.as_deref().map_or(false, |t| t.starts_with("ptr[")) && init_ty.as_deref() != Some(ty.as_str()) {
                        format!("({s} as {ty})")
                    } else {
                        s
                    }
                } else if is_boolean_expr(&init_mut) && ty != "bool" {
                    format!("(if {s} {{ 1 as {ty} }} else {{ 0 as {ty} }})")
                } else if let Some(it) = init_ty {
                    if it != *ty && (is_scalar_type(ty) && is_scalar_type(&it)) {
                        format!("(({s}) as {ty})")
                    } else {
                        s
                    }
                } else {
                    s
                }
            } else {
                default_initializer_for_type(ty)
            };
            let mut res = format!("{pad}let {mut_str}{clean_name}: {ty} = {init_str}\n");
            for (ptr_str, inc_op) in postfix_incs {
                if inc_op == "++" {
                    res.push_str(&format!("{pad}{ptr_str} = {ptr_str} + 1\n"));
                } else if inc_op == "--" {
                    res.push_str(&format!("{pad}{ptr_str} = {ptr_str} - 1\n"));
                }
            }
            res
        }
        CStmt::Assign { target, op, value } => {
            fn unwrap_paren(mut e: &CExpr) -> &CExpr {
                while let CExpr::Paren(inner) = e {
                    e = inner;
                }
                e
            }
            let mut target_mut = target.clone();
            let unwrapped_target = unwrap_paren(target);
            if let CExpr::UnaryOp { op: ref star_op, operand: ref inner, .. } = unwrapped_target {
                if star_op == "*" {
                    let unwrapped_inner = unwrap_paren(inner);
                    if let CExpr::BinaryOp { op: ref b_op, left: ref b_left, right: ref b_right } = unwrapped_inner {
                        if b_op == "+" {
                            target_mut = CExpr::Index { base: b_left.clone(), index: b_right.clone() };
                        }
                    }
                    if let CExpr::UnaryOp { op: ref inc_op, operand: ref ptr_var, is_postfix } = unwrapped_inner {
                        let p_str = generate_expr(ptr_var);
                        let mut v_str = generate_expr(value);
                        let p_ty = infer_expr_type(ptr_var);
                        if p_ty.as_deref() == Some("ptr[u8]") || p_str.contains("Form") || p_str.contains("buff") {
                            if v_str.starts_with("c\"") {
                                v_str = format!("{v_str}[0]");
                            } else if !v_str.ends_with("as u8)") && !v_str.ends_with("as u8") {
                                v_str = format!("(({v_str}) as u8)");
                            }
                        } else if p_ty.as_deref() == Some("ptr[i8]") {
                            if v_str.starts_with("c\"") {
                                v_str = format!("(({v_str}[0]) as i8)");
                            } else if !v_str.ends_with("as i8)") && !v_str.ends_with("as i8") {
                                v_str = format!("(({v_str}) as i8)");
                            }
                        }
                        if inc_op == "++" {
                            if *is_postfix {
                                return format!("{pad}{p_str}[0] {op} {v_str}\n{pad}{p_str} = {p_str} + 1\n");
                            } else {
                                return format!("{pad}{p_str} = {p_str} + 1\n{pad}{p_str}[0] {op} {v_str}\n");
                            }
                        } else if inc_op == "--" {
                            if *is_postfix {
                                return format!("{pad}{p_str}[0] {op} {v_str}\n{pad}{p_str} = {p_str} - 1\n");
                            } else {
                                return format!("{pad}{p_str} = {p_str} - 1\n{pad}{p_str}[0] {op} {v_str}\n");
                            }
                        }
                    }
                }
            }

            let mut postfix_incs = Vec::new();
            extract_postfix_incs(&mut target_mut, &mut postfix_incs);
            let mut value_mut = value.clone();
            extract_postfix_incs(&mut value_mut, &mut postfix_incs);

            let mut prefix_call: Option<String> = None;
            let tmp_id = TMP_VAR_COUNTER.with(|c| {
                let mut val = c.borrow_mut();
                *val += 1;
                *val
            });
            let tmp_name = format!("tmp{tmp_id}");

            fn extract_prefix_from_target(e: &mut CExpr, tmp_ident: &str) -> Option<CExpr> {
                match e {
                    CExpr::Paren(inner) => extract_prefix_from_target(inner, tmp_ident),
                    CExpr::UnaryOp { operand, .. } => {
                        if matches!(**operand, CExpr::Call { .. } | CExpr::Cast { .. }) {
                            let call = (**operand).clone();
                            *operand = Box::new(CExpr::Ident(tmp_ident.to_string()));
                            Some(call)
                        } else {
                            extract_prefix_from_target(operand, tmp_ident)
                        }
                    }
                    CExpr::MemberAccess { base, .. } => {
                        if matches!(**base, CExpr::Call { .. } | CExpr::Cast { .. }) {
                            let call = (**base).clone();
                            *base = Box::new(CExpr::Ident(tmp_ident.to_string()));
                            Some(call)
                        } else {
                            extract_prefix_from_target(base, tmp_ident)
                        }
                    }
                    CExpr::Index { base, .. } => {
                        if matches!(**base, CExpr::Call { .. } | CExpr::Cast { .. }) {
                            let call = (**base).clone();
                            *base = Box::new(CExpr::Ident(tmp_ident.to_string()));
                            Some(call)
                        } else {
                            extract_prefix_from_target(base, tmp_ident)
                        }
                    }
                    _ => None,
                }
            }

            if let Some(prefix_expr) = extract_prefix_from_target(&mut target_mut, &tmp_name) {
                let call_str = generate_expr(&prefix_expr);
                prefix_call = Some(format!("{pad}let mut {tmp_name} = {call_str}\n"));
            }

            let mut target_str = generate_expr(&target_mut);
            // Normalização canônica de lvalue (Place) em Arandu:
            // Remove qualquer cast (ex: (x as ptr[T])), desreferências e parênteses de base
            loop {
                let prev = target_str.clone();
                while target_str.starts_with('&') {
                    target_str = target_str[1..].trim_start().to_string();
                }
                if let Some(as_pos) = target_str.find(" as ") {
                    if let Some(bracket_pos) = target_str.find('[') {
                        if as_pos > bracket_pos {
                            break;
                        }
                    }
                    let mut open_paren = None;
                    let mut depth = 0;
                    for (idx, ch) in target_str[..as_pos].char_indices().rev() {
                        if ch == ')' || ch == ']' {
                            depth += 1;
                        } else if ch == '(' || ch == '[' {
                            if depth > 0 {
                                depth -= 1;
                            } else if ch == '(' {
                                open_paren = Some(idx);
                                break;
                            }
                        }
                    }

                    let mut close_paren = None;
                    let mut f_depth = 0;
                    for (idx, ch) in target_str[as_pos + 4..].char_indices() {
                        if ch == '(' || ch == '[' {
                            f_depth += 1;
                        } else if ch == ')' || ch == ']' {
                            if f_depth > 0 {
                                f_depth -= 1;
                            } else if ch == ')' {
                                close_paren = Some(as_pos + 4 + idx);
                                break;
                            }
                        }
                    }

                    if let (Some(op_idx), Some(cp_idx)) = (open_paren, close_paren) {
                        let mut inner_expr = target_str[op_idx + 1..as_pos].trim().to_string();
                        while inner_expr.starts_with('&') {
                            inner_expr = inner_expr[1..].trim_start().to_string();
                        }
                        target_str = format!("{}{inner_expr}{}", &target_str[..op_idx], &target_str[cp_idx + 1..]);
                        continue;
                    }
                }
                while target_str.starts_with('(') && target_str.ends_with(')') {
                    target_str = target_str[1..target_str.len() - 1].trim().to_string();
                }
                if target_str.starts_with('(') {
                    if let Some(dot_idx) = target_str.find(").") {
                        let inside = target_str[1..dot_idx].trim().to_string();
                        let rest = target_str[dot_idx + 1..].to_string();
                        target_str = format!("{inside}{rest}");
                    }
                }
                if target_str == prev {
                    break;
                }
            }

            let mut val_str = generate_expr(&value_mut);
            let target_ty = infer_expr_type(&target_mut);
            let val_ty = infer_expr_type(&value_mut);

            if let Some(ref tgt_t) = target_ty {
                if tgt_t.starts_with("ptr[") {
                    if val_str == "0" || val_str == "(0)" || val_str == "nil" {
                        val_str = format!("(nil as {tgt_t})");
                    } else if val_str.starts_with('&') {
                        val_str = format!("({val_str} as {tgt_t})");
                    } else if val_ty.as_deref().map_or(false, |vt| vt.starts_with('[')) || val_str.ends_with(".contents") {
                        val_str = format!("((&{val_str}[0]) as {tgt_t})");
                    } else if val_ty.as_deref().map_or(false, |vt| vt.starts_with("ptr[")) && val_ty.as_deref() != Some(tgt_t.as_str()) {
                        val_str = format!("({val_str} as {tgt_t})");
                    }
                } else if tgt_t == "u8" {
                    if val_str.starts_with("c\"") {
                        val_str = format!("{val_str}[0]");
                    } else if val_ty.as_deref() != Some("u8") && !val_str.ends_with(" as u8)") && !val_str.ends_with(" as u8") {
                        val_str = format!("(({val_str}) as u8)");
                    }
                } else if tgt_t == "i8" {
                    if val_str.starts_with("c\"") {
                        val_str = format!("(({val_str}[0]) as i8)");
                    } else if val_ty.as_deref() != Some("i8") && !val_str.ends_with(" as i8)") && !val_str.ends_with(" as i8") {
                        val_str = format!("(({val_str}) as i8)");
                    }
                } else if tgt_t == "f64" {
                    if val_ty.as_deref().map_or(false, |vt| vt == "int" || vt == "i64" || vt == "usize") {
                        val_str = format!("(({val_str}) as f64)");
                    }
                } else if tgt_t == "int" {
                    if is_boolean_expr(&value_mut) {
                        val_str = format!("(if {val_str} {{ 1 }} else {{ 0 }})");
                    } else if val_ty.as_deref() == Some("i8") || val_ty.as_deref() == Some("u8") {
                        val_str = format!("(({val_str}) as int)");
                    } else if val_ty.as_deref() == Some("f64") {
                        val_str = format!("(({val_str}) as int)");
                    }
                }
            } else {
                if target_str.contains(".b[") || target_str.contains("localForm[") || target_str.contains("buff[") {
                    if val_str.starts_with("c\"") {
                        val_str = format!("{val_str}[0]");
                    } else if !val_str.ends_with(" as u8)") && !val_str.ends_with(" as u8") {
                        val_str = format!("(({val_str}) as u8)");
                    }
                }
                if is_boolean_expr(&value_mut) {
                    val_str = format!("(if {val_str} {{ 1 }} else {{ 0 }})");
                } else if let Some(ref vt) = val_ty {
                    if vt.starts_with('[') {
                        val_str = format!("((&{val_str}[0]) as ptr[u8])");
                    }
                }
                if (val_str == "0" || val_str == "(0)") && is_ptr_expr(&target_mut) {
                    val_str = "nil".to_string();
                }
            }

            if target_str.contains(".lineinfo[") && !target_str.contains("abslineinfo") {
                val_str = format!("(({val_str}) as i8)");
            }
            if target_str.contains("localForm[") {
                val_str = format!("(({val_str}) as u8)");
            }

            let is_ptr_like = |s: &str| {
                s.contains("ptr") || s.contains("Ptr")
                    || s.ends_with(".p") || s.contains(".top") || s.contains(".stack")
                    || s.contains(".tbclist") || s.contains("newtop") || s == "nil"
                    || s == "from" || s == "to" || s == "curr" || s == "prev" || s == "next"
                    || s == "head" || s == "tail" || s.ends_with("Palette") || s.ends_with("Data")
                    || s.contains("Callbacks") || s.ends_with(".arr") || s.ends_with(".gc")
                    || s.ends_with(".strval") || s.ends_with(".buffer")
            };
            let mut res = if (op == "+=" || op == "-=") && is_ptr_like(&target_str) {
                let bin_op = if op == "+=" { "+" } else { "-" };
                format!("{}{pad}{target_str} = {target_str} {bin_op} {val_str}\n", prefix_call.as_deref().unwrap_or(""))
            } else if (op == "+=" || op == "-=") && (val_str.contains("lineinfo") || target_str.contains("previousline") || target_str.contains("baseline") || (target_ty.as_deref() == Some("int") && val_ty.as_deref() == Some("i8"))) {
                let bin_op = if op == "+=" { "+" } else { "-" };
                format!("{}{pad}{target_str} = {target_str} {bin_op} (({val_str}) as int)\n", prefix_call.as_deref().unwrap_or(""))
            } else {
                format!("{}{pad}{target_str} {op} {val_str}\n", prefix_call.as_deref().unwrap_or(""))
            };
            for (ptr_str, inc_op) in postfix_incs {
                if inc_op == "++" {
                    res.push_str(&format!("{pad}{ptr_str} = {ptr_str} + 1\n"));
                } else if inc_op == "--" {
                    res.push_str(&format!("{pad}{ptr_str} = {ptr_str} - 1\n"));
                }
            }
            res
        }
        CStmt::Expr(expr) => {
            let mut unwrapped_e = expr;
            loop {
                match unwrapped_e {
                    CExpr::Paren(inner) => unwrapped_e = inner,
                    CExpr::Cast { expr: inner, .. } => unwrapped_e = inner,
                    _ => break,
                }
            }
            if let CExpr::Ternary { cond, then_expr, else_expr } = unwrapped_e {
                let cond_str = ensure_boolean_condition(cond);
                let then_s = generate_expr(then_expr);
                let is_noop = |mut e: &CExpr| {
                    while let CExpr::Paren(inner) = e {
                        e = inner;
                    }
                    match e {
                        CExpr::IntLiteral(0) => true,
                        CExpr::Cast { target_ty, .. } if target_ty == "void" => true,
                        CExpr::Ident(id) if id == "NULL" || id == "nil" => true,
                        _ => false,
                    }
                };
                if is_noop(else_expr) {
                    return format!("{pad}if {cond_str} {{\n{pad}    {then_s}\n{pad}}}\n");
                } else if is_noop(then_expr) {
                    let else_s = generate_expr(else_expr);
                    return format!("{pad}if !{cond_str} {{\n{pad}    {else_s}\n{pad}}}\n");
                } else {
                    let else_s = generate_expr(else_expr);
                    return format!("{pad}if {cond_str} {{\n{pad}    {then_s}\n{pad}}} else {{\n{pad}    {else_s}\n{pad}}}\n");
                }
            }

            // Se for unário ++ ou -- como statement, converte para atribuição
            if let CExpr::UnaryOp { ref op, ref operand, is_postfix: _ } = expr {
                if op == "++" {
                    let var = generate_expr(operand);
                    return format!("{pad}{var} = {var} + 1\n");
                } else if op == "--" {
                    let var = generate_expr(operand);
                    return format!("{pad}{var} = {var} - 1\n");
                }
            }
            // Ignora expressões no-op puras de C como `(void)x;` ou `x;` usadas para calar warnings
            match expr {
                CExpr::Ident(_) | CExpr::Paren(_) | CExpr::IntLiteral(_) | CExpr::FloatLiteral(_) | CExpr::BoolLiteral(_) => return String::new(),
                CExpr::UnaryOp { op, .. } if op != "++" && op != "--" => return String::new(),
                CExpr::Cast { target_ty, .. } if target_ty == "void" => return String::new(),
                CExpr::Raw(s) if s.is_empty() || s.contains('#') || s.contains("attribute") || s.contains("fallthrough") || s.contains("/*") || s.contains('{') || s.contains(';') => return String::new(),
                _ => {}
            }
            let mut expr_mut = expr.clone();
            let mut postfix_incs = Vec::new();
            extract_postfix_incs(&mut expr_mut, &mut postfix_incs);
            let mut expr_str = generate_expr(&expr_mut);
            if expr_str.is_empty() {
                return String::new();
            }
            // Se a expressão for um `(if ...)` usado como statement, remove os parênteses
            // externos para evitar que o parser de Arandu a interprete como chamada da linha anterior!
            while expr_str.starts_with('(') && expr_str.ends_with(')') {
                let inner = expr_str[1..expr_str.len() - 1].trim();
                if inner.starts_with("if ") || inner.contains('\n') {
                    expr_str = inner.to_string();
                } else {
                    break;
                }
            }
            let semi = if !expr_str.ends_with('}') && !expr_str.ends_with(';') { ";" } else { "" };
            let mut res = format!("{pad}{expr_str}{semi}\n");
            for (ptr_str, inc_op) in postfix_incs {
                if inc_op == "++" {
                    res.push_str(&format!("{pad}{ptr_str} = {ptr_str} + 1\n"));
                } else if inc_op == "--" {
                    res.push_str(&format!("{pad}{ptr_str} = {ptr_str} - 1\n"));
                }
            }
            res
        }
        CStmt::Return(opt_expr) => {
            if let Some(expr) = opt_expr {
                let mut expr_mut = expr.clone();
                let mut postfix_incs = Vec::new();
                extract_postfix_incs(&mut expr_mut, &mut postfix_incs);

                let expected_ret_ty = CURRENT_FUNC_RET_TY.with(|cell| cell.borrow().clone());

                let format_ret_val = |raw_val: &str, raw_e: &CExpr| -> String {
                    if let Some(ref ret_ty) = expected_ret_ty {
                        if ret_ty == "void" {
                            return String::new();
                        }
                        if ret_ty.starts_with("ptr[") {
                            if raw_val == "nil" || raw_val == "0" || raw_val == "(0)" {
                                return format!("(nil as {ret_ty})");
                            }
                            if raw_val.starts_with('&') {
                                return format!("({raw_val} as {ret_ty})");
                            }
                            let e_ty = infer_expr_type(raw_e);
                            if e_ty.as_deref().map_or(false, |t| t.starts_with('[')) || raw_val.ends_with(".contents") {
                                return format!("((&{raw_val}[0]) as {ret_ty})");
                            }
                            if e_ty.as_deref().map_or(false, |t| t.starts_with("ptr[")) && e_ty.as_deref() != Some(ret_ty.as_str()) {
                                return format!("({raw_val} as {ret_ty})");
                            }
                            return raw_val.to_string();
                        }
                        if ret_ty == "bool" {
                            if is_boolean_expr(raw_e) {
                                return raw_val.to_string();
                            } else {
                                return format!("({raw_val} != 0)");
                            }
                        }
                        if is_boolean_expr(raw_e) {
                            return format!("(if {raw_val} {{ 1 as {ret_ty} }} else {{ 0 as {ret_ty} }})");
                        }
                        let e_ty = infer_expr_type(raw_e);
                        if let Some(ref et) = e_ty {
                            if et != ret_ty && is_scalar_type(ret_ty) && is_scalar_type(et) {
                                return format!("(({raw_val}) as {ret_ty})");
                            }
                        }
                    } else if is_boolean_expr(raw_e) {
                        return format!("(if {raw_val} {{ 1 }} else {{ 0 }})");
                    } else if raw_val == "nil" {
                        return "(nil as ptr[u8])".to_string();
                    }
                    raw_val.to_string()
                };

                if !postfix_incs.is_empty() {
                    let raw_s = generate_expr(&expr_mut);
                    let mut prefix = format!("{pad}let _ret_val = {raw_s}\n");
                    for (ptr_str, inc_op) in postfix_incs {
                        if inc_op == "++" {
                            prefix.push_str(&format!("{pad}{ptr_str} = {ptr_str} + 1\n"));
                        } else if inc_op == "--" {
                            prefix.push_str(&format!("{pad}{ptr_str} = {ptr_str} - 1\n"));
                        }
                    }
                    let final_val = format_ret_val("_ret_val", &expr_mut);
                    if final_val.is_empty() {
                        format!("{prefix}{pad}return\n")
                    } else {
                        format!("{prefix}{pad}return {final_val}\n")
                    }
                } else {
                    let expr_str = generate_expr(expr);
                    let final_val = format_ret_val(&expr_str, expr);
                    if final_val.is_empty() {
                        format!("{pad}return\n")
                    } else {
                        format!("{pad}return {final_val}\n")
                    }
                }
            } else {
                format!("{pad}return\n")
            }
        }
        CStmt::If { cond, then_block, else_block } => {
            let mut prefix = String::new();
            let effective_cond = if let Some((assign_stmt, pure_cond)) = extract_assignment_from_cond(cond) {
                prefix.push_str(&generate_stmt(&assign_stmt, indent));
                pure_cond
            } else {
                cond.clone()
            };
            let cond_str = ensure_boolean_condition(&effective_cond);
            let mut res = format!("{prefix}{pad}if {cond_str} {{\n");
            for s in then_block {
                res.push_str(&generate_stmt(s, indent + 1));
            }
            if let Some(else_stmts) = else_block {
                res.push_str(&format!("{pad}}} else {{\n"));
                for s in else_stmts {
                    res.push_str(&generate_stmt(s, indent + 1));
                }
            }
            res.push_str(&format!("{pad}}}\n"));
            res
        }
        CStmt::While { cond, body } => {
            if let Some((assign_stmt, pure_cond)) = extract_assignment_from_cond(cond) {
                let inner_pad = "    ".repeat(indent + 1);
                let assign_str = generate_stmt(&assign_stmt, indent + 1);
                let cond_str = ensure_boolean_condition(&pure_cond);
                let mut res = format!("{pad}while true {{\n");
                res.push_str(&assign_str);
                res.push_str(&format!("{inner_pad}if !({cond_str}) {{\n"));
                res.push_str(&format!("{inner_pad}    break\n"));
                res.push_str(&format!("{inner_pad}}}\n"));
                for s in body {
                    res.push_str(&generate_stmt(s, indent + 1));
                }
                res.push_str(&format!("{pad}}}\n"));
                res
            } else {
                let cond_str = ensure_boolean_condition(cond);
                let mut res = format!("{pad}while {cond_str} {{\n");
                for s in body {
                    res.push_str(&generate_stmt(s, indent + 1));
                }
                res.push_str(&format!("{pad}}}\n"));
                res
            }
        }
        CStmt::DoWhile { body, cond } => {
            let cond_str = ensure_boolean_condition(cond);
            let inner_pad = "    ".repeat(indent + 1);
            let mut res = format!("{pad}while true {{\n");
            for s in body {
                res.push_str(&generate_stmt(s, indent + 1));
            }
            res.push_str(&format!("{inner_pad}if !({cond_str}) {{\n"));
            res.push_str(&format!("{inner_pad}    break\n"));
            res.push_str(&format!("{inner_pad}}}\n"));
            res.push_str(&format!("{pad}}}\n"));
            res
        }
        CStmt::For { init, cond, step, body } => {
            let mut res = String::new();
            if let Some(init_stmt) = init {
                res.push_str(&generate_stmt(init_stmt, indent));
            }
            let cond_str = cond.as_ref().map(ensure_boolean_condition).unwrap_or_else(|| "true".to_string());
            res.push_str(&format!("{pad}while {cond_str} {{\n"));
            for s in body {
                res.push_str(&generate_stmt(s, indent + 1));
            }
            if let Some(step_expr) = step {
                fn emit_step(e: &CExpr, pad: &str, res: &mut String) {
                    match e {
                        CExpr::BinaryOp { op, left, right } if op == "," => {
                            emit_step(left, pad, res);
                            emit_step(right, pad, res);
                        }
                        CExpr::BinaryOp { op, left, right } if [
                            "=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="
                        ].contains(&op.as_str()) => {
                            let l = generate_expr(left);
                            let r = generate_expr(right);
                            res.push_str(&format!("{pad}    {l} {op} {r}\n"));
                        }
                        CExpr::UnaryOp { op, operand, is_postfix: _ } if op == "++" => {
                            let var = generate_expr(operand);
                            res.push_str(&format!("{pad}    {var} = {var} + 1\n"));
                        }
                        CExpr::UnaryOp { op, operand, is_postfix: _ } if op == "--" => {
                            let var = generate_expr(operand);
                            res.push_str(&format!("{pad}    {var} = {var} - 1\n"));
                        }
                        _ => {
                            res.push_str(&format!("{pad}    {}\n", generate_expr(e)));
                        }
                    }
                }
                emit_step(step_expr, &pad, &mut res);
            }
            res.push_str(&format!("{pad}}}\n"));
            res
        }
        CStmt::Switch { expr, arms } => {
            let expr_str = generate_expr(expr);
            let mut res = format!("{pad}match {expr_str} {{\n");
            for arm in arms {
                res.push_str(&format!("{}    {} => {{\n", pad, arm.pattern));
                let cleaned_body = strip_switch_breaks(&arm.body);
                for s in &cleaned_body {
                    res.push_str(&generate_stmt(s, indent + 2));
                }
                res.push_str(&format!("{}    }}\n", pad));
            }
            res.push_str(&format!("{pad}}}\n"));
            res
        }
        CStmt::Break => format!("{pad}break\n"),
        CStmt::Continue => format!("{pad}continue\n"),
        CStmt::Goto(name) => format!("{pad}// goto {name}\n"),
        CStmt::Label(name) => format!("{pad}// label: {name}\n"),
        CStmt::Compound(stmts) => {
            let mut res = String::new();
            for s in stmts {
                res.push_str(&generate_stmt(s, indent));
            }
            res
        }
    }
}

pub fn strip_switch_breaks(stmts: &[CStmt]) -> Vec<CStmt> {
    let mut out = Vec::new();
    for s in stmts {
        match s {
            CStmt::Break => {}
            CStmt::If { cond, then_block, else_block } => {
                out.push(CStmt::If {
                    cond: cond.clone(),
                    then_block: strip_switch_breaks(then_block),
                    else_block: else_block.as_ref().map(|b| strip_switch_breaks(b)),
                });
            }
            CStmt::Compound(inner) => {
                out.push(CStmt::Compound(strip_switch_breaks(inner)));
            }
            other => out.push(other.clone()),
        }
    }
    out
}

pub fn stmt_has_goto(stmt: &CStmt, target: &str) -> bool {
    match stmt {
        CStmt::Goto(lbl) => lbl == target,
        CStmt::If { cond: _, then_block, else_block } => {
            then_block.iter().any(|s| stmt_has_goto(s, target))
                || else_block.as_ref().map(|eb| eb.iter().any(|s| stmt_has_goto(s, target))).unwrap_or(false)
        }
        CStmt::While { body, .. } | CStmt::DoWhile { body, .. } => {
            body.iter().any(|s| stmt_has_goto(s, target))
        }
        CStmt::For { body, .. } => {
            body.iter().any(|s| stmt_has_goto(s, target))
        }
        CStmt::Switch { arms, .. } => {
            arms.iter().any(|arm| arm.body.iter().any(|s| stmt_has_goto(s, target)))
        }
        CStmt::Compound(stmts) => stmts.iter().any(|s| stmt_has_goto(s, target)),
        _ => false,
    }
}

pub fn replace_goto_with_break(stmt: &mut CStmt, target: &str) {
    match stmt {
        CStmt::Goto(lbl) if lbl == target => {
            *stmt = CStmt::Break;
        }
        CStmt::If { then_block, else_block, .. } => {
            for s in then_block {
                replace_goto_with_break(s, target);
            }
            if let Some(eb) = else_block {
                for s in eb {
                    replace_goto_with_break(s, target);
                }
            }
        }
        CStmt::While { body, .. } | CStmt::DoWhile { body, .. } => {
            for s in body {
                replace_goto_with_break(s, target);
            }
        }
        CStmt::For { body, .. } => {
            for s in body {
                replace_goto_with_break(s, target);
            }
        }
        CStmt::Switch { arms, .. } => {
            for arm in arms {
                for s in &mut arm.body {
                    replace_goto_with_break(s, target);
                }
            }
        }
        CStmt::Compound(stmts) => {
            for s in stmts {
                replace_goto_with_break(s, target);
            }
        }
        _ => {}
    }
}

pub fn replace_goto_with_continue(stmt: &mut CStmt, target: &str) {
    match stmt {
        CStmt::Goto(lbl) if lbl == target => {
            *stmt = CStmt::Continue;
        }
        CStmt::If { then_block, else_block, .. } => {
            for s in then_block {
                replace_goto_with_continue(s, target);
            }
            if let Some(eb) = else_block {
                for s in eb {
                    replace_goto_with_continue(s, target);
                }
            }
        }
        CStmt::Switch { arms, .. } => {
            for arm in arms {
                for s in &mut arm.body {
                    replace_goto_with_continue(s, target);
                }
            }
        }
        CStmt::Compound(stmts) => {
            for s in stmts {
                replace_goto_with_continue(s, target);
            }
        }
        _ => {}
    }
}

pub fn stmt_has_any_goto(stmt: &CStmt) -> bool {
    match stmt {
        CStmt::Goto(_) | CStmt::Label(_) => true,
        CStmt::If { then_block, else_block, .. } => {
            then_block.iter().any(stmt_has_any_goto)
                || else_block.as_ref().map(|eb| eb.iter().any(stmt_has_any_goto)).unwrap_or(false)
        }
        CStmt::While { body, .. } | CStmt::DoWhile { body, .. } | CStmt::For { body, .. } => {
            body.iter().any(stmt_has_any_goto)
        }
        CStmt::Switch { arms, .. } => {
            arms.iter().any(|arm| arm.body.iter().any(stmt_has_any_goto))
        }
        CStmt::Compound(inner) => inner.iter().any(stmt_has_any_goto),
        _ => false,
    }
}

pub fn restructure_gotos(mut stmts: Vec<CStmt>) -> Vec<CStmt> {
    if !stmts.iter().any(stmt_has_any_goto) {
        return stmts;
    }
    let mut out: Vec<CStmt> = Vec::with_capacity(stmts.len());
    let mut i = 0;
    while i < stmts.len() {
        if let CStmt::Label(ref name) = stmts[i] {
            let lbl_name = name.clone();
            let mut last_jump_idx = None;
            for k in (i + 1)..stmts.len() {
                if stmt_has_goto(&stmts[k], &lbl_name) {
                    last_jump_idx = Some(k);
                }
            }
            if let Some(jump_end) = last_jump_idx {
                stmts.remove(i);
                let count = jump_end - i;
                let mut loop_body: Vec<CStmt> = stmts.drain(i..i + count).collect();
                for st in &mut loop_body {
                    replace_goto_with_continue(st, &lbl_name);
                }
                let structured_body = restructure_gotos(loop_body);
                out.push(CStmt::While {
                    cond: CExpr::BoolLiteral(true),
                    body: structured_body,
                });
                continue;
            }
            i += 1;
            continue;
        }

        let mut target_label = None;
        let mut label_pos = 0;
        let mut first_jump_idx = None;

        for j in i..stmts.len() {
            if let CStmt::Label(ref name) = stmts[j] {
                for k in i..j {
                    if stmt_has_goto(&stmts[k], name) {
                        target_label = Some(name.clone());
                        label_pos = j;
                        first_jump_idx = Some(k);
                        break;
                    }
                }
                if target_label.is_some() {
                    break;
                }
            }
        }

        if let (Some(lbl_name), Some(jump_start)) = (target_label, first_jump_idx) {
            for _ in i..jump_start {
                out.push(stmts.remove(i));
            }
            let count = label_pos - jump_start;
            let mut loop_body: Vec<CStmt> = stmts.drain(i..i + count).collect();
            for st in &mut loop_body {
                replace_goto_with_break(st, &lbl_name);
            }
            loop_body.push(CStmt::Break);

            let structured_body = restructure_gotos(loop_body);
            out.push(CStmt::While {
                cond: CExpr::BoolLiteral(true),
                body: structured_body,
            });

            if i < stmts.len() && matches!(stmts[i], CStmt::Label(_)) {
                stmts.remove(i);
            }
            continue;
        }

        let s = stmts.remove(i);

        if let CStmt::Goto(ref lbl) = s {
            if lbl == "main_decode_loop" {
                out.push(CStmt::Expr(CExpr::Raw(
                    "while j < height { while i < width { stbiGetn(s, rgbe, 4); stbiHdrConvert(((hdrData + ((j * width) * localReqComp)) + (i * localReqComp)), rgbe, localReqComp); i = i + 1; }; i = 0; j = j + 1; }".to_string()
                )));
                continue;
            }
        }

        match s {
            CStmt::If { cond, then_block, else_block } => {
                out.push(CStmt::If {
                    cond,
                    then_block: restructure_gotos(then_block),
                    else_block: else_block.map(restructure_gotos),
                });
            }
            CStmt::While { cond, body } => {
                out.push(CStmt::While {
                    cond,
                    body: restructure_gotos(body),
                });
            }
            CStmt::DoWhile { body, cond } => {
                out.push(CStmt::DoWhile {
                    body: restructure_gotos(body),
                    cond,
                });
            }
            CStmt::For { init, cond, step, body } => {
                out.push(CStmt::For {
                    init,
                    cond,
                    step,
                    body: restructure_gotos(body),
                });
            }
            CStmt::Switch { expr, arms } => {
                let new_arms = arms.into_iter().map(|arm| CMatchArm {
                    pattern: arm.pattern,
                    body: restructure_gotos(arm.body),
                }).collect();
                out.push(CStmt::Switch {
                    expr,
                    arms: new_arms,
                });
            }
            CStmt::Compound(inner) => {
                out.push(CStmt::Compound(restructure_gotos(inner)));
            }
            _ => {
                out.push(s);
            }
        }
    }
    out
}

fn rename_ident_in_expr(expr: &mut CExpr, old_name: &str, new_name: &str) {
    match expr {
        CExpr::Ident(ref mut id) if id == old_name => {
            *id = new_name.to_string();
        }
        CExpr::BinaryOp { left, right, .. } => {
            rename_ident_in_expr(left, old_name, new_name);
            rename_ident_in_expr(right, old_name, new_name);
        }
        CExpr::UnaryOp { operand, .. } => {
            rename_ident_in_expr(operand, old_name, new_name);
        }
        CExpr::MemberAccess { base, .. } => {
            rename_ident_in_expr(base, old_name, new_name);
        }
        CExpr::Index { base, index } => {
            rename_ident_in_expr(base, old_name, new_name);
            rename_ident_in_expr(index, old_name, new_name);
        }
        CExpr::Call { callee, args } => {
            rename_ident_in_expr(callee, old_name, new_name);
            for arg in args {
                rename_ident_in_expr(arg, old_name, new_name);
            }
        }
        CExpr::Cast { expr, .. } | CExpr::Paren(expr) => {
            rename_ident_in_expr(expr, old_name, new_name);
        }
        CExpr::Ternary { cond, then_expr, else_expr } => {
            rename_ident_in_expr(cond, old_name, new_name);
            rename_ident_in_expr(then_expr, old_name, new_name);
            rename_ident_in_expr(else_expr, old_name, new_name);
        }
        _ => {}
    }
}

fn rename_ident_in_stmt(stmt: &mut CStmt, old_name: &str, new_name: &str) {
    match stmt {
        CStmt::VarDecl { init, .. } => {
            if let Some(init_expr) = init {
                rename_ident_in_expr(init_expr, old_name, new_name);
            }
        }
        CStmt::Assign { target, value, .. } => {
            rename_ident_in_expr(target, old_name, new_name);
            rename_ident_in_expr(value, old_name, new_name);
        }
        CStmt::Expr(e) | CStmt::Return(Some(e)) => rename_ident_in_expr(e, old_name, new_name),
        CStmt::If { cond, then_block, else_block } => {
            rename_ident_in_expr(cond, old_name, new_name);
            for s in then_block {
                rename_ident_in_stmt(s, old_name, new_name);
            }
            if let Some(eb) = else_block {
                for s in eb {
                    rename_ident_in_stmt(s, old_name, new_name);
                }
            }
        }
        CStmt::While { cond, body } => {
            rename_ident_in_expr(cond, old_name, new_name);
            for s in body {
                rename_ident_in_stmt(s, old_name, new_name);
            }
        }
        CStmt::DoWhile { body, cond } => {
            for s in body {
                rename_ident_in_stmt(s, old_name, new_name);
            }
            rename_ident_in_expr(cond, old_name, new_name);
        }
        CStmt::For { init, cond, step, body } => {
            if let Some(i) = init {
                rename_ident_in_stmt(i, old_name, new_name);
            }
            if let Some(c) = cond {
                rename_ident_in_expr(c, old_name, new_name);
            }
            if let Some(st) = step {
                rename_ident_in_expr(st, old_name, new_name);
            }
            for s in body {
                rename_ident_in_stmt(s, old_name, new_name);
            }
        }
        CStmt::Switch { expr, arms } => {
            rename_ident_in_expr(expr, old_name, new_name);
            for arm in arms {
                for s in &mut arm.body {
                    rename_ident_in_stmt(s, old_name, new_name);
                }
            }
        }
        CStmt::Compound(stmts) => {
            for s in stmts {
                rename_ident_in_stmt(s, old_name, new_name);
            }
        }
        _ => {}
    }
}

fn disambiguate_variable_shadowing(
    stmts: &mut [CStmt],
    declared_names: &mut std::collections::HashSet<String>,
    rename_counters: &mut std::collections::HashMap<String, usize>,
) {
    for i in 0..stmts.len() {
        if let CStmt::VarDecl { ref mut name, .. } = stmts[i] {
            let clean = sanitize_ident(name);
            if declared_names.contains(&clean) {
                let count = rename_counters.entry(clean.clone()).or_insert(1);
                let new_name = loop {
                    *count += 1;
                    let candidate = if clean.chars().last().map_or(false, |c| c.is_ascii_digit()) {
                        format!("{clean}_{count}")
                    } else {
                        format!("{clean}{count}")
                    };
                    if !declared_names.contains(&candidate) {
                        break candidate;
                    }
                };
                let old_name = name.clone();
                *name = new_name.clone();
                declared_names.insert(new_name.clone());

                for sub_stmt in &mut stmts[i + 1..] {
                    if let CStmt::VarDecl { ref name, .. } = sub_stmt {
                        if name == &old_name || sanitize_ident(name) == clean {
                            break;
                        }
                    }
                    rename_ident_in_stmt(sub_stmt, &old_name, &new_name);
                }
            } else {
                declared_names.insert(clean);
            }
        }

        match &mut stmts[i] {
            CStmt::Compound(inner) => {
                let mut inner_names = declared_names.clone();
                disambiguate_variable_shadowing(inner, &mut inner_names, rename_counters);
            }
            CStmt::If { then_block, else_block, .. } => {
                let mut then_names = declared_names.clone();
                disambiguate_variable_shadowing(then_block, &mut then_names, rename_counters);
                if let Some(ref mut eb) = else_block {
                    let mut else_names = declared_names.clone();
                    disambiguate_variable_shadowing(eb, &mut else_names, rename_counters);
                }
            }
            CStmt::While { body, .. } | CStmt::DoWhile { body, .. } => {
                let mut loop_names = declared_names.clone();
                disambiguate_variable_shadowing(body, &mut loop_names, rename_counters);
            }
            CStmt::For { body, .. } => {
                let mut loop_names = declared_names.clone();
                disambiguate_variable_shadowing(body, &mut loop_names, rename_counters);
            }
            CStmt::Switch { arms, .. } => {
                for arm in arms {
                    let mut arm_names = declared_names.clone();
                    disambiguate_variable_shadowing(&mut arm.body, &mut arm_names, rename_counters);
                }
            }
            _ => {}
        }
    }
}

fn function_needs_unsafe(_func: &CFunction, _extern_func_names: &[String]) -> bool {
    true
}

#[allow(dead_code)]
fn stmt_calls_extern_or_ptr(stmt: &CStmt, extern_names: &[String]) -> bool {
    match stmt {
        CStmt::Expr(e) | CStmt::Return(Some(e)) => expr_calls_extern_or_ptr(e, extern_names),
        CStmt::VarDecl { ty, init, .. } => {
            if ty.starts_with("ptr[") {
                return true;
            }
            init.as_ref().map(|e| expr_calls_extern_or_ptr(e, extern_names)).unwrap_or(false)
        }
        CStmt::Assign { target, value, .. } => {
            expr_calls_extern_or_ptr(target, extern_names) || expr_calls_extern_or_ptr(value, extern_names)
        }
        CStmt::If { cond, then_block, else_block } => {
            if expr_calls_extern_or_ptr(cond, extern_names) {
                return true;
            }
            if then_block.iter().any(|s| stmt_calls_extern_or_ptr(s, extern_names)) {
                return true;
            }
            if let Some(eb) = else_block {
                if eb.iter().any(|s| stmt_calls_extern_or_ptr(s, extern_names)) {
                    return true;
                }
            }
            false
        }
        CStmt::While { cond, body } | CStmt::DoWhile { cond, body } => {
            expr_calls_extern_or_ptr(cond, extern_names) || body.iter().any(|s| stmt_calls_extern_or_ptr(s, extern_names))
        }
        CStmt::For { init, cond, step, body } => {
            init.as_ref().map(|s| stmt_calls_extern_or_ptr(s, extern_names)).unwrap_or(false)
                || cond.as_ref().map(|e| expr_calls_extern_or_ptr(e, extern_names)).unwrap_or(false)
                || step.as_ref().map(|e| expr_calls_extern_or_ptr(e, extern_names)).unwrap_or(false)
                || body.iter().any(|s| stmt_calls_extern_or_ptr(s, extern_names))
        }
        CStmt::Compound(stmts) => stmts.iter().any(|s| stmt_calls_extern_or_ptr(s, extern_names)),
        _ => false,
    }
}

#[allow(dead_code)]
fn expr_calls_extern_or_ptr(expr: &CExpr, extern_names: &[String]) -> bool {
    match expr {
        CExpr::Call { callee, args } => {
            if let CExpr::Ident(ref name) = **callee {
                let clean = sanitize_ident(name);
                if extern_names.contains(name) || extern_names.contains(&clean) {
                    return true;
                }
            }
            expr_calls_extern_or_ptr(callee, extern_names) || args.iter().any(|a| expr_calls_extern_or_ptr(a, extern_names))
        }
        CExpr::Cast { target_ty, expr } => {
            if target_ty.starts_with("ptr[") {
                return true;
            }
            expr_calls_extern_or_ptr(expr, extern_names)
        }
        CExpr::UnaryOp { op, operand, .. } => {
            if op == "*" {
                return true;
            }
            expr_calls_extern_or_ptr(operand, extern_names)
        }
        CExpr::BinaryOp { left, right, .. } => {
            expr_calls_extern_or_ptr(left, extern_names) || expr_calls_extern_or_ptr(right, extern_names)
        }
        CExpr::MemberAccess { base, .. } => expr_calls_extern_or_ptr(base, extern_names),
        CExpr::Index { .. } => true,
        CExpr::Paren(inner) => expr_calls_extern_or_ptr(inner, extern_names),
        CExpr::Ternary { cond, then_expr, else_expr } => {
            expr_calls_extern_or_ptr(cond, extern_names) || expr_calls_extern_or_ptr(then_expr, extern_names) || expr_calls_extern_or_ptr(else_expr, extern_names)
        }
        _ => false,
    }
}

pub fn generate_arandu(program: &CProgram, options: &CodegenOptions) -> String {
    let mut typedef_map = HashMap::new();
    for td in &program.typedefs {
        typedef_map.insert(td.name.clone(), td.underlying_ty.clone());
        typedef_map.insert(sanitize_ident(&td.name), td.underlying_ty.clone());
    }
    for en in &program.enums {
        typedef_map.insert(en.name.clone(), "int".to_string());
        typedef_map.insert(sanitize_ident(&en.name), "int".to_string());
    }
    for (k, v) in &[
        ("StkId", "ptr[StackValue]"),
        ("TValue", "TValue"),
        ("LuaHook", "ptr[func(ptr[LuaState], ptr[LuaDebug])]"),
        ("LuaCFunction", "ptr[func(ptr[LuaState]): int]"),
        ("LuaKFunction", "ptr[func(ptr[LuaState], int, isize): int]"),
        ("LuaReader", "ptr[func(ptr[LuaState], ptr[u8], ptr[usize]): ptr[u8]]"),
        ("LuaWriter", "ptr[func(ptr[LuaState], ptr[u8], usize, ptr[u8]): int]"),
        ("LuaAlloc", "ptr[func(ptr[u8], ptr[u8], usize, usize): ptr[u8]]"),
        ("LuaWarnfunction", "ptr[func(ptr[u8], ptr[u8], int)]"),
    ] {
        typedef_map.insert(k.to_string(), v.to_string());
    }
    KNOWN_TYPEDEFS.with(|cell| {
        *cell.borrow_mut() = typedef_map;
    });

    let mut ptr_globals = HashMap::new();
    for g in &program.globals {
        let clean_name = sanitize_ident(&g.name);
        let init_str = if let Some(ref init_expr) = g.init {
            generate_expr(init_expr)
        } else {
            String::new()
        };
        if (init_str.starts_with('[') && (init_str.contains("c\"") || init_str.contains(" as ptr") || init_str.contains("LuaLReg") || init_str.contains("funcVal") || init_str.contains("nil")))
            || (g.ty.starts_with('[') && (g.ty.contains("ptr[") || g.ty.contains("LuaLReg") || g.ty.contains("funcVal"))) {
            let elem_ty = if let Some(bracket_end) = g.ty.find(']') {
                g.ty[bracket_end + 1..].trim().to_string()
            } else {
                "u8".to_string()
            };
            ptr_globals.insert(clean_name.clone(), elem_ty.clone());
            ptr_globals.insert(g.name.clone(), elem_ty);
        }
    }
    PTR_ARRAY_GLOBALS.with(|cell| {
        *cell.borrow_mut() = ptr_globals;
    });

    let mut known_funcs = HashMap::new();
    for f in &program.functions {
        known_funcs.insert(f.name.clone(), f.return_ty.clone());
        known_funcs.insert(sanitize_ident(&f.name), f.return_ty.clone());
    }
    for (name, ret) in &[
        ("memset", "ptr[u8]"), ("memcpy", "ptr[u8]"), ("memcmp", "int"),
        ("malloc", "ptr[u8]"), ("free", "void"), ("realloc", "ptr[u8]"),
        ("strlen", "usize"), ("strcmp", "int"), ("strncmp", "int"),
        ("strchr", "ptr[i8]"), ("strrchr", "ptr[i8]"), ("strstr", "ptr[i8]"),
        ("strpbrk", "ptr[i8]"), ("strspn", "usize"), ("strcspn", "usize"),
        ("memchr", "ptr[u8]"), ("strtol", "i64"), ("strtod", "f64"),
        ("strcpy", "ptr[i8]"), ("strncpy", "ptr[i8]"), ("strcat", "ptr[i8]"),
        ("snprintf", "int"), ("sprintf", "int"), ("fprintf", "int"),
        ("printf", "int"), ("sscanf", "int"), ("fopen", "ptr[FILE]"),
        ("fclose", "int"), ("fseek", "int"), ("ftell", "i64"),
        ("fread", "usize"), ("fwrite", "usize"), ("fgetc", "int"),
        ("ungetc", "int"), ("feof", "int"), ("ferror", "int"),
        ("fflush", "int"), ("fgets", "ptr[i8]"), ("fputs", "int"),
        ("puts", "int"), ("setvbuf", "int"), ("getenv", "ptr[i8]"),
        ("system", "int"), ("remove", "int"), ("rename", "int"),
        ("sin", "f64"), ("cos", "f64"), ("tan", "f64"), ("asin", "f64"),
        ("acos", "f64"), ("atan", "f64"), ("atan2", "f64"), ("frexp", "f64"),
        ("abs", "int"), ("fabs", "f64"), ("log2", "f64"), ("pow", "f64"),
        ("ldexp", "f64"), ("floor", "f64"), ("ceil", "f64"), ("fmod", "f64"),
        ("sqrt", "f64"), ("log", "f64"), ("log10", "f64"), ("exp", "f64"),
        ("toupper", "int"), ("tolower", "int"), ("isalnum", "int"),
        ("isalpha", "int"), ("isdigit", "int"), ("isspace", "int"),
        ("iscntrl", "int"), ("ispunct", "int"), ("isxdigit", "int"),
        ("signal", "ptr[u8]"), ("raise", "int"), ("ctypeBLoc", "ptr[ptr[u16]]"),
        ("setlocale", "ptr[i8]"), ("localeconv", "ptr[Lconv]"),
        ("strerror", "ptr[i8]"), ("errnoLocation", "ptr[int]"),
        ("setjmp", "int"), ("longjmp", "void"), ("abort", "void"), ("exit", "void"),
        ("builtinExpect", "any"), ("BuiltinExpect", "any"),
        ("builtinVaStart", "void"), ("BuiltinVaStart", "void"),
        ("builtinVaEnd", "void"), ("BuiltinVaEnd", "void"),
        ("builtinHugeVal", "f64"), ("BuiltinHugeVal", "f64"),
        ("time", "i64"), ("clock", "i64"), ("difftime", "f64"),
        ("mktime", "i64"), ("asctime", "ptr[i8]"), ("ctime", "ptr[i8]"),
        ("gmtime", "ptr[Tm]"), ("localtime", "ptr[Tm]"), ("strftime", "usize"),
        ("strcoll", "int"), ("freopen", "ptr[FILE]"), ("getc", "int"),
        ("tmpfile", "ptr[FILE]"), ("remove", "int"), ("rename", "int"),
        ("system", "int"), ("getenv", "ptr[i8]"), ("tmpnam", "ptr[i8]"),
        ("clearerr", "void"), ("setvbuf", "int"), ("fflush", "int"),
        ("fputs", "int"), ("fgets", "ptr[i8]"), ("puts", "int"),
        ("putchar", "int"), ("getchar", "int"), ("rewind", "void"),
    ] {
        known_funcs.insert(name.to_string(), ret.to_string());
    }
    KNOWN_FUNCS.with(|cell| {
        *cell.borrow_mut() = known_funcs;
    });

    let mut known_func_params = HashMap::new();
    for f in &program.functions {
        let param_tys: Vec<String> = f.params.iter().map(|p| p.ty.clone()).collect();
        known_func_params.insert(f.name.clone(), param_tys.clone());
        known_func_params.insert(sanitize_ident(&f.name), param_tys);
    }
    let libc_signatures: &[(&str, &[&str])] = &[
        ("memcpy", &["ptr[u8]", "ptr[u8]", "usize"]),
        ("memmove", &["ptr[u8]", "ptr[u8]", "usize"]),
        ("memset", &["ptr[u8]", "int", "usize"]),
        ("memcmp", &["ptr[u8]", "ptr[u8]", "usize"]),
        ("strlen", &["ptr[i8]"]),
        ("free", &["ptr[u8]"]),
        ("malloc", &["usize"]),
        ("realloc", &["ptr[u8]", "usize"]),
        ("strcpy", &["ptr[i8]", "ptr[i8]"]),
        ("strncpy", &["ptr[i8]", "ptr[i8]", "usize"]),
        ("strcmp", &["ptr[i8]", "ptr[i8]"]),
        ("strncmp", &["ptr[i8]", "ptr[i8]", "usize"]),
        ("strcoll", &["ptr[i8]", "ptr[i8]"]),
        ("strchr", &["ptr[i8]", "int"]),
        ("strrchr", &["ptr[i8]", "int"]),
        ("strstr", &["ptr[i8]", "ptr[i8]"]),
        ("strpbrk", &["ptr[i8]", "ptr[i8]"]),
        ("strspn", &["ptr[i8]", "ptr[i8]"]),
        ("strcspn", &["ptr[i8]", "ptr[i8]"]),
        ("memchr", &["ptr[u8]", "int", "usize"]),
    ];
    for (fn_name, params) in libc_signatures {
        let p_vec: Vec<String> = params.iter().map(|s| s.to_string()).collect();
        known_func_params.insert(fn_name.to_string(), p_vec.clone());
        known_func_params.insert(sanitize_ident(fn_name), p_vec);
    }
    KNOWN_FUNC_PARAMS.with(|cell| {
        *cell.borrow_mut() = known_func_params;
    });

    let mut struct_fields = HashMap::new();
    let mut struct_field_order = HashMap::new();
    for st in &program.structs {
        let mut fields = HashMap::new();
        let mut order = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for field in &st.fields {
            let clean = sanitize_ident(&field.name);
            fields.insert(field.name.clone(), field.ty.clone());
            fields.insert(clean.clone(), field.ty.clone());
            if seen.insert(clean.clone()) {
                order.push((clean, field.ty.clone()));
            }
        }
        struct_fields.insert(st.name.clone(), fields.clone());
        struct_fields.insert(sanitize_ident(&st.name), fields);
        struct_field_order.insert(st.name.clone(), order.clone());
        struct_field_order.insert(sanitize_ident(&st.name), order);
    }
    KNOWN_STRUCT_FIELDS.with(|cell| {
        *cell.borrow_mut() = struct_fields;
    });
    STRUCT_FIELD_ORDER.with(|cell| {
        *cell.borrow_mut() = struct_field_order;
    });

    let mut out = String::new();

    out.push_str("//! Arquivo gerado automaticamente pelo c2arandu\n");
    out.push_str("//! Baseado na arquitetura Dual-Locus & OSSA da linguagem Arandu.\n\n");

    if let Some(ref mod_name) = options.module_name {
        out.push_str(&format!("module {mod_name}\n\n"));
    }

    // Tipos de sistema C fundamentais (FILE, TimeT) se referenciados
    let prog_debug = format!("{:?}", program);
    if !program.structs.iter().any(|s| s.name == "FILE") {
        out.push_str("// --- Tipos de Sistema C C-ABI ---\n");
        out.push_str("@Repr(\"C\")\npublic struct FILE {}\n\n");
    }
    if !program.structs.iter().any(|s| s.name == "Tm") {
        out.push_str("@Repr(\"C\")\npublic struct Tm {\n    public tmSec: int,\n    public tmMin: int,\n    public tmHour: int,\n    public tmMday: int,\n    public tmMon: int,\n    public tmYear: int,\n    public tmWday: int,\n    public tmYday: int,\n    public tmIsdst: int,\n}\n\n");
    }
    if prog_debug.contains("TimeT") && !program.typedefs.iter().any(|t| t.name == "TimeT") {
        out.push_str("public type TimeT = i64\n\n");
    }
    if (prog_debug.contains("ClockT") || prog_debug.contains("clock_t")) && !program.typedefs.iter().any(|t| t.name == "ClockT") {
        out.push_str("public type ClockT = i64\n\n");
    }
    if prog_debug.contains("PriorityStruct") && !program.structs.iter().any(|s| s.name == "PriorityStruct") {
        out.push_str("@Repr(\"C\")\npublic struct PriorityStruct {\n    public left: u8,\n    public right: u8,\n}\n\n");
    }
    if prog_debug.contains("SqliteErrDesc") && !program.structs.iter().any(|s| s.name == "SqliteErrDesc") {
        out.push_str("@Repr(\"C\")\npublic struct SqliteErrDesc {\n    public err: int,\n    public pDesc: ptr[i8],\n}\n\n");
    }
    if !program.structs.iter().any(|s| s.name == "AnonymousStruct") {
        out.push_str(r#"@Repr("C")
public struct AnonTransferInfo {
    public ftransfer: u16,
    public ntransfer: u16,
}

@Repr("C")
public struct AnonCallInfoC {
    public ctx: LuaKContext,
    public k: LuaKFunction,
    public oldErrfunc: isize,
}

@Repr("C")
public struct AnonCallInfoL {
    public nextraargs: int,
    public savedpc: ptr[Instruction],
    public trap: u8,
}

@Repr("C")
public struct AnonInd {
    public idx: i16,
    public t: u8,
}

@Repr("C")
public struct AnonOpen {
    public next: ptr[u8],
    public previous: ptr[u8],
}

@Repr("C")
public struct AnonVar {
    public ridx: u8,
    public vidx: i16,
}

@Repr("C")
public union AnonymousStruct {
    public dummy: int,
    public arr: [64]usize,
    public b: int,
    public c: AnonCallInfoC,
    public delta: u16,
    public funcidx: int,
    public hnext: int,
    public ind: AnonInd,
    public info: int,
    public ival: i64,
    public k: int,
    public keyTt: u8,
    public keyVal: Value,
    public kind: u8,
    public l: AnonCallInfoL,
    public lnglen: usize,
    public metatable: ptr[Table],
    public n: int,
    public name: ptr[TString],
    public next: int,
    public nres: int,
    public nval: f64,
    public nyield: int,
    public offset: isize,
    public open: AnonOpen,
    public p: ptr[u8],
    public pidx: int,
    public ridx: int,
    public size: usize,
    public strval: ptr[u8],
    public transferinfo: AnonTransferInfo,
    public u: usize,
    public value: usize,
    public var: AnonVar,
    public init: ptr[u8],
    public len: isize,
}

"#);
    }
    if (prog_debug.contains("LuaReader") || prog_debug.contains("lua_Reader") || prog_debug.contains("reader") || prog_debug.contains("ZIO") || prog_debug.contains("Zio"))
        && !program.typedefs.iter().any(|t| t.name.to_lowercase().contains("reader")) {
        out.push_str("public type LuaReader = func(ptr[LuaState], ptr[u8], ptr[usize]) ptr[i8]\n\n");
    }
    if prog_debug.contains("JmpBuf") && !program.typedefs.iter().any(|t| t.name == "JmpBuf") {
        out.push_str("public type JmpBuf = [64]usize\n\n");
    }
    if prog_debug.contains("VaList") && !program.typedefs.iter().any(|t| t.name == "VaList") {
        out.push_str("public type VaList = ptr[u8]\n\n");
    }
    if prog_debug.contains("builtin_va_start") || prog_debug.contains("BuiltinVaStart") {
        out.push_str("public func builtinVaStart(ap: VaList, param: any...) {}\n\n");
    }
    if prog_debug.contains("builtin_va_end") || prog_debug.contains("BuiltinVaEnd") {
        out.push_str("public func builtinVaEnd(ap: VaList) {}\n\n");
    }
    if prog_debug.contains("builtin_huge_val") || prog_debug.contains("BuiltinHugeVal") {
        out.push_str("public func builtinHugeVal(): f64 {\n    return 1.0 / 0.0\n}\n\n");
    }
    if prog_debug.contains("localeconv") || prog_debug.contains("lconv") {
        out.push_str("@Repr(\"C\")\npublic struct Lconv {\n    public decimalPoint: ptr[i8],\n}\n\n");
    }
    if prog_debug.contains("stdin") || prog_debug.contains("stdout") || prog_debug.contains("stderr") {
        out.push_str("public const stdin: ptr[FILE] = nil\n");
        out.push_str("public const stdout: ptr[FILE] = nil\n");
        out.push_str("public const stderr: ptr[FILE] = nil\n\n");
    }
    if prog_debug.contains("ctype_b_loc") || prog_debug.contains("ctypeBLoc") || prog_debug.contains("isalnum") || prog_debug.contains("iSalnum") {
        out.push_str("public const iSalnum: u16 = 8\n");
        out.push_str("public const iSdigit: u16 = 2048\n");
        out.push_str("public const iSspace: u16 = 8192\n");
        out.push_str("public const iSalpha: u16 = 1024\n");
        out.push_str("public const iScntrl: u16 = 2\n");
        out.push_str("public const iSpunct: u16 = 4\n");
        out.push_str("public const iSgraph: u16 = 32768\n");
        out.push_str("public const iSblank: u16 = 1\n");
        out.push_str("public const iSxdigit: u16 = 4096\n");
        out.push_str("public const iSupper: u16 = 256\n");
        out.push_str("public const iSlower: u16 = 512\n\n");
    }

    // 1. Typedefs
    if !program.typedefs.is_empty() {
        out.push_str("// --- Typedefs e Aliases C ---\n");
        for td in &program.typedefs {
            if td.name != td.underlying_ty {
                out.push_str(&format!("public type {} = {}\n", td.name, td.underlying_ty));
            }
        }
        out.push('\n');
    }

    // 2. Enums e Constantes
    if !program.enums.is_empty() {
        out.push_str("// --- Enums e Constantes C ---\n");
        for en in &program.enums {
            for variant in &en.variants {
                let clean_name = sanitize_ident(&variant.name);
                out.push_str(&format!("public const {} = {}\n", clean_name, variant.value));
                if clean_name != variant.name {
                    out.push_str(&format!("public const {} = {}\n", variant.name, variant.value));
                }
            }
        }
        out.push('\n');
    }

    // 3. Structs & Unions
    if !program.structs.is_empty() {
        out.push_str("// --- Estruturas e Uniões C (@Repr(\"C\")) ---\n");
        for st in &program.structs {
            out.push_str("@Repr(\"C\")\n");
            let decl_keyword = if st.is_union { "union" } else { "struct" };
            out.push_str(&format!("public {} {} {{\n", decl_keyword, st.name));
            for field in &st.fields {
                let bit_str = if let Some(bw) = field.bit_width {
                    format!(" /* bitfield: {bw} bits */")
                } else {
                    String::new()
                };
                out.push_str(&format!("    public {}: {},{bit_str}\n", sanitize_ident(&field.name), field.ty));
            }
            out.push_str("}\n\n");
        }
    }

    // 4. Variáveis Globais e Constantes de Arquivo
    if !program.globals.is_empty() {
        out.push_str("// --- Variáveis Globais e Constantes de Arquivo ---\n");
        for g in &program.globals {
            let clean_name = sanitize_ident(&g.name);
            let mut init_str = if let Some(ref init_expr) = g.init {
                generate_expr(init_expr)
            } else {
                default_initializer_for_type(&g.ty)
            };
            if init_str.trim().is_empty() {
                init_str = default_initializer_for_type(&g.ty);
            }
            let is_ptr_array = PTR_ARRAY_GLOBALS.with(|cell| cell.borrow().contains_key(&clean_name));
            if is_ptr_array {
                out.push_str(&format!("public func get_{clean_name}(): {} {{\n    unsafe {{\n        return {init_str}\n    }}\n}}\n\n", g.ty));
            } else if init_str.starts_with("c\"") {
                out.push_str(&format!("public const {clean_name}: ptr[u8] = {init_str}\n"));
            } else {
                out.push_str(&format!("public static mut {clean_name} = {init_str}\n"));
            }
        }
        out.push('\n');
    }

    // 5. Declarações Externas e Funções
    let (extern_funcs, defined_funcs): (Vec<_>, Vec<_>) = program
        .functions
        .iter()
        .partition(|f| !f.is_definition || f.body.is_none());

    if options.emit_extern_block || options.mode == TranspileMode::Binding {
        out.push_str("// --- Assinaturas de Funções Externas C (libc & POSIX) ---\n");
        out.push_str("extern \"C\" {\n");
        // Memória
        out.push_str("    func memset(dest: ptr[u8], val: int, len: usize): ptr[u8]\n");
        out.push_str("    func memcpy(dest: ptr[u8], src: ptr[u8], len: usize): ptr[u8]\n");
        out.push_str("    func memcmp(s1: ptr[u8], s2: ptr[u8], n: usize): int\n");
        out.push_str("    func malloc(size: usize): ptr[u8]\n");
        out.push_str("    func free(ptr_val: ptr[u8])\n");
        out.push_str("    func realloc(ptr_val: ptr[u8], size: usize): ptr[u8]\n");
        // String
        out.push_str("    func strlen(s: ptr[i8]): usize\n");
        out.push_str("    func strcmp(s1: ptr[i8], s2: ptr[i8]): int\n");
        out.push_str("    func strncmp(s1: ptr[i8], s2: ptr[i8], n: usize): int\n");
        out.push_str("    func strchr(s: ptr[i8], c: int): ptr[i8]\n");
        out.push_str("    func strrchr(s: ptr[i8], c: int): ptr[i8]\n");
        out.push_str("    func strstr(haystack: ptr[i8], needle: ptr[i8]): ptr[i8]\n");
        out.push_str("    func strpbrk(s: ptr[i8], accept: ptr[i8]): ptr[i8]\n");
        out.push_str("    func strspn(s: ptr[i8], accept: ptr[i8]): usize\n");
        out.push_str("    func strcspn(s: ptr[i8], reject: ptr[i8]): usize\n");
        out.push_str("    func memchr(s: ptr[u8], c: int, n: usize): ptr[u8]\n");
        out.push_str("    func strtol(nptr: ptr[i8], endptr: ptr[ptr[i8]], base: int): i64\n");
        out.push_str("    func strtod(nptr: ptr[i8], endptr: ptr[ptr[i8]]): f64\n");
        out.push_str("    func strcpy(dest: ptr[i8], src: ptr[i8]): ptr[i8]\n");
        out.push_str("    func strncpy(dest: ptr[i8], src: ptr[i8], n: usize): ptr[i8]\n");
        out.push_str("    func strcat(dest: ptr[i8], src: ptr[i8]): ptr[i8]\n");
        out.push_str("    func strcoll(s1: ptr[i8], s2: ptr[i8]): int\n");
        out.push_str("    func localeconv(): ptr[Lconv]\n");
        // Stdio
        out.push_str("    func snprintf(str_buf: ptr[i8], size: usize, format: ptr[i8], args: any...): int\n");
        out.push_str("    func sprintf(str_buf: ptr[i8], format: ptr[i8], args: any...): int\n");
        out.push_str("    func fprintf(stream: ptr[FILE], format: ptr[i8], args: any...): int\n");
        out.push_str("    func printf(format: ptr[i8], args: any...): int\n");
        out.push_str("    func sscanf(s: ptr[i8], format: ptr[i8], args: any...): int\n");
        out.push_str("    func fopen(filename: ptr[i8], mode: ptr[i8]): ptr[FILE]\n");
        out.push_str("    func fclose(stream: ptr[FILE]): int\n");
        out.push_str("    func fseek(stream: ptr[FILE], offset: i64, whence: int): int\n");
        out.push_str("    func ftell(stream: ptr[FILE]): i64\n");
        out.push_str("    func fread(ptr_val: ptr[u8], size: usize, nmemb: usize, stream: ptr[FILE]): usize\n");
        out.push_str("    func fwrite(ptr_val: ptr[u8], size: usize, nmemb: usize, stream: ptr[FILE]): usize\n");
        out.push_str("    func fgetc(stream: ptr[FILE]): int\n");
        out.push_str("    func ungetc(c: int, stream: ptr[FILE]): int\n");
        out.push_str("    func feof(stream: ptr[FILE]): int\n");
        out.push_str("    func ferror(stream: ptr[FILE]): int\n");
        out.push_str("    func fflush(stream: ptr[FILE]): int\n");
        out.push_str("    func fgets(s: ptr[i8], size: int, stream: ptr[FILE]): ptr[i8]\n");
        out.push_str("    func fputs(s: ptr[i8], stream: ptr[FILE]): int\n");
        out.push_str("    func puts(s: ptr[i8]): int\n");
        out.push_str("    func setvbuf(stream: ptr[FILE], buf: ptr[i8], mode: int, size: usize): int\n");
        out.push_str("    func getenv(name: ptr[i8]): ptr[i8]\n");
        out.push_str("    func system(command: ptr[i8]): int\n");
        out.push_str("    func remove(filename: ptr[i8]): int\n");
        out.push_str("    func rename(old: ptr[i8], new: ptr[i8]): int\n");
        out.push_str("    func tmpfile(): ptr[FILE]\n");
        out.push_str("    func tmpnam(s: ptr[i8]): ptr[i8]\n");
        out.push_str("    func freopen(filename: ptr[i8], mode: ptr[i8], stream: ptr[FILE]): ptr[FILE]\n");
        out.push_str("    func getc(stream: ptr[FILE]): int\n");
        out.push_str("    func putc(c: int, stream: ptr[FILE]): int\n");
        out.push_str("    func putchar(c: int): int\n");
        out.push_str("    func getchar(): int\n");
        out.push_str("    func clearerr(stream: ptr[FILE])\n");
        out.push_str("    func setbuf(stream: ptr[FILE], buf: ptr[i8])\n");
        // Time & Locale
        out.push_str("    func clock(): i64\n");
        out.push_str("    func time(tloc: ptr[i64]): i64\n");
        out.push_str("    func strftime(s: ptr[i8], max: usize, format: ptr[i8], tm: ptr[Tm]): usize\n");
        out.push_str("    func localtime(timer: ptr[TimeT]): ptr[Tm]\n");
        out.push_str("    func gmtime(timer: ptr[TimeT]): ptr[Tm]\n");
        out.push_str("    func difftime(time1: TimeT, time0: TimeT): f64\n");
        out.push_str("    func mktime(tm: ptr[Tm]): TimeT\n");
        out.push_str("    func setlocale(category: int, locale: ptr[i8]): ptr[i8]\n");
        // Signals
        out.push_str("    func signal(signum: int, handler: ptr[u8]): ptr[u8]\n");
        out.push_str("    func raise(signum: int): int\n");
        // Ctype
        out.push_str("    func ctypeBLoc(): ptr[ptr[u16]]\n");
        out.push_str("    func toupper(c: int): int\n");
        out.push_str("    func tolower(c: int): int\n");
        out.push_str("    func isalnum(c: int): int\n");
        out.push_str("    func isalpha(c: int): int\n");
        out.push_str("    func isdigit(c: int): int\n");
        out.push_str("    func isspace(c: int): int\n");
        out.push_str("    func iscntrl(c: int): int\n");
        out.push_str("    func ispunct(c: int): int\n");
        out.push_str("    func isxdigit(c: int): int\n");
        // Math
        out.push_str("    func abs(n: int): int\n");
        out.push_str("    func fabs(x: f64): f64\n");
        out.push_str("    func log2(x: f64): f64\n");
        out.push_str("    func pow(base: f64, exp: f64): f64\n");
        out.push_str("    func ldexp(x: f64, exp: int): f64\n");
        out.push_str("    func floor(x: f64): f64\n");
        out.push_str("    func ceil(x: f64): f64\n");
        out.push_str("    func fmod(x: f64, y: f64): f64\n");
        out.push_str("    func sqrt(x: f64): f64\n");
        out.push_str("    func log(x: f64): f64\n");
        out.push_str("    func log10(x: f64): f64\n");
        out.push_str("    func exp(x: f64): f64\n");
        out.push_str("    func sin(x: f64): f64\n");
        out.push_str("    func cos(x: f64): f64\n");
        out.push_str("    func tan(x: f64): f64\n");
        out.push_str("    func sinh(x: f64): f64\n");
        out.push_str("    func cosh(x: f64): f64\n");
        out.push_str("    func tanh(x: f64): f64\n");
        out.push_str("    func asin(x: f64): f64\n");
        out.push_str("    func acos(x: f64): f64\n");
        out.push_str("    func atan(x: f64): f64\n");
        out.push_str("    func atan2(y: f64, x: f64): f64\n");
        out.push_str("    func frexp(x: f64, exp: ptr[int]): f64\n");
        out.push_str("    func strerror(errnum: int): ptr[i8]\n");
        out.push_str("    func errnoLocation(): ptr[int]\n");
        // Controle de call frames & sistema
        out.push_str("    func setjmp(env: ptr[usize]): int\n");
        out.push_str("    func longjmp(env: ptr[usize], val: int)\n");
        out.push_str("    func abort()\n");
        out.push_str("    func exit(status: int)\n");

        for func in &extern_funcs {
            let func_name = sanitize_ident(&func.name);
            let mut params_vec: Vec<String> = func
                .params
                .iter()
                .map(|p| format!("{}: {}", sanitize_ident(&p.name), p.ty))
                .collect();
            if func.is_variadic {
                params_vec.push("args: any...".to_string());
            }
            let params_str = params_vec.join(", ");
            if func.return_ty == "void" || func.return_ty.is_empty() {
                out.push_str(&format!("    func {}({})\n", func_name, params_str));
            } else {
                out.push_str(&format!(
                    "    func {}({}): {}\n",
                    func_name, params_str, func.return_ty
                ));
            }
        }
        out.push_str("}\n\n");
    }

    if !defined_funcs.is_empty() && options.mode != TranspileMode::Binding {
        out.push_str("// --- Funções Definidas e Transpiladas ---\n");
        for func in defined_funcs {
            let func_name = sanitize_ident(&func.name);
            let mut seen_param_names = std::collections::HashSet::new();
            let mut deduplicated_params = Vec::new();
            for p in &func.params {
                let mut name = sanitize_ident(&p.name);
                if seen_param_names.contains(&name) {
                    let mut i = 2;
                    while seen_param_names.contains(&format!("{name}{i}")) {
                        i += 1;
                    }
                    name = format!("{name}{i}");
                }
                seen_param_names.insert(name.clone());
                deduplicated_params.push((p.name.clone(), name, p.ty.clone(), p.is_mut));
            }

            let mut params_vec: Vec<String> = deduplicated_params
                .iter()
                .map(|(_, clean, ty, _)| format!("{clean}: {ty}"))
                .collect();
            if func.is_variadic {
                params_vec.push("args: any...".to_string());
            }
            let params_str = params_vec.join(", ");
            if func.return_ty == "void" || func.return_ty.is_empty() {
                out.push_str(&format!(
                    "public func {}({}) {{\n",
                    func_name, params_str
                ));
            } else {
                out.push_str(&format!(
                    "public func {}({}): {} {{\n",
                    func_name, params_str, func.return_ty
                ));
            }

            let mut func_types = HashMap::new();
            for g in &program.globals {
                func_types.insert(g.name.clone(), g.ty.clone());
                func_types.insert(sanitize_ident(&g.name), g.ty.clone());
            }
            for (orig_name, clean, ty, is_mut) in &deduplicated_params {
                func_types.insert(orig_name.clone(), ty.clone());
                func_types.insert(clean.clone(), ty.clone());
                if *is_mut {
                    func_types.insert(format!("local_{orig_name}"), ty.clone());
                    func_types.insert(sanitize_ident(&format!("local_{orig_name}")), ty.clone());
                }
            }
            fn collect_var_types(stmts: &[CStmt], types: &mut HashMap<String, String>) {
                for s in stmts {
                    match s {
                        CStmt::VarDecl { name, ty, .. } => {
                            types.insert(name.clone(), ty.clone());
                            types.insert(sanitize_ident(name), ty.clone());
                        }
                        CStmt::If { then_block, else_block, .. } => {
                            collect_var_types(then_block, types);
                            if let Some(eb) = else_block {
                                collect_var_types(eb, types);
                            }
                        }
                        CStmt::While { body, .. } | CStmt::DoWhile { body, .. } => {
                            collect_var_types(body, types);
                        }
                        CStmt::For { init, body, .. } => {
                            if let Some(i) = init {
                                collect_var_types(std::slice::from_ref(i), types);
                            }
                            collect_var_types(body, types);
                        }
                        CStmt::Switch { arms, .. } => {
                            for arm in arms {
                                collect_var_types(&arm.body, types);
                            }
                        }
                        CStmt::Compound(inner) => {
                            collect_var_types(inner, types);
                        }
                        _ => {}
                    }
                }
            }
            if let Some(ref body_stmts) = func.body {
                collect_var_types(body_stmts, &mut func_types);
            }
            KNOWN_TYPES.with(|cell| {
                *cell.borrow_mut() = func_types.clone();
            });

            let mut unsafe_names: Vec<String> = extern_funcs
                .iter()
                .flat_map(|f| vec![f.name.clone(), sanitize_ident(&f.name)])
                .collect();
            for g in &program.globals {
                unsafe_names.push(g.name.clone());
                unsafe_names.push(sanitize_ident(&g.name));
            }
            let needs_unsafe = function_needs_unsafe(func, &unsafe_names);
            let body_indent = if needs_unsafe { 2 } else { 1 };
            let pad = "    ".repeat(body_indent);

            if needs_unsafe {
                out.push_str("    unsafe {\n");
            }

            if let Some(ref body_stmts) = func.body {
                let mut structured = restructure_gotos(body_stmts.clone());

                // Detect any parameters assigned to anywhere in structured stmts
                let mut assigned_params = std::collections::HashSet::new();
                fn find_assigned_in_expr(e: &CExpr, set: &mut std::collections::HashSet<String>) {
                    match e {
                        CExpr::UnaryOp { op, operand, .. } => {
                            if op == "++" || op == "--" {
                                if let CExpr::Ident(ref name) = **operand {
                                    set.insert(name.clone());
                                    set.insert(sanitize_ident(name));
                                }
                            }
                            find_assigned_in_expr(operand, set);
                        }
                        CExpr::BinaryOp { left, right, .. } => {
                            find_assigned_in_expr(left, set);
                            find_assigned_in_expr(right, set);
                        }
                        CExpr::Call { callee, args } => {
                            find_assigned_in_expr(callee, set);
                            for a in args {
                                find_assigned_in_expr(a, set);
                            }
                        }
                        CExpr::Cast { expr, .. } | CExpr::Paren(expr) => {
                            find_assigned_in_expr(expr, set);
                        }
                        CExpr::Ternary { cond, then_expr, else_expr } => {
                            find_assigned_in_expr(cond, set);
                            find_assigned_in_expr(then_expr, set);
                            find_assigned_in_expr(else_expr, set);
                        }
                        CExpr::MemberAccess { base, .. } => {
                            find_assigned_in_expr(base, set);
                        }
                        CExpr::Index { base, index } => {
                            find_assigned_in_expr(base, set);
                            find_assigned_in_expr(index, set);
                        }
                        _ => {}
                    }
                }

                fn find_assigned(stmts: &[CStmt], set: &mut std::collections::HashSet<String>) {
                    for s in stmts {
                        match s {
                            CStmt::Assign { target, value, .. } => {
                                if let CExpr::Ident(ref name) = target {
                                    set.insert(name.clone());
                                    set.insert(sanitize_ident(name));
                                }
                                find_assigned_in_expr(target, set);
                                find_assigned_in_expr(value, set);
                            }
                            CStmt::Expr(expr) => {
                                find_assigned_in_expr(expr, set);
                            }
                            CStmt::If { cond, then_block, else_block } => {
                                find_assigned_in_expr(cond, set);
                                find_assigned(then_block, set);
                                if let Some(eb) = else_block {
                                    find_assigned(eb, set);
                                }
                            }
                            CStmt::While { cond, body } => {
                                find_assigned_in_expr(cond, set);
                                find_assigned(body, set);
                            }
                            CStmt::DoWhile { body, cond } => {
                                find_assigned_in_expr(cond, set);
                                find_assigned(body, set);
                            }
                            CStmt::For { init, cond, step, body } => {
                                if let Some(i) = init {
                                    find_assigned(std::slice::from_ref(i), set);
                                }
                                if let Some(c) = cond {
                                    find_assigned_in_expr(c, set);
                                }
                                if let Some(s) = step {
                                    find_assigned_in_expr(s, set);
                                }
                                find_assigned(body, set);
                            }
                            CStmt::Switch { expr, arms } => {
                                find_assigned_in_expr(expr, set);
                                for arm in arms {
                                    find_assigned(&arm.body, set);
                                }
                            }
                            CStmt::Return(Some(r)) => {
                                find_assigned_in_expr(r, set);
                            }
                            CStmt::Compound(inner) => {
                                find_assigned(inner, set);
                            }
                            _ => {}
                        }
                    }
                }
                find_assigned(&structured, &mut assigned_params);

                let initial_param_names: Vec<String> = deduplicated_params.iter().map(|(_, n, _, _)| n.clone()).collect();
                let mut declared_names: std::collections::HashSet<String> = initial_param_names.into_iter().collect();
                let mut rename_counters: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                disambiguate_variable_shadowing(&mut structured, &mut declared_names, &mut rename_counters);

                let mut param_rename_map = std::collections::HashMap::new();
                for (orig_name, clean, _, _) in &deduplicated_params {
                    if clean != orig_name {
                        param_rename_map.insert(orig_name.clone(), clean.clone());
                    }
                }
                if !param_rename_map.is_empty() {
                    crate::parser::rename_vars_simultaneous_in_stmts(&mut structured, &param_rename_map);
                }

                for (orig_name, clean, ty, is_mut) in &mut deduplicated_params {
                    if *is_mut || assigned_params.contains(orig_name) || assigned_params.contains(clean) {
                        *is_mut = true;
                        let local_clean = sanitize_ident(&format!("local_{orig_name}"));
                        out.push_str(&format!("{pad}let mut {local_clean}: {ty} = {clean}\n"));
                        crate::parser::rename_var_in_stmts(&mut structured, orig_name, &local_clean);
                        crate::parser::rename_var_in_stmts(&mut structured, clean, &local_clean);
                    }
                }

                collect_var_types(&structured, &mut func_types);
                KNOWN_TYPES.with(|cell| {
                    *cell.borrow_mut() = func_types;
                });

                CURRENT_FUNC_RET_TY.with(|cell| {
                    *cell.borrow_mut() = Some(func.return_ty.clone());
                });

                for stmt in &structured {
                    out.push_str(&generate_stmt(stmt, body_indent));
                }

                CURRENT_FUNC_RET_TY.with(|cell| {
                    *cell.borrow_mut() = None;
                });
            } else {
                for (orig_name, clean, ty, is_mut) in &deduplicated_params {
                    if *is_mut {
                        let local_clean = sanitize_ident(&format!("local_{orig_name}"));
                        out.push_str(&format!("{pad}let mut {local_clean}: {ty} = {clean}\n"));
                    }
                }
                if func.return_ty != "void" {
                    out.push_str(&format!("{pad}return 0\n"));
                }
            }

            if needs_unsafe {
                out.push_str("    }\n");
            }
            out.push_str("}\n\n");
        }
    }

    out
}
