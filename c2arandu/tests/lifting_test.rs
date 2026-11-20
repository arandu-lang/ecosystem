use c2arandu::ast::*;
use c2arandu::lifting::*;

#[test]
fn test_lifting_pointer_to_ref_and_mut_ref() {
    let func = CFunction {
        name: "process_point".to_string(),
        return_ty: "void".to_string(),
        params: vec![
            CParam {
                name: "reader".to_string(),
                ty: "ptr[Point]".to_string(),
                is_mut: false,
            },
            CParam {
                name: "writer".to_string(),
                ty: "ptr[Point]".to_string(),
                is_mut: false,
            },
        ],
        body: Some(vec![
            // Leitura de reader: reader.x
            CStmt::Expr(CExpr::MemberAccess {
                base: Box::new(CExpr::Ident("reader".to_string())),
                member: "x".to_string(),
                is_arrow: true,
            }),
            // Escrita em writer: writer.y = 10
            CStmt::Assign {
                target: CExpr::MemberAccess {
                    base: Box::new(CExpr::Ident("writer".to_string())),
                    member: "y".to_string(),
                    is_arrow: true,
                },
                op: "=".to_string(),
                value: CExpr::IntLiteral(10),
            },
        ]),
        is_definition: true,
        is_variadic: false,
    };

    let mut program = CProgram {
        functions: vec![func],
        ..Default::default()
    };

    apply_semantic_lifting(&mut program);

    let lifted_func = &program.functions[0];
    assert_eq!(lifted_func.params[0].ty, "ref Point", "reader deve ser promovido a ref Point");
    assert_eq!(lifted_func.params[1].ty, "mut ref Point", "writer deve ser promovido a mut ref Point");
}

#[test]
fn test_lifting_nullable_pointer_keeps_optional_ptr() {
    let func = CFunction {
        name: "maybe_null".to_string(),
        return_ty: "void".to_string(),
        params: vec![
            CParam {
                name: "p".to_string(),
                ty: "ptr[Node]".to_string(),
                is_mut: false,
            },
        ],
        body: Some(vec![
            CStmt::If {
                cond: CExpr::BinaryOp {
                    op: "==".to_string(),
                    left: Box::new(CExpr::Ident("p".to_string())),
                    right: Box::new(CExpr::Ident("NULL".to_string())),
                },
                then_block: vec![CStmt::Return(None)],
                else_block: None,
            },
        ]),
        is_definition: true,
        is_variadic: false,
    };

    let mut program = CProgram {
        functions: vec![func],
        ..Default::default()
    };

    apply_semantic_lifting(&mut program);

    let lifted_func = &program.functions[0];
    assert_eq!(lifted_func.params[0].ty, "ptr[Node]?", "ponteiro checado com NULL deve virar ptr[Node]?");
}

#[test]
fn test_lifting_local_malloc_free_to_stack_buffer() {
    let func = CFunction {
        name: "temporary_buffer".to_string(),
        return_ty: "int".to_string(),
        params: vec![],
        body: Some(vec![
            CStmt::VarDecl {
                name: "buf".to_string(),
                ty: "ptr[u8]".to_string(),
                init: Some(CExpr::Call {
                    callee: Box::new(CExpr::Ident("malloc".to_string())),
                    args: vec![CExpr::IntLiteral(256)],
                }),
                is_mut: true,
            },
            CStmt::Assign {
                target: CExpr::Index {
                    base: Box::new(CExpr::Ident("buf".to_string())),
                    index: Box::new(CExpr::IntLiteral(0)),
                },
                op: "=".to_string(),
                value: CExpr::IntLiteral(42),
            },
            CStmt::Expr(CExpr::Call {
                callee: Box::new(CExpr::Ident("free".to_string())),
                args: vec![CExpr::Ident("buf".to_string())],
            }),
            CStmt::Return(Some(CExpr::IntLiteral(0))),
        ]),
        is_definition: true,
        is_variadic: false,
    };

    let mut program = CProgram {
        functions: vec![func],
        ..Default::default()
    };

    apply_semantic_lifting(&mut program);

    let lifted_func = &program.functions[0];
    let body = lifted_func.body.as_ref().unwrap();

    // 1. buf deve ter sido transformado em [256]u8
    if let CStmt::VarDecl { name, ty, .. } = &body[0] {
        assert_eq!(name, "buf");
        assert_eq!(ty, "[256]u8", "malloc local deve virar array de stack [256]u8");
    } else {
        panic!("esperava VarDecl na primeira instrução");
    }

    // 2. free(buf) deve ter sido removido!
    let has_free = body.iter().any(|s| {
        if let CStmt::Expr(CExpr::Call { callee, .. }) = s {
            if let CExpr::Ident(ref name) = **callee {
                return name == "free";
            }
        }
        false
    });
    assert!(!has_free, "chamada a free() em alocação local deve ser eliminada");
}
