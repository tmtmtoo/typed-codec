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

#[quickcheck]
fn equivalent_when_encode_and_then_decode(value: String) {
    let encoded = value.encode::<Base64Codec>();
    let decoded = encoded.decode::<Base64Codec>().unwrap();
    assert_eq!(decoded, value);
}
