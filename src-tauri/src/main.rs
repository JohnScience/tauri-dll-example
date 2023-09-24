#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::ffi::{c_int, c_void};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn print_hello() {
    unsafe {
        let lib = libloading::Library::new("UnityPlayer.dll").unwrap();
        type UnityMainFunc = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int);
        let func: libloading::Symbol<UnityMainFunc> = lib.get(b"UnityMain").unwrap();

        let hinstance = std::ptr::null_mut();
        let hprevinstance = std::ptr::null_mut();
        let lpcmdline = std::ptr::null_mut();
        let nshowcmd = 1;

        func(hinstance, hprevinstance, lpcmdline, nshowcmd);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, print_hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
