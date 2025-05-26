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
use crate::digraph::state::Canvas;
use static_analysis::debugger::Debugger;
use static_analysis::ident::IDGraph;

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
fn parse_file(source: String) -> Vec<Node> {
    let mut parser = Parser::new(source).unwrap();
    let mut nodes = parser.parse().unwrap();
    (&mut nodes[..]).fill_addr();
    nodes
}

#[tauri::command]
fn handle_event(
    event: String,
    mut payload: Canvas,
    value: Option<String>,
) -> (bool, String, Canvas) {
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
fn save_note(note: String, mut payload: Canvas) -> Canvas {
    payload.save_note(note);
    payload
}

#[tauri::command]
fn fetch_note(payload: Canvas) -> String {
    payload.fetch_field(|node| node.note.clone())
}

#[tauri::command]
fn fetch_err(payload: Canvas) -> String {
    payload.fetch_field(|node| node.err.as_ref().map(|e| e.to_string()))
}

#[tauri::command]
fn sync_idents(payload: Canvas) -> IDGraph {
    let idg = IDGraph::from_state(&payload);
    idg.populate_valid_idents();
    idg
}

#[tauri::command]
fn runWalkthrough(mut debugger_: Debugger, mut idg: IDGraph) -> (Debugger, String, IDGraph, i8) {
    let (expl, action) = debugger_.explain(&mut idg);
    let mut exitcode = 0;
    if let Ok(fin) = action.execute(&mut debugger_) {
        if fin {
            exitcode = 1;
        }
    } else {
        exitcode = -1;
    }
    idg.populate_valid_idents();
    (debugger_, expl, idg, exitcode)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_file_hierarchy,
            parse_file,
            handle_event,
            save_note,
            fetch_note,
            fetch_err,
            sync_idents,
            runWalkthrough,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
