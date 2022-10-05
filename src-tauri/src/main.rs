use std::io::Write;
use std::fs::OpenOptions;
use std::path::Path;
// use std::io::prelude::*;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// #[tauri::command]
// fn save(data: &str) -> std::io::Result<()> {
//     // let mut file = File::create("data.txt");
//     // file::write("./data.txt", data).expect("error");
// }
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn save(num: &str, fact: &str) -> String {
    let mut data;
    let string = format!("facture : {} , facture number : {} \n",fact , num);
    if Path::new("./data.txt").exists() {
        data = OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();

        if let Err(e) = writeln!(data,"{}", string) {
            eprintln!("couldn't write to file {}", e);
        }
    } else {
        data = std::fs::File::create("data.txt").expect("error creating file");
        data.write_all(string.as_bytes()).expect("error writing ");
    }
    format!("your data has been saved! facture number : {} \n facture : {}", num, fact)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
