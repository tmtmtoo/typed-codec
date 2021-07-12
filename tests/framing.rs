extern crate typed_codec;

use quickcheck_macros::quickcheck;
use typed_codec::*;

enum LengthHeaderCodec {}

impl<R> Decode<&mut R, Result<u32, Box<dyn std::error::Error>>> for LengthHeaderCodec
where
    R: std::io::Read,
{
    fn decode(reader: &mut R) -> Result<u32, Box<dyn std::error::Error>> {
        let mut buf = [0; 4];

        reader.read_exact(&mut buf)?;

        let length = u32::from_be_bytes(buf);

        Ok(length)
    }
}

impl<W> ContextualEncode<&mut W, u32, Result<(), Box<dyn std::error::Error>>> for LengthHeaderCodec
where
    W: std::io::Write,
{
    fn encode(writer: &mut W, length: u32) -> Result<(), Box<dyn std::error::Error>> {
        writer.write_all(&length.to_be_bytes()).map_err(Into::into)
    }
}

enum PayloadCodec {}

impl<R> ContextualDecode<&mut R, u32, Result<Vec<u8>, Box<dyn std::error::Error>>> for PayloadCodec
where
    R: std::io::Read,
{
    fn decode(reader: &mut R, length: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buf = vec![0; length as usize];

        reader.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl<W, V> ContextualEncode<&mut W, V, Result<(), Box<dyn std::error::Error>>> for PayloadCodec
where
    W: std::io::Write,
    V: AsRef<[u8]>,
{
    fn encode(writer: &mut W, value: V) -> Result<(), Box<dyn std::error::Error>> {
        writer.write_all(value.as_ref()).map_err(Into::into)
    }
}

#[derive(Debug, PartialEq)]
struct Frame {
    payload: Vec<u8>,
}

enum FrameCodec {}

impl<R> Decode<&mut R, Result<Frame, Box<dyn std::error::Error>>> for FrameCodec
where
    R: std::io::Read,
{
    fn decode(reader: &mut R) -> Result<Frame, Box<dyn std::error::Error>> {
        let length = reader.decode_mut::<LengthHeaderCodec>()?;

        let payload = reader.contextual_decode_mut::<PayloadCodec>(length)?;

        Ok(Frame { payload })
    }
}

impl<W> ContextualEncode<&mut W, &Frame, Result<(), Box<dyn std::error::Error>>> for FrameCodec
where
    W: std::io::Write,
{
    fn encode(writer: &mut W, frame: &Frame) -> Result<(), Box<dyn std::error::Error>> {
        use std::convert::TryFrom;

        let length = u32::try_from(frame.payload.len())?;

        writer.write_all(&length.to_be_bytes())?;

        writer.write_all(&frame.payload).map_err(Into::into)
    }
}

#[test]
fn encode_length_header() {
    let mut buff = Vec::new();
    let length = 0x1234u32;

    buff.contextual_encode_mut::<LengthHeaderCodec>(length)
        .unwrap();

    let expected = vec![0x00, 0x00, 0x12, 0x34];

    assert_eq!(buff, expected);
}

#[test]
fn decode_length_header() {
    let mut bytes = std::io::Cursor::new(vec![0x00u8, 0x00, 0x12, 0x34]);

    let actual = bytes.decode_mut::<LengthHeaderCodec>().unwrap();
    let expected = 0x1234;

    assert_eq!(actual, expected);
}

#[test]
fn encode_payload() {
    let mut buff = Vec::new();
    let payload = vec![1, 2, 3, 4];

    buff.contextual_encode_mut::<PayloadCodec>(&payload)
        .unwrap();

    assert_eq!(buff, payload)
}

#[test]
fn decode_payload() {
    let mut bytes = std::io::Cursor::new(vec![1, 2, 3, 4, 5]);

    let actual = bytes.contextual_decode_mut::<PayloadCodec>(4).unwrap();
    let expected = vec![1, 2, 3, 4];

    assert_eq!(actual, expected)
}

#[test]
fn encode_frame() {
    let mut buff = Vec::new();
    let frame = Frame {
        payload: vec![1, 2, 3, 4],
    };

    buff.contextual_encode_mut::<FrameCodec>(&frame).unwrap();
    let expected = vec![0, 0, 0, 4, 1, 2, 3, 4];

    assert_eq!(buff, expected);
}

#[test]
fn decode_frame() {
    let mut bytes = std::io::Cursor::new(vec![0, 0, 0, 4, 1, 2, 3, 4]);

    let actual = bytes.decode_mut::<FrameCodec>().unwrap();
    let expected = Frame {
        payload: vec![1, 2, 3, 4],
    };

    assert_eq!(actual, expected);
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
