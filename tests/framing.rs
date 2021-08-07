extern crate typed_codec;

use quickcheck_macros::quickcheck;
use typed_codec::*;

struct LengthHeaderCodec<'a, T>(std::marker::PhantomData<&'a T>);

impl<'a, T> Decode for LengthHeaderCodec<'a, T>
where
    T: std::io::Read,
{
    type Target = &'a mut T;
    type Output = Result<u32, Box<dyn std::error::Error>>;

    fn decode(reader: Self::Target) -> Self::Output {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;

        let length = u32::from_be_bytes(buf);
        Ok(length)
    }
}

impl<'a, T> ContextualEncode for LengthHeaderCodec<'a, T>
where
    T: std::io::Write,
{
    type Target = &'a mut T;
    type Context = u32;
    type Output = Result<(), Box<dyn std::error::Error>>;

    fn encode(writer: Self::Target, length: Self::Context) -> Self::Output {
        writer.write_all(&length.to_be_bytes()).map_err(Into::into)
    }
}

struct PayloadCodec<'a, T>(std::marker::PhantomData<&'a T>);

impl<'a, T> ContextualDecode for PayloadCodec<'a, T>
where
    T: std::io::Read,
{
    type Target = &'a mut T;
    type Context = u32;
    type Output = Result<Vec<u8>, Box<dyn std::error::Error>>;

    fn decode(reader: Self::Target, length: Self::Context) -> Self::Output {
        let mut buf = vec![0; length as usize];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<'a, T, U> ContextualEncode for PayloadCodec<'a, (T, U)>
where
    T: std::io::Write,
    U: AsRef<[u8]>,
{
    type Target = &'a mut T;
    type Context = U;
    type Output = Result<(), Box<dyn std::error::Error>>;

    fn encode(writer: Self::Target, value: Self::Context) -> Self::Output {
        writer.write_all(value.as_ref()).map_err(Into::into)
    }
}

#[derive(Debug, PartialEq)]
struct Frame {
    payload: Vec<u8>,
}

struct FrameCodec<'a, T>(std::marker::PhantomData<&'a T>);

impl<'a, T> Decode for FrameCodec<'a, T>
where
    T: std::io::Read,
{
    type Target = &'a mut T;
    type Output = Result<Frame, Box<dyn std::error::Error>>;

    fn decode(reader: Self::Target) -> Self::Output {
        let length = reader.decode_mut::<LengthHeaderCodec<_>>()?;
        let payload = reader.contextual_decode_mut::<PayloadCodec<_>>(length)?;
        Ok(Frame { payload })
    }
}

impl<'a, T> ContextualEncode for FrameCodec<'a, T>
where
    T: std::io::Write,
{
    type Target = &'a mut T;
    type Context = &'a Frame;
    type Output = Result<(), Box<dyn std::error::Error>>;

    fn encode(writer: Self::Target, frame: Self::Context) -> Self::Output {
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

    buff.contextual_encode_mut::<LengthHeaderCodec<_>>(length)
        .unwrap();

    let expected = vec![0x00, 0x00, 0x12, 0x34];

    assert_eq!(buff, expected);
}

#[test]
fn decode_length_header() {
    let mut bytes = std::io::Cursor::new(vec![0x00u8, 0x00, 0x12, 0x34]);

    let actual = bytes.decode_mut::<LengthHeaderCodec<_>>().unwrap();
    let expected = 0x1234;

    assert_eq!(actual, expected);
}

#[test]
fn encode_payload() {
    let mut buff = Vec::new();
    let payload = vec![1, 2, 3, 4];

    buff.contextual_encode_mut::<PayloadCodec<_>>(&payload)
        .unwrap();

    assert_eq!(buff, payload)
}

#[test]
fn decode_payload() {
    let mut bytes = std::io::Cursor::new(vec![1, 2, 3, 4, 5]);

    let actual = bytes.contextual_decode_mut::<PayloadCodec<_>>(4).unwrap();
    let expected = vec![1, 2, 3, 4];

    assert_eq!(actual, expected)
}

#[test]
fn encode_frame() {
    let mut buff = Vec::new();
    let frame = Frame {
        payload: vec![1, 2, 3, 4],
    };

    buff.contextual_encode_mut::<FrameCodec<_>>(&frame).unwrap();
    let expected = vec![0, 0, 0, 4, 1, 2, 3, 4];

    assert_eq!(buff, expected);
}

#[test]
fn decode_frame() {
    let mut bytes = std::io::Cursor::new(vec![0, 0, 0, 4, 1, 2, 3, 4]);

    let actual = bytes.decode_mut::<FrameCodec<_>>().unwrap();
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

    let frame = reader.decode_mut::<FrameCodec<_>>().unwrap();

    let mut buff = Vec::new();

    buff.contextual_encode_mut::<FrameCodec<_>>(&frame).unwrap();

    assert_eq!(buff, bytes);
}
