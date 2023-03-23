// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod statistics;
use std::process::Command;

// keeping this for teamplate
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//#[tauri::command]
//fn greet(name: &str) -> String {
//    format!("Hello, {}! You've been greeted from Rust!", name)
//}

fn main() {
    let mut msr = Command::new("systemctl");
    msr.arg("start").arg("msr_server.service");
    _ = msr.output();

    // keeping this for teamplate
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
