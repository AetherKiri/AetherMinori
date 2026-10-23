use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;

use aether_minori_runtime::script::{Language, Program, Statement};

fn main() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let mut language = Language::default();
    let mut paths: Vec<PathBuf> = Vec::new();
    while let Some(argument) = args.next() {
        let text = argument.to_string_lossy().into_owned();
        if text == "--language" {
            let value = args.next().ok_or("--language requires a code")?;
            language = Language::from_code(&value.to_string_lossy())
                .ok_or_else(|| format!("unknown language code {value:?}"))?;
        } else {
            paths.push(PathBuf::from(argument));
        }
    }
    if paths.is_empty() {
        return Err("usage: minori-script [--language j|e] SCRIPT.sc [...]".into());
    }

    let mut commands = BTreeMap::<String, usize>::new();
    let mut statement_count = 0usize;
    for path in &paths {
        let data = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
        let source = decode(&data).ok_or_else(|| format!("{}: undecodable script", path.display()))?;
        let program = Program::parse_with_language(&source, language)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        statement_count += program.statements.len();
        for statement in program.statements {
            if let Statement::Command { kind, .. } = statement {
                *commands.entry(kind.name().to_string()).or_default() += 1;
            }
        }
    }

    println!(
        "scripts={} statements={} commands={} language={}",
        paths.len(),
        statement_count,
        commands.values().sum::<usize>(),
        language.code()
    );
    for (name, count) in commands {
        println!(".{name}	{count}");
    }
    Ok(())
}

fn decode(bytes: &[u8]) -> Option<String> {
    let (shift_jis, _, sjis_errors) = encoding_rs::SHIFT_JIS.decode(bytes);
    if !sjis_errors {
        return Some(shift_jis.into_owned());
    }
    let (gbk, _, gbk_errors) = encoding_rs::GBK.decode(bytes);
    if !gbk_errors {
        return Some(gbk.into_owned());
    }
    None
}
