// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod statistics;
use std::process::Command;

use statistics::*;
use tauri::{generate_context, generate_handler};

#[tauri::command]
fn tauri_get_msr_data() -> Result<Msr, ()> {
    match get_msr() {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("rs#{:?}", err);   
            Err(())
        },
    }
}


#[tauri::command]
fn tauri_get_nv() -> Result<NvStats, ()> {
    match get_nv() {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("rs#{:?}", err);   
            Err(())
        },
    }
}

#[tauri::command]
fn tauri_get_radeon() -> Result<RadeonStats, ()> {
    match get_radeon() {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("rs#{:?}", err);   
            Err(())
        },
    } 
}

#[tauri::command]
fn tauri_get_intel_gpu() -> Result<IgStats, ()> {
    match get_intel_gpu() {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("rs#{:?}", err);   
            Err(())
        },
    }
}

fn main() {
    let mut msr = Command::new("systemctl");
    msr.args(["start","msr_server.service"]);
    _ = msr.output();

    tauri::Builder::default()
        .invoke_handler(generate_handler![
            tauri_get_msr_data,
            tauri_get_nv,
            tauri_get_radeon,
            tauri_get_intel_gpu
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
