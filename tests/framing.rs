extern crate typed_codec;

use quickcheck_macros::quickcheck;
use typed_codec::*;

enum LengthHeaderCodec {}

impl<T> Decode<&mut T, Result<u32, Box<dyn std::error::Error>>> for LengthHeaderCodec
where
    T: std::io::Read,
{
    fn decode(reader: &mut T) -> Result<u32, Box<dyn std::error::Error>> {
        let mut buf = [0; 4];

        reader.read_exact(&mut buf)?;

        let length = u32::from_be_bytes(buf);

        Ok(length)
    }
}

impl<T> ContextualEncode<&u32, &mut T, Result<(), Box<dyn std::error::Error>>> for LengthHeaderCodec
where
    T: std::io::Write,
{
    fn encode(length: &u32, writer: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        writer.write_all(&length.to_be_bytes()).map_err(Into::into)
    }
}

enum PayloadCodec {}

impl<T> ContextualDecode<&u32, &mut T, Result<Vec<u8>, Box<dyn std::error::Error>>> for PayloadCodec
where
    T: std::io::Read,
{
    fn decode(ctx: &u32, reader: &mut T) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buf = vec![0; *ctx as usize];

        reader.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl<T> ContextualEncode<&Vec<u8>, &mut T, Result<(), Box<dyn std::error::Error>>> for PayloadCodec
where
    T: std::io::Write,
{
    fn encode(value: &Vec<u8>, writer: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        writer.write_all(&value).map_err(Into::into)
    }
}

#[derive(Debug)]
struct Frame {
    payload: Vec<u8>,
}

enum FrameCodec {}

impl<T> Decode<&mut T, Result<Frame, Box<dyn std::error::Error>>> for FrameCodec
where
    T: std::io::Read,
{
    fn decode(reader: &mut T) -> Result<Frame, Box<dyn std::error::Error>> {
        let length = reader.decode_mut::<LengthHeaderCodec>()?;

        let payload = reader.contextual_decode_mut::<PayloadCodec>(&length)?;

        Ok(Frame { payload })
    }
}

impl<T> ContextualEncode<&Frame, &mut T, Result<(), Box<dyn std::error::Error>>> for FrameCodec
where
    T: std::io::Write,
{
    fn encode(frame: &Frame, writer: &mut T) -> Result<(), Box<dyn std::error::Error>> {
        use std::convert::TryFrom;

        let length = u32::try_from(frame.payload.len())?;

        writer.write_all(&length.to_be_bytes())?;

        writer.write_all(&frame.payload).map_err(Into::into)
    }
}

#[quickcheck]
fn equivalent_when_decode_and_then_encode(mut payload: Vec<u8>) {
    payload.truncate(u32::MAX as usize);

    let length = (payload.len() as u32).to_be_bytes().to_vec();

    // length header: 4 bytes big endian
    // payload: payload length depends on the length header
    let bytes = vec![length, payload].concat();

    let mut reader = std::io::Cursor::new(&bytes);

    let frame = reader.decode_mut::<FrameCodec>().unwrap();

    let mut buff = Vec::new();

    buff.contextual_encode_mut::<FrameCodec>(&frame).unwrap();

    assert_eq!(buff, bytes);
}
