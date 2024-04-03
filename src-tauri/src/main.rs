// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod error;
pub mod file_utils;
pub mod prelude;
pub mod runner;
pub mod scanner;
mod syntax;
mod tests;
pub mod transpiler;

#[macro_use]
extern crate lazy_static;
use crate::runner::compile;
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum FileType {
    File(File),
    Folder(Folder),
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Folder {
    name: String,
    children: Vec<FileType>,
}

#[tauri::command]
fn get_file_hierarchy(root_path: &str) -> FileType {
    let root_name = root_path.to_string();

    let mut children = Vec::new();

    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    let file = FileType::File(File { name: file_name });
                    children.push(file);
                } else if path.is_dir() {
                    let folder = get_file_hierarchy(path.to_str().unwrap());
                    children.push(folder);
                }
            }
        }
    }

    FileType::Folder(Folder {
        name: root_name,
        children,
    })
}

#[tauri::command]
fn run_code(code: &str, path: &str) -> String {
    return compile(code.to_string(), path.to_string()).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_file_hierarchy, run_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
