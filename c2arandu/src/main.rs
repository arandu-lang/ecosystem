use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

use c2arandu::{CodegenOptions, ParserOptions, TranspileMode, generate_arandu, parse_c_file};

#[derive(Parser, Debug)]
#[command(
    name = "c2arandu",
    author = "Arandu Language Team",
    version = "0.1.0",
    about = "Transpilador de C para Arandu seguro e idiomático com suporte a Dual-Locus"
)]
struct Args {
    /// Arquivo C de entrada (.c ou .h)
    #[arg(required = true)]
    input: PathBuf,

    /// Arquivo de saída (.aru). Se omitido, imprime no stdout
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Modo de operação: transpile (código nativo), binding (cabeçalho FFI) ou auto (detectado pela extensão)
    #[arg(long, value_enum, default_value_t = TranspileMode::Auto)]
    mode: TranspileMode,

    /// Desativa a elevação semântica OSSA (mantém ponteiros crus ptr[T])
    #[arg(long)]
    no_lifting: bool,

    /// Diretórios de include do C (-I /path/to/include)
    #[arg(short = 'I', long = "include")]
    includes: Vec<PathBuf>,

    /// Nome do módulo Arandu gerado
    #[arg(short = 'm', long = "module")]
    module: Option<String>,

    /// Flags adicionais passadas para o compilador Clang
    #[arg(last = true)]
    clang_flags: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut c_flags = vec!["-std=c11".to_string()];

    // Autodescoberta do resource-dir do Clang (onde moram stdbool.h, stdint.h etc)
    if let Ok(output) = std::process::Command::new("clang")
        .arg("-print-resource-dir")
        .output()
    {
        if output.status.success() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                let trimmed = path.trim();
                if !trimmed.is_empty() {
                    c_flags.push("-resource-dir".to_string());
                    c_flags.push(trimmed.to_string());
                }
            }
        }
    }

    for inc in &args.includes {
        c_flags.push(format!("-I{}", inc.display()));
    }
    c_flags.extend(args.clang_flags);

    let parser_opts = ParserOptions {
        c_flags,
        main_file_only: true,
    };

    eprintln!("🔍 Parseando arquivo C com Clang: {}", args.input.display());
    let mut program = parse_c_file(&args.input, &parser_opts)
        .with_context(|| format!("Falha ao parsear '{}'", args.input.display()))?;

    eprintln!(
        "✨ Extraído com sucesso: {} structs, {} enums, {} typedefs, {} funções",
        program.structs.len(),
        program.enums.len(),
        program.typedefs.len(),
        program.functions.len()
    );

    if !args.no_lifting {
        eprintln!("🧠 Aplicando elevação semântica OSSA (RFC 0002)...");
        c2arandu::apply_semantic_lifting(&mut program);
    }

    let effective_mode = match args.mode {
        TranspileMode::Auto => {
            if args.input.extension().and_then(|ext| ext.to_str()) == Some("h") {
                TranspileMode::Binding
            } else {
                TranspileMode::Transpile
            }
        }
        other => other,
    };

    let codegen_opts = CodegenOptions {
        mode: effective_mode,
        emit_extern_block: true,
        module_name: args.module,
    };

    let arandu_code = generate_arandu(&program, &codegen_opts);

    if let Some(out_path) = args.output {
        fs::write(&out_path, &arandu_code)
            .with_context(|| format!("Falha ao escrever em '{}'", out_path.display()))?;
        eprintln!("🎉 Código Arandu gerado com sucesso em: {}", out_path.display());
    } else {
        println!("{arandu_code}");
    }

    Ok(())
}
