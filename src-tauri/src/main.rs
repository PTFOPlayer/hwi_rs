// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod statistics;
use std::process::Command;

use statistics::{get_cpu, CpuData};
use tauri::generate_context;

#[tauri::command]
fn tauri_get_cpu() -> String {
    match get_cpu() {
        Ok(res) =>  format!("{}", res.power),
        Err(err) => format!("error") 
    }
}

fn main() {
    let mut msr = Command::new("systemctl");
    msr.arg("start").arg("msr_server.service");
    _ = msr.output();

    //keeping this for teamplate
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tauri_get_cpu])
        .run(generate_context!())
        .expect("error while running tauri application");

    ()
}
