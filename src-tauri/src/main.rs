// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod statistics;
use std::process::Command;

use statistics::*;
use tauri::{generate_context, generate_handler};

#[tauri::command]
fn tauri_get_cpu() -> Result<CpuData, ()> {
    match get_cpu() {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("rs#{:?}", err);   
            Err(())
        },
    }
}

#[tauri::command]
fn tauri_get_memory() -> Result<MemData, ()> {
    match get_mem() {
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
    msr.arg("start").arg("msr_server.service");
    _ = msr.output();

    tauri::Builder::default()
        .invoke_handler(generate_handler![
            tauri_get_cpu,
            tauri_get_memory,
            tauri_get_nv,
            tauri_get_radeon,
            tauri_get_intel_gpu
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
