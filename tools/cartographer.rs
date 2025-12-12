/// tools/cartographer.rs
/// Versão Inteligente: Encontra a raiz do projeto automaticamente.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // 1. Encontra a raiz do projeto (onde está o .git)
    let root = find_project_root().expect("❌ Não foi possível encontrar a raiz do projeto (.git)");
    println!("📍 Project Root found at: {}", root.display());

    // 2. Define as camadas
    let layers = vec!["00_nucleo", "01_core", "02_shell", "03_infra", "04_wiring", "_lab"];

    let mut global_map = String::from("# 🗺️ Project Atlas (Crystalline)\n\n");
    global_map.push_str("> Map of architectural layers and their active modules.\n\n");

    for layer in &layers {
        let layer_path = root.join(layer);

        if layer_path.exists() {
            global_map.push_str(&format!("## 📂 {}\n", layer));

            // Varre subpastas
            let entries = fs::read_dir(&layer_path)?;
            let mut subdirs: Vec<_> = entries.map(|e| e.unwrap().path()).collect();
            subdirs.sort();

            for sub in subdirs {
                if sub.is_dir() {
                    let sub_name = sub.file_name().unwrap().to_string_lossy();
                    // Ignora pastas irrelevantes
                    if sub_name.starts_with(".") || sub_name == "target" || sub_name == "templates" { continue; }

                    // Gera o mapa local
                    let generated = generate_module_map(&sub)?;

                    // Calcula caminho relativo para o link funcionar no Markdown
                    let rel_path = format!("{}/{}", layer, sub_name);

                    if generated {
                        global_map.push_str(&format!("- **[{}]** - [View Map]({}/_MAP.md)\n", sub_name, rel_path));
                    } else {
                        global_map.push_str(&format!("- **[{}]**\n", sub_name));
                    }
                }
            }
            global_map.push_str("\n");
        }
    }

    // Salva o mapa global na RAIZ encontrada
    fs::write(root.join("PROJECT_MAP.md"), global_map)?;
    println!("✅ Atlas updated: {}/PROJECT_MAP.md", root.display());
    Ok(())
}

fn find_project_root() -> io::Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Root not found"));
        }
    }
}

fn generate_module_map(dir: &Path) -> io::Result<bool> {
    let dir_name = dir.file_name().unwrap().to_string_lossy();
    let mut content = format!("# 🗺️ Context: {}\n\n| File | Purpose |\n|---|---|\n", dir_name);
    let mut has_relevant_files = false;

    let entries = fs::read_dir(dir)?;
    let mut paths: Vec<_> = entries.map(|r| r.unwrap().path()).collect();
    paths.sort();

    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy();

        if name == "_MAP.md" || name.starts_with(".") || name == "Cargo.toml" { continue; }

        if path.is_file() {
            if let Some(desc) = extract_doc_comment(&path) {
                content.push_str(&format!("| `{}` | {} |\n", name, desc));
                has_relevant_files = true;
            }
        }
    }

    if has_relevant_files {
        fs::write(dir.join("_MAP.md"), content)?;
        println!("   📝 Sub-map generated: {}", dir.display());
        return Ok(true);
    }

    // Limpeza: Se a pasta ficou vazia de código relevante, apaga o mapa antigo
    let _ = fs::remove_file(dir.join("_MAP.md"));
    Ok(false)
}

fn extract_doc_comment(path: &Path) -> Option<String> {
    let ext = path.extension().unwrap_or_default().to_string_lossy();
    // Tenta ler o arquivo (ignora se for binário/imagem)
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let t = line.trim();
            match ext.as_ref() {
                "rs" => if t.starts_with("//!") { return Some(t.replace("//!", "").trim().to_string()) },
                "md" if !t.starts_with("#") => return Some(t.to_string()),
                _ => {}
            }
        }
        return Some("-".to_string());
    }
    None
}
