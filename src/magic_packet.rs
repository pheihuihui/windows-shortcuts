// https://github.com/TeemuRemes/wake-on-lan-rust/blob/master/src/lib.rs

use std::net::{Ipv4Addr, ToSocketAddrs, UdpSocket};

pub struct MagicPacket {
    magic_bytes: [u8; 102],
}

impl MagicPacket {
    pub fn new(mac_address: &[u8; 6]) -> MagicPacket {
        let mut magic_bytes: [u8; 102];

        unsafe {
            magic_bytes = std::mem::uninitialized();

            let mut src: *const u8 = &MAGIC_BYTES_HEADER[0];
            let mut dst: *mut u8 = &mut magic_bytes[0];
            dst.copy_from_nonoverlapping(src, 6);

            src = &mac_address[0];
            dst = dst.offset(6);
            dst.copy_from_nonoverlapping(src, 6);

            let src: *const u8 = dst;
            dst = dst.offset(6);
            dst.copy_from_nonoverlapping(src, 6);

            dst = dst.offset(6);
            dst.copy_from_nonoverlapping(src, 12);

            dst = dst.offset(12);
            dst.copy_from_nonoverlapping(src, 24);

            dst = dst.offset(24);
            dst.copy_from_nonoverlapping(src, 48);
        }

        MagicPacket { magic_bytes }
    }

    pub fn send(&self) -> std::io::Result<()> {
        self.send_to(
            (Ipv4Addr::new(255, 255, 255, 255), 9),
            (Ipv4Addr::new(0, 0, 0, 0), 0),
        )
    }

    pub fn send_to<A: ToSocketAddrs>(&self, to_addr: A, from_addr: A) -> std::io::Result<()> {
        let socket = UdpSocket::bind(from_addr)?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.magic_bytes, to_addr)?;

        Ok(())
    }

}

const MAGIC_BYTES_HEADER: [u8; 6] = [0xFF; 6];
