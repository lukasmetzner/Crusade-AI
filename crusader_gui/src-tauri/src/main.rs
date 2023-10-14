// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, params};
use serde::Serialize;

use std::thread;

use crusader_lib::extract_data;

#[derive(Debug, Default, Serialize)]
struct Resources {
    tick: u32,
    gold: u32,
    wood: u32,
    stone: u32,
    iron: u32,
    hops: u32,
    pitch: u32,
    ale: u32,
    flour: u32,
    peasants: u32,
    max_peasants: u32,
}

#[tauri::command]
fn get_resources() -> Resources {
    let conn = Connection::open("./db.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM resources ORDER BY tick DESC LIMIT 1").unwrap();
    stmt.query_row(params![], |row| {
        Ok(Resources {
            tick: row.get(0)?,
            gold: row.get(1)?,
            wood: row.get(2)?,
            stone: row.get(3)?,
            iron: row.get(4)?,
            hops: row.get(5)?,
            pitch: row.get(6)?,
            ale: row.get(7)?,
            flour: row.get(8)?,
            peasants: row.get(9)?,
            max_peasants: row.get(10)?,
        })
    }).unwrap()
}

fn main() {
    let handle = thread::spawn(|| {
        extract_data();
    });
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_resources])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    handle.join().unwrap();
}
