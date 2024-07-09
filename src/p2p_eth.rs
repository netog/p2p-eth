pub mod secret;
pub mod hash_mac;
mod ecies;
mod error;


use bytes::{Bytes, BytesMut};

use crate::p2p_eth::hash_mac::HashMac;
pub struct Handshake {

}

impl Handshake{
    pub fn new() -> Self {

    }

    pub fn auth(&mut self) -> BytesMut {
    }

}