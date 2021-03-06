extern crate typed_codec;

use des::cipher::*;
use quickcheck_macros::quickcheck;
use typed_codec::*;

struct DesCrypto<T>(T);

impl<T> ContextualEncode for DesCrypto<T>
where
    T: AsRef<[u8]>,
{
    type Target = T;
    type Context = T;
    type Output = Result<Vec<u8>, String>;

    fn encode(value: Self::Target, key: Self::Context) -> Self::Output {
        let des = des::Des::new_from_slice(key.as_ref()).map_err(|e| format!("{}", e))?;

        let mut block = generic_array::GenericArray::clone_from_slice(value.as_ref());

        des.encrypt_block(&mut block);

        Ok(block.to_vec())
    }
}

impl<T> ContextualDecode for DesCrypto<T>
where
    T: AsRef<[u8]>,
{
    type Target = T;
    type Context = T;
    type Output = Result<Vec<u8>, String>;

    fn decode(value: Self::Target, key: Self::Context) -> Self::Output {
        let des = des::Des::new_from_slice(key.as_ref()).map_err(|e| format!("{}", e))?;

        let mut block = generic_array::GenericArray::clone_from_slice(value.as_ref());

        des.decrypt_block(&mut block);

        Ok(block.to_vec())
    }
}

#[test]
fn encode() {
    let key = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    let value = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    let actual = value.contextual_encode::<DesCrypto<_>>(&key).unwrap();
    let expected = vec![0xe4, 0x04, 0xf3, 0xdf, 0x18, 0xa4, 0x53, 0x1b];

    assert_eq!(actual, expected);
}

#[test]
fn decode() {
    let key = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    let value = vec![0xe4, 0x04, 0xf3, 0xdf, 0x18, 0xa4, 0x53, 0x1b];

    let actual = value.contextual_decode::<DesCrypto<_>>(&key).unwrap();
    let expected = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    assert_eq!(actual, expected);
}

#[quickcheck]
fn equivalent_when_encode_and_then_decode(mut random_key: Vec<u8>, mut random_value: Vec<u8>) {
    random_key.resize(8, 0);
    random_value.resize(8, 0);

    let actual = random_value
        .contextual_encode::<DesCrypto<_>>(&random_key)
        .unwrap()
        .contextual_decode::<DesCrypto<_>>(&random_key)
        .unwrap();

    assert_eq!(actual, random_value);
}
