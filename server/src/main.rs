// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

pub mod digraph;
pub mod error;
pub mod file_utils;
pub mod prelude;
pub mod runner;
pub mod scanner;
pub mod static_analysis;
mod syntax;
mod tests;
pub mod transpiler;

#[macro_use]
extern crate lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::fs;

use crate::digraph::address::Addressable;
use crate::digraph::event::KeyboardEvent;
use crate::digraph::parser::{Node, Parser};
use crate::digraph::state::CursorState;
use crate::runner::compile;

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

// This runs a Rattle script at a specified path and returns output of the script
#[tauri::command]
fn run_code(code: &str, path: &str) -> String {
    let part1 = compile(code.to_string(), path.to_string()).unwrap();
    let output1 = std::process::Command::new("python")
        .arg("../linalg.py")
        .output()
        .expect("failed to run")
        .stdout;
    format!("{}: {}", part1, std::str::from_utf8(&output1).unwrap())
}

#[tauri::command]
fn parse_file(source: String) -> Vec<Node> {
    let mut parser = Parser::new(source).unwrap();
    let mut nodes = parser.parse().unwrap();
    (&mut nodes[..]).fill_addr();
    nodes
}

#[tauri::command]
fn handle_event(
    event: String,
    mut payload: CursorState,
    value: Option<String>,
) -> (bool, String, CursorState) {
    let Ok(e): Result<KeyboardEvent, _> = serde_json::from_str(&event) else {
        panic!("Failed to parse keyboardEvent");
    };

    if let Some(value) = value {
        match payload.update_value(value) {
            Ok(_) => {}
            Err(e) => return (false, e.to_string(), payload),
        }
    }

    let (succeeded, err) = match e.parse_command(&mut payload) {
        Ok(_) => (true, "".into()),
        Err(e) => (false, e.to_string()),
    };

    (succeeded, err, payload)
}

#[tauri::command]
fn save_note(note: String, mut payload: CursorState) -> CursorState {
    payload.save_note(note);
    payload
}

#[tauri::command]
fn fetch_note(payload: CursorState) -> String {
    payload.fetch_field(|node| node.note.clone())
}

#[tauri::command]
fn fetch_err(payload: CursorState) -> String {
    payload.fetch_field(|node| node.err.as_ref().map(|e| e.to_string()))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_file_hierarchy,
            run_code,
            parse_file,
            handle_event,
            save_note,
            fetch_note,
            fetch_err,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
