use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::net::TcpStream;
use std::io::{Read,Write};
use serde::{Serialize,Deserialize};
use tauri::Window;
/// Tcp package struct.
#[derive(Serialize, Deserialize,Clone)]
struct TcpPacket {
    length:u32,
    message_id:u32,
    header_length:u32,
    body_length:u32,
    header:Vec<u8>,
    body:Vec<u8>,
}

impl TcpPacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.length.to_be_bytes());
        bytes.extend(&self.message_id.to_be_bytes());
        bytes.extend(&self.header_length.to_be_bytes());
        bytes.extend(&self.body_length.to_be_bytes());
        bytes.extend(&self.header);
        bytes.extend(&self.body);
        return bytes;
    }
    
    pub fn from_bytes(bytes:&[u8]) -> Option<Self>{
        if bytes.len() < 16 {
            return None
        }
        
        let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let message_id = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        
        let header_length = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let body_length = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        if bytes.len() < 16+header_length as usize+body_length as usize {
            return None
        }
        let header = bytes[16..16+header_length as usize].to_vec();
        let body = bytes[16+header_length as usize..16+header_length as usize+body_length as usize].to_vec();
        
        return Some(Self {
            length,
            message_id,
            header_length,
            body_length,
            header,
            body,
        })
    }
}

pub fn tcp_connect(window: Window) {
    let host = "127.0.0.1";
    let port = 8088;
    let stream = TcpStream::connect((host, port)).expect("Failed to connect");
    let stream = Arc::new(Mutex::new(stream));
    // write
    let write_stream = stream.clone();
    let (tx, rx) = channel::<Vec<u8>>();
    window.listen("send", move |event| {
        let message = event.payload().unwrap();
        let mut bytes = Vec::new();
        bytes.extend(message.as_bytes());
        tx.send(bytes).unwrap();
    });
    thread::spawn( move || {
        
        loop {
            let mut bytes = Vec::new();
            match rx.recv() {
                Ok(data) => {
                    bytes.extend(&data);
                    write_stream.lock().unwrap().write(&bytes).unwrap();
                },
                Err(_) => {
                    break;
                }
            }
        }
    });
    // read
    let read_stream = stream.clone();
    thread::spawn( move|| {
        loop {
            let message = read_data(read_stream.lock().unwrap().try_clone().unwrap());
            window.emit("recv", message).unwrap();
        };
    });
}

fn read_data(mut stream:TcpStream)-> TcpPacket{
    let mut flag = 4;
    let mut length = [0,0,0,0];
    let mut bytes = Vec::new();
    while bytes.len() < flag {
        let num = stream.read(&mut length).unwrap();
        bytes.extend(&length[0..num]);
    }
    // get message length
    let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    flag = length as usize + 4;
    
    while bytes.len() < flag {
        let mut buffer = [0;1024];
        let num = stream.read(&mut buffer).unwrap();
        bytes.extend(&buffer[0..num]);
    }
    // get message
    let message = TcpPacket::from_bytes(&bytes);
    
    return message.unwrap();
}