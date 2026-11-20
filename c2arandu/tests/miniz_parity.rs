use std::process::Command;
use std::path::PathBuf;

#[test]
fn test_miniz_header_transpile_and_check() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let miniz_h = manifest_dir.join("benchmarks/miniz/miniz.h");
    let output_aru = manifest_dir.join("benchmarks/miniz/miniz_h.aru");
    let arandu_bin = PathBuf::from("/home/bruno/Documentos/Desenvolvimento/Arandu-Lang/target/debug/arandu_cli");

    // 1. Executa o c2arandu no miniz.h
    let status = Command::new(env!("CARGO_BIN_EXE_c2arandu"))
        .arg(&miniz_h)
        .arg("-I")
        .arg(manifest_dir.join("benchmarks/miniz"))
        .arg("-o")
        .arg(&output_aru)
        .status()
        .expect("falha ao executar c2arandu");
    assert!(status.success(), "c2arandu falhou ao transpilar miniz.h");

    // 2. Valida com arandu check
    if arandu_bin.exists() {
        let check_status = Command::new(&arandu_bin)
            .arg("check")
            .arg(&output_aru)
            .status()
            .expect("falha ao executar arandu_cli check");
        assert!(check_status.success(), "arandu check falhou em miniz_h.aru");
    }
}

#[test]
fn test_miniz_adler32_parity_jit() {
    let arandu_bin = PathBuf::from("/home/bruno/Documentos/Desenvolvimento/Arandu-Lang/target/debug/arandu_cli");
    if !arandu_bin.exists() {
        return;
    }

    let test_code = r#"
public func mz_adler32(adler: u64, ptr_val: []u8, buf_len: usize): u64 {
    let mut i: usize = 0
    let mut s1: u64 = adler & 65535
    let mut s2: u64 = adler >> 16
    let mut block_len: usize = buf_len % 5552
    let mut offset: usize = 0
    let mut remaining: usize = buf_len
    if ptr_val.len() == 0 {
        return 1
    }
    while remaining != 0 {
        i = 0
        while (i + 7) < block_len {
            s1 = s1 + (ptr_val[offset + 0] as u64)
            s2 = s2 + s1
            s1 = s1 + (ptr_val[offset + 1] as u64)
            s2 = s2 + s1
            s1 = s1 + (ptr_val[offset + 2] as u64)
            s2 = s2 + s1
            s1 = s1 + (ptr_val[offset + 3] as u64)
            s2 = s2 + s1
            s1 = s1 + (ptr_val[offset + 4] as u64)
            s2 = s2 + s1
            s1 = s1 + (ptr_val[offset + 5] as u64)
            s2 = s2 + s1
            s1 = s1 + (ptr_val[offset + 6] as u64)
            s2 = s2 + s1
            s1 = s1 + (ptr_val[offset + 7] as u64)
            s2 = s2 + s1
            i = i + 8
            offset = offset + 8
        }
        while i < block_len {
            s1 = s1 + (ptr_val[offset] as u64)
            s2 = s2 + s1
            i = i + 1
            offset = offset + 1
        }
        s1 = s1 % 65521
        s2 = s2 % 65521
        remaining = remaining - block_len
        block_len = 5552
    }
    return (s2 << 16) + s1
}

public func main(): int {
    let msg: []u8 = "World".asBytes()
    let checksum = mz_adler32(1, msg, 5)
    // Adler-32 de "World" com seed 1 é exatamente 101057033
    if checksum == 101057033 {
        return 0
    }
    return 1
}
"#;

    let tmp_file = std::env::temp_dir().join("test_miniz_adler_parity.aru");
    std::fs::write(&tmp_file, test_code).unwrap();

    let run_status = Command::new(&arandu_bin)
        .arg("run")
        .arg(&tmp_file)
        .status()
        .expect("falha ao executar arandu run");
    assert!(run_status.success(), "mz_adler32 JIT parity verification falhou");
}

#[test]
fn test_miniz_crc32_parity_jit() {
    let arandu_bin = PathBuf::from("/home/bruno/Documentos/Desenvolvimento/Arandu-Lang/target/debug/arandu_cli");
    if !arandu_bin.exists() {
        return;
    }

    let test_code = r#"
public func mz_crc32(crc: u64, ptr_val: []u8, buf_len: usize): u64 {
    let s_crc32: [16]u32 = [
        0, 498536548, 997073096, 651767980, 1994146192, 1802195444, 1303535960, 1342533948,
        3988292384, 4027552580, 3604390888, 3412177804, 2607071920, 2262029012, 2685067896, 3183342108
    ]
    let mut crcu32: u32 = crc as u32
    if ptr_val.len() == 0 {
        return 0
    }
    crcu32 = ~crcu32
    let mut i: usize = 0
    while i < buf_len {
        let b = ptr_val[i] as u32
        let idx1 = ((crcu32 & 15) ^ (b & 15)) as usize
        crcu32 = (crcu32 >> 4) ^ s_crc32[idx1]
        let idx2 = ((crcu32 & 15) ^ (b >> 4)) as usize
        crcu32 = (crcu32 >> 4) ^ s_crc32[idx2]
        i = i + 1
    }
    return (~crcu32) as u64
}

public func main(): int {
    let msg: []u8 = "Hello World".asBytes()
    let crc = mz_crc32(0, msg, 11)
    // CRC-32 de "Hello World" (standard IEEE 802.3 / zlib) é 1243066710
    if crc == 1243066710 {
        return 0
    }
    return 1
}
"#;

    let tmp_file = std::env::temp_dir().join("test_miniz_crc32_parity.aru");
    std::fs::write(&tmp_file, test_code).unwrap();

    let run_status = Command::new(&arandu_bin)
        .arg("run")
        .arg(&tmp_file)
        .status()
        .expect("falha ao executar arandu run");
    assert!(run_status.success(), "mz_crc32 JIT parity verification falhou");
}
