use clang_sys::*;
use std::ffi::CStr;

pub unsafe fn get_range_tokens(tu: CXTranslationUnit, range: CXSourceRange) -> Vec<String> {
    let mut tokens: *mut CXToken = std::ptr::null_mut();
    let mut num_tokens: std::os::raw::c_uint = 0;

    clang_tokenize(tu, range, &mut tokens, &mut num_tokens);
    let mut res = Vec::new();
    if !tokens.is_null() && num_tokens > 0 {
        for i in 0..num_tokens {
            let tok = *tokens.add(i as usize);
            let cx_str = clang_getTokenSpelling(tu, tok);
            let c_str = clang_getCString(cx_str);
            if !c_str.is_null() {
                res.push(CStr::from_ptr(c_str).to_string_lossy().into_owned());
            }
            clang_disposeString(cx_str);
        }
        clang_disposeTokens(tu, tokens, num_tokens);
    }
    res
}

pub unsafe fn get_cursor_tokens(cursor: CXCursor) -> Vec<String> {
    let tu = clang_Cursor_getTranslationUnit(cursor);
    let range = clang_getCursorExtent(cursor);
    get_range_tokens(tu, range)
}

pub unsafe fn get_cursor_token_string(cursor: CXCursor) -> String {
    get_cursor_tokens(cursor).join(" ")
}

pub unsafe fn get_cursor_spelling(cursor: CXCursor) -> String {

    let cx_str = clang_getCursorSpelling(cursor);
    let c_str = clang_getCString(cx_str);
    let s = if !c_str.is_null() {
        CStr::from_ptr(c_str).to_string_lossy().into_owned()
    } else {
        String::new()
    };
    clang_disposeString(cx_str);
    s
}

pub unsafe fn get_type_spelling(ty: CXType) -> String {
    let cx_str = clang_getTypeSpelling(ty);
    let c_str = clang_getCString(cx_str);
    let s = if !c_str.is_null() {
        CStr::from_ptr(c_str).to_string_lossy().into_owned()
    } else {
        String::new()
    };
    clang_disposeString(cx_str);
    // Remove qualificadores C como 'const ' e 'volatile '
    s.replace("const ", "").replace("volatile ", "")
}

/// Converte um identificador C para PascalCase, exigido para tipos do Arandu.
pub fn to_pascal_case(s: &str) -> String {
    if s.is_empty() {
        return "Type".to_string();
    }
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in s.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(ch.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    if result.is_empty() {
        "Type".to_string()
    } else {
        result
    }
}
pub unsafe fn c_type_to_arandu(ty: CXType) -> String {
    // Se for ponteiro, desreferencia e remove const da ponta
    if ty.kind == CXType_Pointer {
        let pointee = clang_getPointeeType(ty);
        if pointee.kind == CXType_Void {
            return "ptr[u8]".to_string();
        }
        if pointee.kind == CXType_FunctionProto || pointee.kind == CXType_FunctionNoProto {
            let fn_str = c_type_to_arandu(pointee);
            return format!("ptr[{fn_str}]");
        }
        if pointee.kind == CXType_Char_S || pointee.kind == CXType_Char_U || pointee.kind == CXType_SChar || pointee.kind == CXType_UChar {
            return "ptr[u8]".to_string();
        }
        let pointee_str = c_type_to_arandu(pointee);
        let clean_pointee = pointee_str.trim_start_matches("const ").trim();
        return format!("ptr[{clean_pointee}]");
    }

    match ty.kind {
        CXType_Void => "void".to_string(),
        CXType_Bool => "bool".to_string(),
        CXType_Char_U | CXType_UChar => "u8".to_string(),
        CXType_Char_S | CXType_SChar => "i8".to_string(),
        CXType_Short => "i16".to_string(),
        CXType_UShort => "u16".to_string(),
        CXType_Int => "int".to_string(),
        CXType_UInt => "uint".to_string(),
        CXType_Long => "i64".to_string(),
        CXType_ULong => "u64".to_string(),
        CXType_LongLong => "i64".to_string(),
        CXType_ULongLong => "u64".to_string(),
        CXType_Float => "f32".to_string(),
        CXType_Double => "f64".to_string(),
        CXType_FunctionProto | CXType_FunctionNoProto => {
            let ret_ty = clang_getResultType(ty);
            let num_args = clang_getNumArgTypes(ty);
            let mut args = Vec::new();
            if num_args > 0 {
                for i in 0..num_args {
                    let arg_ty = clang_getArgType(ty, i as u32);
                    let arg_str = if arg_ty.kind == CXType_ConstantArray
                        || arg_ty.kind == CXType_IncompleteArray
                        || arg_ty.kind == CXType_VariableArray
                    {
                        let elem = clang_getArrayElementType(arg_ty);
                        format!("ptr[{}]", c_type_to_arandu(elem))
                    } else {
                        c_type_to_arandu(arg_ty)
                    };
                    args.push(arg_str);
                }
            }
            let args_str = args.join(", ");
            if ret_ty.kind == CXType_Void {
                format!("func({args_str})")
            } else {
                let ret_str = c_type_to_arandu(ret_ty);
                format!("func({args_str}) {ret_str}")
            }
        }
        CXType_ConstantArray => {
            let size = clang_getArraySize(ty);
            let elem = clang_getArrayElementType(ty);
            let elem_str = c_type_to_arandu(elem);
            format!("[{size}]{elem_str}")
        }
        CXType_IncompleteArray | CXType_VariableArray => {
            let elem = clang_getArrayElementType(ty);
            let elem_str = c_type_to_arandu(elem);
            format!("ptr[{elem_str}]")
        }
        CXType_Record => {
            let name = get_type_spelling(ty);
            let clean_name = name
                .strip_prefix("struct ")
                .or_else(|| name.strip_prefix("union "))
                .unwrap_or(&name);
            if clean_name.contains("unnamed at") || clean_name.is_empty() {
                let decl = unsafe { clang_getTypeDeclaration(ty) };
                let children = unsafe { crate::parser::get_children(decl) };
                let field_names: Vec<String> = children.iter()
                    .filter(|c| c.kind == CXCursor_FieldDecl)
                    .map(|c| get_cursor_spelling(*c))
                    .collect();
                if field_names.iter().any(|f| f == "left") && field_names.iter().any(|f| f == "right") {
                    "PriorityStruct".to_string()
                } else if field_names.iter().any(|f| f == "pDesc") {
                    "SqliteErrDesc".to_string()
                } else {
                    let parent = unsafe { clang_getCursorSemanticParent(decl) };
                    let parent_raw = get_cursor_spelling(parent);
                    let parent_clean = parent_raw
                        .strip_prefix("struct ")
                        .or_else(|| parent_raw.strip_prefix("union "))
                        .unwrap_or(&parent_raw);
                    let prefix = if !parent_clean.is_empty()
                        && !parent_clean.contains("unnamed")
                        && !parent_clean.contains('(')
                        && !parent_clean.contains('/')
                        && !parent_clean.contains('\\')
                        && !parent_clean.contains('.')
                        && parent.kind != CXCursor_TranslationUnit
                    {
                        to_pascal_case(parent_clean)
                    } else {
                        String::new()
                    };

                    let suffix: String = if !field_names.is_empty() {
                        field_names.iter().take(3).map(|s| to_pascal_case(s)).collect::<Vec<_>>().join("")
                    } else {
                        let hash = unsafe { clang_hashCursor(decl) };
                        format!("{:x}", hash & 0xFFFF)
                    };

                    if !prefix.is_empty() {
                        format!("{prefix}_{suffix}")
                    } else {
                        format!("Anon_{suffix}")
                    }
                }
            } else {
                to_pascal_case(clean_name)
            }
        }
        CXType_Typedef => {
            let name = get_type_spelling(ty);
            match name.as_str() {
                "sig_atomic_t" => "int".to_string(),
                "size_t" | "uintptr_t" | "__uintptr_t" | "__size_t" => "usize".to_string(),
                "ptrdiff_t" | "intptr_t" | "__ptrdiff_t" | "__intptr_t" | "ssize_t" | "__ssize_t" | "__off_t" | "__off64_t" => "isize".to_string(),
                "uint8_t" => "u8".to_string(),
                "uint16_t" => "u16".to_string(),
                "uint32_t" => "u32".to_string(),
                "uint64_t" => "u64".to_string(),
                "int8_t" => "i8".to_string(),
                "int16_t" => "i16".to_string(),
                "int32_t" => "i32".to_string(),
                "int64_t" => "i64".to_string(),
                "mz_uint8" | "mz_uint8_t" => "u8".to_string(),
                "mz_uint16" | "mz_uint16_t" => "u16".to_string(),
                "mz_uint32" | "mz_uint32_t" | "mz_uint" => "u32".to_string(),
                "mz_uint64" | "mz_uint64_t" | "mz_ulong" => "u64".to_string(),
                "mz_int8" | "mz_int8_t" => "i8".to_string(),
                "mz_int16" | "mz_int16_t" => "i16".to_string(),
                "mz_int32" | "mz_int32_t" => "i32".to_string(),
                "mz_int64" | "mz_int64_t" => "i64".to_string(),
                "mz_bool" => "bool".to_string(),
                "IdxT" => "i64".to_string(),
                other => to_pascal_case(other),
            }
        }
        CXType_Enum => "int".to_string(),
        CXType_Elaborated => {
            let named = clang_Type_getNamedType(ty);
            c_type_to_arandu(named)
        }
        _ => {
            let name = get_type_spelling(ty);
            match name.as_str() {
                "size_t" | "uintptr_t" | "__uintptr_t" | "__size_t" => "usize".to_string(),
                "ptrdiff_t" | "intptr_t" | "__ptrdiff_t" | "__intptr_t" | "ssize_t" | "__ssize_t" | "__off_t" | "__off64_t" => "isize".to_string(),
                "uint8_t" | "__uint8_t" => "u8".to_string(),
                "uint16_t" | "__uint16_t" => "u16".to_string(),
                "uint32_t" | "__uint32_t" => "u32".to_string(),
                "uint64_t" | "__uint64_t" => "u64".to_string(),
                "int8_t" | "__int8_t" => "i8".to_string(),
                "int16_t" | "__int16_t" => "i16".to_string(),
                "int32_t" | "__int32_t" => "i32".to_string(),
                "int64_t" | "__int64_t" => "i64".to_string(),
                "" => "/* unknown_type */ int".to_string(),
                other => to_pascal_case(other),
            }
        }
    }
}
