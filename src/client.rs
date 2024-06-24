use std::{io::{Read, Write}, net::TcpStream};
use crate::shared::Packet;
use macroquad::prelude::*;

mod shared;

#[macroquad::main("Online test")]
async fn main() {
    println!("Connecting");
    let stream = TcpStream::connect("127.0.0.1:6666").unwrap();
    let mut socket = Connection { stream, buf: [0; 1024] };
    println!("Wait for packet");
    let (id, mut other_player) = if let Some(Packet::OnJoin(id, other_player)) = socket.recv_packet() {
        (id, other_player)
    } else {
        panic!("Could not get on join packet")
    };
    println!("{:?}", other_player);
    socket.set_no_block();
    loop {
        let delta_time = get_frame_time();

        clear_background(BLACK);

        

        while let Some(packet) = socket.recv_packet() {
            match packet {
                Packet::Joined => {
                    other_player = true;
                }
                Packet::Left => {
                    other_player = false;
                }
                _ => unreachable!()
            }
        }

        next_frame().await
    }
}

struct Connection {
    stream: TcpStream,
    buf: [u8; 1024],
}

impl Connection {
    fn send_packet(&mut self, packet: Packet) {
        let data = packet.to_slice();
        self.stream.write_all(data.as_slice()).expect("Couldnt send packet");
    }

    fn recv_packet(&mut self) -> Option<Packet> {
        match self.stream.read(&mut self.buf) {
            Ok(_) => {
                let packet = Packet::from_slice(&self.buf);
                self.buf.fill(0);
                Some(packet)
            }
            Err(_) => None,
        }
    }

    fn set_no_block(&mut self) {
        self.stream.set_nonblocking(true).unwrap();
    }
}