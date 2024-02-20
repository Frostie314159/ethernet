#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use ether_type::EtherType;
use mac_parser::*;

use scroll::{
    ctx::{MeasureWith, SizeWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// An EthernetII header as described in IEEE 802.3
/// ```
/// use ethernet::Ethernet2Header;
/// use ether_type::EtherType;
///
/// let bytes = [
///     0x00, 0x80, 0x41, 0xff, 0xf0, 0x0d, // dst
///     0x00, 0x80, 0x41, 0xba, 0xbe, 0xff, // src
///     0x86, 0xdd // EtherType
/// ];
/// let eth2header = Ethernet2Header::from_fixed_bytes(bytes);
/// assert_eq!(eth2header, Ethernet2Header{
///     dst: [0x00, 0x80, 0x41, 0xff, 0xf0, 0x0d].into(),
///     src: [0x00, 0x80, 0x41, 0xba, 0xbe, 0xff].into(),
///     ether_type: EtherType::IPv6
/// });
/// assert_eq!(eth2header.to_fixed_bytes(), bytes);
/// ```
pub struct Ethernet2Header {
    /// Destination
    pub dst: MACAddress,

    /// Source
    pub src: MACAddress,

    /// EtherType of the payload
    pub ether_type: EtherType,
}
impl Ethernet2Header {
    /// The header length in bytes.
    ///
    /// Useful if you want to define a fixed array.
    pub const HEADER_LENGTH: usize = 14;

    /// Conveniece method, which calls scroll internally.
    ///
    /// This method can only fail if the provided data was too short.
    /// # Returns
    /// - `Some` If the data was long enough.
    /// - `None` If the data was too short.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.pread(0).ok()
    }

    /// Deserialize the struct from a fixed array.
    ///
    /// Allows skipping internal checks.
    pub fn from_fixed_bytes(bytes: [u8; Self::HEADER_LENGTH]) -> Self {
        Self::from_bytes(bytes.as_slice()).unwrap()
    }

    /// Conveniece method, which calls scroll internally.
    ///
    /// This method can only fail if the provided data was too short.
    /// # Returns
    /// - `Some` If the buffer was long enough.
    /// - `None` If the buffer was too short.
    pub fn to_bytes(self, buf: &mut [u8]) -> Option<()> {
        buf.pwrite(self, 0).ok().map(|_| ())
    }

    /// Serializes the struct into a fixed array.
    ///
    /// This method is infallible.
    pub fn to_fixed_bytes(self) -> [u8; Self::HEADER_LENGTH] {
        let mut buf = [0x00; Self::HEADER_LENGTH];

        // It's impossible for this unwrap to panic, since the length will always be correct.
        buf.as_mut_slice().pwrite(self, 0).unwrap();

        buf
    }
}
impl SizeWith for Ethernet2Header {
    fn size_with(_ctx: &()) -> usize {
        Self::HEADER_LENGTH
    }
}
impl TryFromCtx<'_> for Ethernet2Header {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'_ [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let dst = from.gread(&mut offset)?;
        let src = from.gread(&mut offset)?;
        let ether_type = EtherType::from_bits(from.gread_with(&mut offset, Endian::Big)?);

        Ok((
            Self {
                dst,
                src,
                ether_type,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for Ethernet2Header {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        buf.gwrite(self.dst, &mut offset)?;
        buf.gwrite(self.src, &mut offset)?;
        buf.gwrite_with(
            self.ether_type.into_bits(),
            &mut offset,
            Endian::Big,
        )?;

        Ok(offset)
    }
}

pub struct Ethernet2Frame<'a> {
    pub header: Ethernet2Header,

    pub payload: &'a [u8],
}
impl Ethernet2Frame<'_> {
    /// Total length in bytes.
    ///
    /// This being an associated item, allows us to make it constant. This enables the compiler to perform more inlining.
    pub const fn length_in_bytes(&self) -> usize {
        Ethernet2Header::HEADER_LENGTH + self.payload.len()
    }

    /// Conveniece method, which calls scroll internally.
    ///
    /// This method can only fail if the provided data was too short.
    /// # Returns
    /// - `Some` If the buffer was long enough.
    /// - `None` If the buffer was too short.
    pub fn from_bytes<'a>(bytes: &'a [u8]) -> Option<Ethernet2Frame<'a>> {
        bytes.pread(0).ok()
    }

    /// Conveniece method, which calls scroll internally.
    ///
    /// This method can only fail if the provided data was too short.
    /// # Returns
    /// - `Some` If the buffer was long enough.
    /// - `None` If the buffer was too short.
    pub fn to_bytes(self, buf: &mut [u8]) -> Option<()> {
        buf.pwrite(self, 0).ok().map(|_| ())
    }
}
impl MeasureWith<()> for Ethernet2Frame<'_> {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.length_in_bytes()
    }
}
impl<'a> TryFromCtx<'a> for Ethernet2Frame<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        if from.len() <= 14 {
            return Err(scroll::Error::BadInput {
                size: 0,
                msg: "Ethernet frame has no body.",
            });
        }
        let mut offset = 0;

        let header = from.gread(&mut offset)?;
        let payload = from.gread_with(&mut offset, from.len() - Ethernet2Header::HEADER_LENGTH)?;

        Ok((Self { header, payload }, offset))
    }
}
impl TryIntoCtx for Ethernet2Frame<'_> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        buf.gwrite(self.header, &mut offset)?;
        buf.gwrite(self.payload, &mut offset)?;

        Ok(offset)
    }
}
#[cfg(feature = "alloc")]
pub struct OwnedEthernet2Frame {
    pub header: Ethernet2Header,

    pub payload: Vec<u8>
}
#[cfg(feature = "alloc")]
impl TryFromCtx<'_> for OwnedEthernet2Frame {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'_ [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        let ethernet_frame = from.gread::<Ethernet2Frame<'_>>(&mut offset)?;
        Ok((
            Self {
                header: ethernet_frame.header,
                payload: ethernet_frame.payload.to_vec()
            },
            offset
        ))
    }
}
#[cfg(feature = "alloc")]
impl TryIntoCtx for OwnedEthernet2Frame {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let ethernet_frame = Ethernet2Frame {
            header: self.header,
            payload: self.payload.as_slice(),
        };
        buf.pwrite(ethernet_frame, 0)
    }
}
