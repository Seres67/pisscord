// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver},
        Mutex,
    }, time::Duration,
};

use tauri::{State, Manager};

struct AppState {
    stream: TcpStream,
    rx: Receiver<String>,
}

struct Stream(Mutex<AppState>);

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String
}

#[tauri::command]
fn get_message(state: State<Stream>) -> String {
    let rx = &state.0.lock().unwrap().rx;

    let msg = rx.try_recv();
    let str = match msg {
        Ok(msg) => msg,
        Err(_) => return String::from(""),
    };
    println!("outside loop {str}");
    std::mem::drop(rx);
    str
}

#[tauri::command]
fn print_text(text: &str, stream: State<Stream>) {
    let mut s = &stream.0.lock().unwrap().stream;
    let peer = s.peer_addr().unwrap();
    println!("sending {text} to {:?}", peer);
    s.write_all(format!("{text}\r\n").as_bytes()).unwrap();
    s.flush().unwrap();
    std::mem::drop(s);
}

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:3001") {
        Ok(stream) => stream,
        Err(_) => panic!("Couldn't connect to server")
    };
    let (tx, rx) = mpsc::channel();
    let state = Stream(Mutex::new(AppState {
        stream: stream.try_clone().unwrap(),
        rx,
    }));

    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async move {
                println!("starting loop");
                let mut str = vec![0; 1024];
                loop {
                    stream.read(&mut str).expect("?");
                    let str2 = String::from_utf8(str.clone()).unwrap();
                    tx.send(str2).unwrap();
                    // app.emit_all("message-received", Payload { message: str2 }).unwrap();
                }
            });
            Ok(())
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![print_text, get_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
