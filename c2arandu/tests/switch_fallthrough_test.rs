use std::process::Command;
use std::fs;

#[test]
fn test_switch_fallthrough_synthesis() {
    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("test_switch_ft.c");
    let aru_file = temp_dir.join("test_switch_ft.aru");

    let c_code = r#"
int compute(int kind) {
    int res = 0;
    switch (kind) {
        case 10:
        case 20:
            res = 1;
            break;
        case 30:
            res = 2;
            /* fallthrough */
        case 40:
            res += 10;
            break;
        default:
            res = -1;
            break;
    }
    return res;
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
    assert!(generated.contains("10 | 20 =>"), "deve agrupar labels 10 | 20");
    assert!(generated.contains("30 =>"), "deve ter arm 30");
    assert!(generated.contains("40 =>"), "deve ter arm 40");

    // Verifica que o arm 30 recebeu os statements do fallthrough (res = 2 e res += 10)
    let idx_30 = generated.find("30 =>").expect("deve encontrar '30 =>'");
    let idx_40 = generated.find("40 =>").expect("deve encontrar '40 =>'");
    let slice_30 = &generated[idx_30..idx_40];
    assert!(slice_30.contains("res = 2"), "arm 30 deve conter res = 2");
    assert!(slice_30.contains("res += 10"), "arm 30 deve herdar res += 10 por fallthrough");
}
