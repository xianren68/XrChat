use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::thread;
use tauri::Window;
#[cfg(test)]
mod tests;
/// Tcp package struct.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct TcpPacket {
    length: u32,
    message_id: u32,
    header_length: u32,
    body_length: u32,
    header: Vec<u8>,
    body: Vec<u8>,
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

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 16 {
            return None;
        }

        let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let message_id = u32::from_be_bytes(bytes[4..8].try_into().unwrap());

        let header_length = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let body_length = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        if bytes.len() < 16 + header_length as usize + body_length as usize {
            return None;
        }
        let header = bytes[16..16 + header_length as usize].to_vec();
        let body = bytes
            [16 + header_length as usize..16 + header_length as usize + body_length as usize]
            .to_vec();

        return Some(Self {
            length,
            message_id,
            header_length,
            body_length,
            header,
            body,
        });
    }
}
#[derive(Serialize, Deserialize, Clone)]
struct Paylod {
    message_id: u32,
    data: Vec<u8>,
}
pub fn tcp_connect(window: Window) {
    let host = "127.0.0.1";
    let port = 8088;
    let mut stream = TcpStream::connect((host, port)).expect("Failed to connect");
    let mut write_stream = stream.try_clone().unwrap();
    let (tx, rx) = channel::<Vec<u8>>();
    window.listen("send", move |event| {
        let message = event.payload().unwrap();
        let msg: Paylod = serde_json::from_str(message).expect("Failed to parse message");
        let send_msg = TcpPacket {
            length: msg.data.len() as u32 + 14,
            message_id: msg.message_id,
            header_length: 2,
            body_length: msg.data.len() as u32,
            header: vec![123, 125], // because this function will not be used, so the header is hardcoded as '{}'.
            body: msg.data,
        };
        let mut bytes = Vec::new();
        bytes.extend(send_msg.to_bytes());
        tx.send(bytes).unwrap();
    });
    thread::spawn(move || loop {
        let mut bytes = Vec::new();
        match rx.recv() {
            Ok(data) => {
                bytes.extend(&data);
                write_stream
                    .write(bytes.as_slice())
                    .expect("Failed to write");
            }
            Err(_) => {
                break;
            }
        }
    });
    // read
    thread::spawn(move || loop {
        let message = read_data(&mut stream);
        window.emit("recv", message).unwrap();
    });
}

fn read_data(stream: &mut TcpStream) -> TcpPacket {
    let mut flag = 4;
    let mut length = [0, 0, 0, 0];
    let mut bytes = Vec::new();
    while bytes.len() < flag {
        let num = stream.read(&mut length).unwrap();
        bytes.extend(&length[0..num]);
    }
    // get message length
    let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    flag = length as usize + 4;

    while bytes.len() < flag {
        let mut buffer = [0; 1024];
        let num = stream.read(&mut buffer).unwrap();
        bytes.extend(&buffer[0..num]);
    }
    // get message
    let message = TcpPacket::from_bytes(&bytes);

    return message.unwrap();
}
