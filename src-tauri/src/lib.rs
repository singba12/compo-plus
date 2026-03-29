// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use hex;
use machineid_rs::{Encryption, IdBuilder};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_hwid() -> String {
    // 1️⃣ Génération d’un identifiant matériel stable
    let raw_id = IdBuilder::new(Encryption::SHA256)
        .add_component(machineid_rs::HWIDComponent::SystemID)
        .build("compo-app")
        .unwrap_or_else(|_| "UNKNOWN".to_string());

    // 2️⃣ Hash supplémentaire (sécurité)
    let mut hasher = Sha256::new();
    hasher.update(raw_id.as_bytes());
    let result = hasher.finalize();

    // 3️⃣ Encodage hexadécimal propre
    hex::encode(result)
}

#[tauri::command]
async fn save_pdf_to_downloads(
    app: AppHandle,
    filename: String,
    bytes: Vec<u8>,
    open_after: bool,
) -> Result<(), String> {

    // 1️⃣ Résoudre le dossier Téléchargements
    let download_dir: PathBuf = app
        .path()
        .resolve("", BaseDirectory::Download)
        .map_err(|e| format!("Erreur résolution dossier Téléchargements : {}", e))?;

    // 2️⃣ S'assurer que le dossier existe (corrige os error 2)
    fs::create_dir_all(&download_dir)
        .map_err(|e| format!("Erreur création dossier Téléchargements : {}", e))?;

    // 3️⃣ Nettoyer le nom du fichier
    let safe_filename = if filename.ends_with(".pdf") {
        filename
    } else {
        format!("{}.pdf", filename)
    };

    // 4️⃣ Construire le chemin final
    let mut final_path = download_dir.join(&safe_filename);

    // 5️⃣ Générer un nom unique si le fichier existe déjà
    if final_path.exists() {

        let stem = Path::new(&safe_filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("document");

        let ext = Path::new(&safe_filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("pdf");

        let mut i = 1;

        loop {
            let candidate = format!("{} ({}).{}", stem, i, ext);
            let candidate_path = download_dir.join(&candidate);

            if !candidate_path.exists() {
                final_path = candidate_path;
                break;
            }

            i += 1;
        }
    }

    // 6️⃣ Écriture du PDF
    fs::write(&final_path, bytes)
        .map_err(|e| format!("Erreur écriture PDF : {}", e))?;

    // 7️⃣ Ouvrir automatiquement le fichier si demandé
    if open_after {
        app.opener()
            .open_path(final_path.to_string_lossy().to_string(), None::<String>)
            .map_err(|e| format!("Erreur ouverture PDF : {}", e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_pdf_to_downloads,
            get_hwid
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}