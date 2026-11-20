use crate::ast::*;

#[derive(Debug, Default)]
pub struct PointerUsage {
    pub has_arithmetic: bool,
    pub has_indexed_access: bool,
    pub has_field_write: bool,
    pub has_deref_write: bool,
    pub has_read: bool,
    pub has_null_check: bool,
    pub escapes: bool,
}

pub fn analyze_expr(expr: &CExpr, target_name: &str, usage: &mut PointerUsage) {
    match expr {
        CExpr::Ident(name) if name == target_name => {
            usage.has_read = true;
        }
        CExpr::BinaryOp { op, left, right } => {
            if op == "==" || op == "!=" {
                if is_ident(left, target_name) && is_null_expr(right) {
                    usage.has_null_check = true;
                } else if is_ident(right, target_name) && is_null_expr(left) {
                    usage.has_null_check = true;
                }
            } else if op == "+" || op == "-" {
                if is_ident(left, target_name) || is_ident(right, target_name) {
                    usage.has_arithmetic = true;
                }
            }
            analyze_expr(left, target_name, usage);
            analyze_expr(right, target_name, usage);
        }
        CExpr::UnaryOp { op, operand, .. } => {
            if is_ident(operand, target_name) {
                if op == "++" || op == "--" {
                    usage.has_arithmetic = true;
                } else if op == "!" {
                    usage.has_null_check = true;
                } else if op == "*" {
                    usage.has_read = true;
                }
            }
            analyze_expr(operand, target_name, usage);
        }
        CExpr::MemberAccess { base, .. } => {
            if is_ident(base, target_name) {
                usage.has_read = true;
            }
            analyze_expr(base, target_name, usage);
        }
        CExpr::Index { base, index } => {
            if is_ident(base, target_name) {
                usage.has_indexed_access = true;
            }
            analyze_expr(base, target_name, usage);
            analyze_expr(index, target_name, usage);
        }
        CExpr::Call { callee, args } => {
            if let CExpr::Ident(ref name) = **callee {
                if name == "free" || name == "mz_free" {
                    for arg in args {
                        if is_ident(arg, target_name) {
                            usage.has_arithmetic = true; // Impede elevação para ref, mantém ptr[T] para free()
                        }
                    }
                }
            }
            analyze_expr(callee, target_name, usage);
            for arg in args {
                analyze_expr(arg, target_name, usage);
            }
        }
        CExpr::Cast { expr, .. } => analyze_expr(expr, target_name, usage),
        CExpr::Paren(inner) => analyze_expr(inner, target_name, usage),
        CExpr::Ternary { cond, then_expr, else_expr } => {
            analyze_expr(cond, target_name, usage);
            analyze_expr(then_expr, target_name, usage);
            analyze_expr(else_expr, target_name, usage);
        }
        _ => {}
    }
}

pub fn analyze_stmt(stmt: &CStmt, target_name: &str, usage: &mut PointerUsage) {
    match stmt {
        CStmt::Assign { target, value, .. } => {
            if is_ident(value, target_name) {
                // Se atribuído para uma struct externa ou ponteiro indireto, escapa
                match target {
                    CExpr::MemberAccess { .. } | CExpr::Index { .. } => {
                        usage.escapes = true;
                    }
                    _ => {}
                }
            }
            // Checar se o alvo é *target_name ou target_name.field ou target_name[i]
            match target {
                CExpr::UnaryOp { op, operand, .. } if op == "*" => {
                    if is_ident(operand, target_name) {
                        usage.has_deref_write = true;
                    }
                }
                CExpr::MemberAccess { base, .. } => {
                    if is_ident(base, target_name) {
                        usage.has_field_write = true;
                    }
                }
                CExpr::Index { base, .. } => {
                    if is_ident(base, target_name) {
                        usage.has_indexed_access = true;
                        usage.has_deref_write = true;
                    }
                }
                _ => {}
            }
            analyze_expr(target, target_name, usage);
            analyze_expr(value, target_name, usage);
        }
        CStmt::VarDecl { init, .. } => {
            if let Some(init_expr) = init {
                if is_ident(init_expr, target_name) {
                    // Inicialização de outra variável com o ponteiro
                }
                analyze_expr(init_expr, target_name, usage);
            }
        }
        CStmt::Expr(expr) => analyze_expr(expr, target_name, usage),
        CStmt::Return(opt_expr) => {
            if let Some(expr) = opt_expr {
                if is_ident(expr, target_name) {
                    usage.escapes = true;
                }
                analyze_expr(expr, target_name, usage);
            }
        }
        CStmt::If { cond, then_block, else_block } => {
            analyze_expr(cond, target_name, usage);
            for s in then_block {
                analyze_stmt(s, target_name, usage);
            }
            if let Some(eb) = else_block {
                for s in eb {
                    analyze_stmt(s, target_name, usage);
                }
            }
        }
        CStmt::While { cond, body } => {
            analyze_expr(cond, target_name, usage);
            for s in body {
                analyze_stmt(s, target_name, usage);
            }
        }
        CStmt::DoWhile { body, cond } => {
            for s in body {
                analyze_stmt(s, target_name, usage);
            }
            analyze_expr(cond, target_name, usage);
        }
        CStmt::For { init, cond, step, body } => {
            if let Some(i) = init {
                analyze_stmt(i, target_name, usage);
            }
            if let Some(c) = cond {
                analyze_expr(c, target_name, usage);
            }
            if let Some(st) = step {
                analyze_expr(st, target_name, usage);
            }
            for s in body {
                analyze_stmt(s, target_name, usage);
            }
        }
        CStmt::Switch { expr, arms } => {
            analyze_expr(expr, target_name, usage);
            for arm in arms {
                for s in &arm.body {
                    analyze_stmt(s, target_name, usage);
                }
            }
        }
        CStmt::Compound(stmts) => {
            for s in stmts {
                analyze_stmt(s, target_name, usage);
            }
        }
        _ => {}
    }
}

fn is_ident(expr: &CExpr, target_name: &str) -> bool {
    if let CExpr::Ident(name) = expr {
        name == target_name
    } else {
        false
    }
}

fn is_null_expr(expr: &CExpr) -> bool {
    match expr {
        CExpr::IntLiteral(0) => true,
        CExpr::Ident(name) => name == "NULL" || name == "nil",
        CExpr::Cast { expr, .. } | CExpr::Paren(expr) => is_null_expr(expr),
        _ => false,
    }
}

/// Aplica o pass de Semantic Lifting em todo o programa C.
/// Promove `ptr[T]` para `ref T`, `mut ref T` ou `[]T` com base nas regras da RFC 0002.
pub fn apply_semantic_lifting(program: &mut CProgram) {
    for func in &mut program.functions {
        let body_stmts = match func.body {
            Some(ref stmts) => stmts,
            None => continue,
        };

        for param in &mut func.params {
            // Verifica se o tipo é um ponteiro: ptr[T]
            if let Some(inner_ty) = extract_pointee_type(&param.ty) {
                let mut usage = PointerUsage::default();
                for stmt in body_stmts {
                    analyze_stmt(stmt, &param.name, &mut usage);
                }

                // Regras da RFC 0002 / Dual-Locus:
                if usage.has_null_check {
                    // Se o ponteiro sofre teste de nulidade em C, no Arandu ele é opcional ptr[T]?
                    // Referências (ref T / mut ref T) não podem ser nulas.
                    param.ty = format!("ptr[{inner_ty}]?");
                    continue;
                }

                if usage.has_arithmetic {
                    // Fallback para ptr[T] cru (delimitado por unsafe)
                    continue;
                }

                if usage.has_indexed_access || usage.has_deref_write {
                    // Elevado para fatia segura
                    let slice_prefix = if usage.has_field_write || usage.has_deref_write {
                        "mut []"
                    } else {
                        "[]"
                    };
                    param.ty = format!("{slice_prefix}{inner_ty}");
                } else if usage.has_field_write {
                    // Elevado para referência mutável exclusiva (OSSA mut ref T)
                    param.ty = format!("mut ref {inner_ty}");
                } else {
                    // Elevado para referência imutável segura (OSSA ref T)
                    param.ty = format!("ref {inner_ty}");
                }
            }
        }

        lift_local_allocations(func);
        check_nullable_returns(func);
    }
}

fn extract_malloc_size(expr: &CExpr) -> Option<usize> {
    match expr {
        CExpr::Call { callee, args } => {
            if let CExpr::Ident(ref name) = **callee {
                if (name == "malloc" || name == "mz_malloc") && args.len() == 1 {
                    if let CExpr::IntLiteral(v) = args[0] {
                        if v > 0 && v <= 65536 {
                            return Some(v as usize);
                        }
                    }
                } else if name == "calloc" && args.len() == 2 {
                    if let (CExpr::IntLiteral(n), CExpr::IntLiteral(sz)) = (&args[0], &args[1]) {
                        let total = n * sz;
                        if total > 0 && total <= 65536 {
                            return Some(total as usize);
                        }
                    }
                }
            }
            None
        }
        CExpr::Cast { expr, .. } | CExpr::Paren(expr) => extract_malloc_size(expr),
        _ => None,
    }
}

fn filter_free_stmts(stmts: &mut Vec<CStmt>, target_name: &str) {
    stmts.retain(|s| {
        if let CStmt::Expr(CExpr::Call { callee, args }) = s {
            if let CExpr::Ident(ref name) = **callee {
                if (name == "free" || name == "mz_free") && args.len() == 1 && is_ident(&args[0], target_name) {
                    return false; // Remove chamada a free redundante/perigosa
                }
            }
        }
        true
    });

    for s in stmts.iter_mut() {
        match s {
            CStmt::If { then_block, else_block, .. } => {
                filter_free_stmts(then_block, target_name);
                if let Some(eb) = else_block {
                    filter_free_stmts(eb, target_name);
                }
            }
            CStmt::While { body, .. } | CStmt::DoWhile { body, .. } | CStmt::For { body, .. } => {
                filter_free_stmts(body, target_name);
            }
            CStmt::Compound(inner) => {
                filter_free_stmts(inner, target_name);
            }
            _ => {}
        }
    }
}

/// Eleva alocações locais (malloc/free) não-escapadas para arrays na stack
pub fn lift_local_allocations(func: &mut CFunction) {
    let body = match func.body {
        Some(ref mut b) => b,
        None => return,
    };

    let mut allocations: Vec<(String, usize)> = Vec::new();
    for stmt in body.iter() {
        if let CStmt::VarDecl { name, init: Some(init_expr), .. } = stmt {
            if let Some(sz) = extract_malloc_size(init_expr) {
                let mut usage = PointerUsage::default();
                for s in body.iter() {
                    analyze_stmt(s, name, &mut usage);
                }
                if !usage.escapes && sz <= 4096 {
                    allocations.push((name.clone(), sz));
                }
            }
        }
    }

    for (var_name, sz) in allocations {
        for stmt in body.iter_mut() {
            if let CStmt::VarDecl { name, ty, init, is_mut } = stmt {
                if name == &var_name {
                    *ty = format!("[{sz}]u8");
                    *init = Some(CExpr::Raw("[0]".to_string()));
                    *is_mut = true;
                }
            }
        }
        filter_free_stmts(body, &var_name);
    }
}

pub fn check_nullable_returns(func: &mut CFunction) {
    if !func.return_ty.starts_with("ptr[") || func.return_ty.ends_with('?') {
        return;
    }
    let mut has_null_return = false;
    if let Some(ref body) = func.body {
        for stmt in body {
            check_stmt_null_return(stmt, &mut has_null_return);
            if has_null_return {
                break;
            }
        }
    }
    if has_null_return {
        func.return_ty.push('?');
    }
}

fn check_stmt_null_return(stmt: &CStmt, found: &mut bool) {
    if *found {
        return;
    }
    match stmt {
        CStmt::Return(Some(expr)) => {
            if is_null_expr(expr) {
                *found = true;
            }
        }
        CStmt::If { then_block, else_block, .. } => {
            for s in then_block {
                check_stmt_null_return(s, found);
            }
            if let Some(eb) = else_block {
                for s in eb {
                    check_stmt_null_return(s, found);
                }
            }
        }
        CStmt::While { body, .. } | CStmt::DoWhile { body, .. } => {
            for s in body {
                check_stmt_null_return(s, found);
            }
        }
        CStmt::For { body, .. } => {
            for s in body {
                check_stmt_null_return(s, found);
            }
        }
        CStmt::Compound(stmts) => {
            for s in stmts {
                check_stmt_null_return(s, found);
            }
        }
        _ => {}
    }
}

fn extract_pointee_type(ty: &str) -> Option<String> {
    if ty.starts_with("ptr[") && ty.ends_with(']') {
        Some(ty[4..ty.len() - 1].trim().to_string())
    } else {
        None
    }
}
