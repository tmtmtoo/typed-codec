extern crate typed_codec;

use quickcheck_macros::quickcheck;
use typed_codec::*;

enum Base64Codec {}

impl<T> Encode<T, String> for Base64Codec
where
    T: AsRef<[u8]>,
{
    fn encode(value: T) -> String {
        base64::encode(value)
    }
}

impl<T> Decode<T, Result<String, Box<dyn std::error::Error>>> for Base64Codec
where
    T: AsRef<[u8]>,
{
    fn decode(value: T) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = base64::decode(value)?;
        String::from_utf8(bytes).map_err(Into::into)
    }
}

#[test]
fn encode() {
    let actual = "foobarbaz12345".encode::<Base64Codec>();
    let expected = "Zm9vYmFyYmF6MTIzNDU=".to_owned();

    assert_eq!(actual, expected);
}

#[test]
fn decode() {
    let actual = "Zm9vYmFyYmF6MTIzNDU=".decode::<Base64Codec>().unwrap();
    let expected = "foobarbaz12345".to_owned();

    assert_eq!(actual, expected);
}

#[quickcheck]
fn equivalent_when_encode_and_then_decode(random_value: String) {
    let actual = random_value
        .encode::<Base64Codec>()
        .decode::<Base64Codec>()
        .unwrap();

    assert_eq!(actual, random_value);
}
