#![no_std]
#![feature(more_qualified_paths)]
#![feature(iter_next_chunk)]

use mac_parser::*;

use bin_utils::*;

use ether_type::*;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
/// An EthernetII header as described in IEEE 802.3
/// ```
/// use ethernet::{Ethernet2Header};
/// use bin_utils::{ReadFixed, WriteFixed};
/// use ether_type::EtherType;
/// 
/// let bytes = [
///     0x00, 0x80, 0x41, 0xff, 0xf0, 0x0d, // dst
///     0x00, 0x80, 0x41, 0xba, 0xbe, 0xff, // src
///     0x86, 0xdd // EtherType
/// ];
/// let eth2header = Ethernet2Header::from_bytes(&bytes).unwrap();
/// assert_eq!(eth2header, Ethernet2Header{
///     dst: [0x00, 0x80, 0x41, 0xff, 0xf0, 0x0d].into(),
///     src: [0x00, 0x80, 0x41, 0xba, 0xbe, 0xff].into(),
///     ether_type: EtherType::IPv6
/// });
/// assert_eq!(eth2header.to_bytes(), bytes);
/// ```
pub struct Ethernet2Header {
    pub dst: MACAddress,

    pub src: MACAddress,

    pub ether_type: EtherType
}
impl ReadFixed<14> for Ethernet2Header {
    fn from_bytes(data: &[u8; 14]) -> Result<Self, ParserError> {
        let mut data = data.iter().copied();
        Ok(Self {
            dst: MACAddress::from_bytes(&data.next_chunk().unwrap()).unwrap(),
            src: MACAddress::from_bytes(&data.next_chunk().unwrap()).unwrap(),
            ether_type: EtherType::from_bytes(&data.next_chunk().unwrap()).unwrap()
        })
    }
}
impl WriteFixed<14> for Ethernet2Header {
    fn to_bytes(&self) -> [u8; 14] {
        let mut bytes = [0x00; 14];
        bytes[0..6].copy_from_slice(&self.dst.to_bytes());
        bytes[6..12].copy_from_slice(&self.src.to_bytes());
        bytes[12..14].copy_from_slice(&self.ether_type.to_bytes());

        bytes
    }
}
