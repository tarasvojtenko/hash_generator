// hash_generator.rs — Rust версия

use sha2::{Sha512, Sha256, Sha384, Sha1, Digest};
use md5::Md5;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use hex;
use base64;

fn compute_hash(data: &[u8], algo: &str) -> String {
    match algo {
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        "sha384" => {
            let mut hasher = Sha384::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        _ => panic!("Неподдерживаемый алгоритм"),
    }
}

fn hash_file(filename: &str, algo: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let total = file.metadata()?.len();
    let mut buffer = [0; 8192];
    let mut processed = 0;
    match algo {
        "sha512" => {
            let mut hasher = Sha512::new();
            loop {
                let n = file.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
                processed += n as u64;
                if total > 1024 * 1024 {
                    let percent = (processed as f64 / total as f64) * 100.0;
                    eprint!("\r⏳ Прогресс: {:.1}%", percent);
                }
            }
            if total > 1024 * 1024 { eprintln!(); }
            Ok(hex::encode(hasher.finalize()))
        }
        _ => {
            // аналогично для других алгоритмов, но для краткости используем compute_hash на всём файле
            // (для больших файлов может быть неэффективно, но для демонстрации сойдёт)
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            Ok(compute_hash(&data, algo))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut algo = "sha512".to_string();
    let mut format = "hex".to_string();
    let mut file_path = None;
    let mut output = None;
    let mut compare = None;
    let mut input = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--algo" | "-a" => {
                algo = args[i+1].clone();
                i += 2;
            }
            "--format" | "-f" => {
                format = args[i+1].clone();
                i += 2;
            }
            "--file" | "-F" => {
                file_path = Some(args[i+1].clone());
                i += 2;
            }
            "--output" | "-o" => {
                output = Some(args[i+1].clone());
                i += 2;
            }
            "--compare" | "-c" => {
                compare = Some(args[i+1].clone());
                i += 2;
            }
            _ => {
                if !args[i].starts_with("-") {
                    input = Some(args[i].clone());
                }
                i += 1;
            }
        }
    }

    println!("\x1b[36m🔐 Hash Generator (Rust)\x1b[0m");
    println!("Алгоритм: {}", algo.to_uppercase());

    let hex_digest = if let Some(ref fpath) = file_path {
        println!("📂 Хеширование файла: {}", fpath);
        hash_file(fpath, &algo)?
    } else if let Some(ref text) = input {
        println!("📝 Входные данные: {}", text);
        compute_hash(text.as_bytes(), &algo)
    } else {
        println!("📝 Чтение из STDIN (Ctrl+D для окончания)");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if buffer.is_empty() {
            eprintln!("\x1b[33m⚠️ Пустой ввод.\x1b[0m");
            std::process::exit(1);
        }
        compute_hash(buffer.as_bytes(), &algo)
    };

    let result = if format == "base64" {
        let bytes = hex::decode(&hex_digest)?;
        base64::encode(&bytes)
    } else {
        hex_digest
    };

    println!("\x1b[32mХеш ({}):\x1b[0m", format);
    println!("{}", result);

    if let Some(ref expected) = compare {
        if result == *expected {
            println!("\x1b[32m✅ Хеши совпадают!\x1b[0m");
        } else {
            println!("\x1b[31m❌ Хеши не совпадают!\x1b[0m");
        }
    }

    if let Some(ref outfile) = output {
        let mut f = File::create(outfile)?;
        writeln!(f, "{}", result)?;
        println!("\x1b[32m💾 Сохранено в {}\x1b[0m", outfile);
    }

    Ok(())
}
