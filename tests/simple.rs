extern crate typed_codec;

use quickcheck_macros::quickcheck;
use typed_codec::*;

struct Base64Codec<T>(T);

impl<T> Encode for Base64Codec<T>
where
    T: AsRef<[u8]>,
{
    type Target = T;
    type Output = String;

    fn encode(value: Self::Target) -> Self::Output {
        base64::encode(value)
    }
}

impl<T> Decode for Base64Codec<T>
where
    T: AsRef<[u8]>,
{
    type Target = T;
    type Output = Result<String, Box<dyn std::error::Error>>;

    fn decode(value: T) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = base64::decode(value)?;
        String::from_utf8(bytes).map_err(Into::into)
    }
}

#[test]
fn encode() {
    let actual = "foobarbaz12345".encode::<Base64Codec<_>>();
    let expected = "Zm9vYmFyYmF6MTIzNDU=".to_owned();

    assert_eq!(actual, expected);
}

#[test]
fn decode() {
    let actual = "Zm9vYmFyYmF6MTIzNDU=".decode::<Base64Codec<_>>().unwrap();
    let expected = "foobarbaz12345".to_owned();

    assert_eq!(actual, expected);
}

#[quickcheck]
fn equivalent_when_encode_and_then_decode(random_value: String) {
    let actual = random_value
        .encode::<Base64Codec<_>>()
        .decode::<Base64Codec<_>>()
        .unwrap();

    assert_eq!(actual, random_value);
}
