// https://github.com/TeemuRemes/wake-on-lan-rust/blob/master/src/lib.rs

use std::{
    mem::MaybeUninit,
    net::{Ipv4Addr, ToSocketAddrs, UdpSocket},
};

use crate::constants::APP_CONFIG;

pub struct MagicPacket {
    magic_bytes: [u8; 102],
}

impl MagicPacket {
    pub fn new(mac_address: &[u8; 6]) -> MagicPacket {
        let magic_bytes: [u8; 102];

        let mut arr: [MaybeUninit<u8>; 102] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, element) in arr.iter_mut().enumerate() {
            if i < 6 {
                *element = MaybeUninit::new(0xff);
            } else {
                let u = i % 6;
                let tmp = mac_address[u];
                *element = MaybeUninit::new(tmp);
            }
        }
        unsafe {
            magic_bytes = std::mem::transmute::<_, [u8; 102]>(arr);
        }

        MagicPacket { magic_bytes }
    }

    #[allow(unused)]
    pub fn print_sth(&self) {
        println!("{:?}", self.magic_bytes);
    }

    pub fn send(&self) -> std::io::Result<()> {
        let local = APP_CONFIG.get().unwrap().server_addr;
        self.send_to(
            (Ipv4Addr::new(255, 255, 255, 255), 9),
            (Ipv4Addr::new(local[0], local[1], local[2], local[3]), 0),
        )
    }

    pub fn send_to<A: ToSocketAddrs>(&self, to_addr: A, from_addr: A) -> std::io::Result<()> {
        let socket = UdpSocket::bind(from_addr)?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.magic_bytes, to_addr)?;

        Ok(())
    }
}
