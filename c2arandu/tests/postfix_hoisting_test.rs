use std::process::Command;
use std::fs;

#[test]
fn test_postfix_hoisting() {
    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("test_inc.c");
    let aru_file = temp_dir.join("test_inc.aru");

    let c_code = r#"
void ext_call(int a);

int test_effects(int x, int* arr) {
    int local = x++;
    arr[x++] = 10;
    int y = x++;
    ext_call(x++);
    return x++;
}
"#;
    fs::write(&c_file, c_code).expect("falha ao escrever c_file");

    let status = Command::new(env!("CARGO_BIN_EXE_c2arandu"))
        .arg(&c_file)
        .arg("-o")
        .arg(&aru_file)
        .status()
        .expect("falha ao rodar c2arandu");
    assert!(status.success(), "c2arandu falhou");

    let generated = fs::read_to_string(&aru_file).expect("falha ao ler aru_file");
    println!("Generated:\n{}", generated);

    // Deve conter mutações de pós-incremento hoisteados (com localX devido à mutabilização de parâmetro)
    assert!(generated.contains("localX = localX + 1"), "deve conter mutações de pós-incremento hoisteados");
    assert!(generated.contains("let _ret_val = localX"), "deve conter let _ret_val no return com pós-incremento");
}
