extern crate typed_codec;

use des::cipher::*;
use quickcheck_macros::quickcheck;
use typed_codec::*;

enum DesCrypto {}

impl<T> ContextualEncode<T, T, Result<Vec<u8>, String>> for DesCrypto
where
    T: AsRef<[u8]>,
{
    fn encode(key: T, value: T) -> Result<Vec<u8>, String> {
        let des = des::Des::new_from_slice(key.as_ref()).map_err(|e| format!("{}", e))?;

        let mut block = generic_array::GenericArray::clone_from_slice(value.as_ref());

        des.encrypt_block(&mut block);

        Ok(block.to_vec())
    }
}

impl<T> ContextualDecode<T, T, Result<Vec<u8>, String>> for DesCrypto
where
    T: AsRef<[u8]>,
{
    fn decode(key: T, value: T) -> Result<Vec<u8>, String> {
        let des = des::Des::new_from_slice(key.as_ref()).map_err(|e| format!("{}", e))?;

        let mut block = generic_array::GenericArray::clone_from_slice(value.as_ref());

        des.decrypt_block(&mut block);

        Ok(block.to_vec())
    }
}

#[quickcheck]
fn equivalent_when_encode_and_then_decode(mut key: Vec<u8>, mut value: Vec<u8>) {
    key.resize(8, 0);
    value.resize(8, 0);

    let encrypted = value.contextual_encode::<DesCrypto>(&key).unwrap();
    let decrypted = encrypted.contextual_decode::<DesCrypto>(&key).unwrap();

    assert_eq!(decrypted, value);
}
