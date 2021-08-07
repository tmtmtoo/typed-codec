#![no_std]
#![deny(missing_docs)]

//! A highly abstracted encode/decode extension for any values.

#[cfg(test)]
#[macro_use]
extern crate std;

/// Trait that provides the encoding of the value.
pub trait Encode<IN, OUT> {
    /// Methods to implement the encoding function
    fn encode(value: IN) -> OUT;
}

/// Trait that provides contextual encoding of value.
pub trait ContextualEncode<IN, CTX, OUT> {
    /// Methods to implement the contextual encoding function
    fn encode(value: IN, ctx: CTX) -> OUT;
}

/// Trait that provides the decoding of the value.
pub trait Decode<IN, OUT> {
    /// Methods to implement the decoding function
    fn decode(value: IN) -> OUT;
}

/// Trait that provides the contextual decoding of the value.
pub trait ContextualDecode<IN, CTX, OUT> {
    /// Methods to implement the contextual decoding function
    fn decode(value: IN, ctx: CTX) -> OUT;
}

/// Trait that provides the encode method for any value.
pub trait EncodeExt<'a, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn encode<E>(&'a self) -> OUT
    where
        E: Encode<&'a Self, OUT>;

    /// Call this method if the value is mutable.
    fn encode_mut<E>(&'a mut self) -> OUT
    where
        E: Encode<&'a mut Self, OUT>;
}

impl<'a, T, OUT> EncodeExt<'a, OUT> for T {
    fn encode<E>(&'a self) -> OUT
    where
        E: Encode<&'a Self, OUT>,
    {
        E::encode(self)
    }

    fn encode_mut<E>(&'a mut self) -> OUT
    where
        E: Encode<&'a mut Self, OUT>,
    {
        E::encode(self)
    }
}

/// Trait that provides the contextual encode method for any value.
pub trait ContextualEncodeExt<'a, CTX, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn contextual_encode<E>(&'a self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<&'a Self, CTX, OUT>;

    /// Call this method if the value is mutable.
    fn contextual_encode_mut<E>(&'a mut self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<&'a mut Self, CTX, OUT>;
}

impl<'a, T, CTX, OUT> ContextualEncodeExt<'a, CTX, OUT> for T {
    fn contextual_encode<E>(&'a self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<&'a Self, CTX, OUT>,
    {
        E::encode(self, ctx)
    }

    fn contextual_encode_mut<E>(&'a mut self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<&'a mut Self, CTX, OUT>,
    {
        E::encode(self, ctx)
    }
}

/// Trait that provides the decode method for any value.
pub trait DecodeExt<'a, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn decode<D>(&'a self) -> OUT
    where
        D: Decode<&'a Self, OUT>;

    /// Call this method if the value is mutable.
    fn decode_mut<D>(&'a mut self) -> OUT
    where
        D: Decode<&'a mut Self, OUT>;
}

impl<'a, T, OUT> DecodeExt<'a, OUT> for T {
    fn decode<D>(&'a self) -> OUT
    where
        D: Decode<&'a Self, OUT>,
    {
        D::decode(self)
    }

    fn decode_mut<D>(&'a mut self) -> OUT
    where
        D: Decode<&'a mut Self, OUT>,
    {
        D::decode(self)
    }
}

/// Trait that provides the contextual decode method for any value.
pub trait ContextualDecodeExt<'a, CTX, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn contextual_decode<D>(&'a self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<&'a Self, CTX, OUT>;

    /// Call this method if the value is mutable.
    fn contextual_decode_mut<D>(&'a mut self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<&'a mut Self, CTX, OUT>;
}

impl<'a, T, CTX, OUT> ContextualDecodeExt<'a, CTX, OUT> for T {
    fn contextual_decode<D>(&'a self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<&'a Self, CTX, OUT>,
    {
        D::decode(self, ctx)
    }

    fn contextual_decode_mut<D>(&'a mut self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<&'a mut Self, CTX, OUT>,
    {
        D::decode(self, ctx)
    }
}

#[cfg(test)]
mod property_based_test {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::vec::Vec;

    enum Raw {}

    impl<T> Encode<T, T> for Raw {
        fn encode(value: T) -> T {
            value
        }
    }

    impl<T> ContextualEncode<T, (), T> for Raw {
        fn encode(value: T, _: ()) -> T {
            value
        }
    }

    impl<T> Decode<T, T> for Raw {
        fn decode(value: T) -> T {
            value
        }
    }

    impl<T> ContextualDecode<T, (), T> for Raw {
        fn decode(value: T, _: ()) -> T {
            value
        }
    }

    #[quickcheck]
    fn equivalent_when_encode(value: Vec<u8>) {
        let actual = value.encode::<Raw>();
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.encode_mut::<Raw>();
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_encode(value: Vec<u8>) {
        let actual = value.contextual_encode::<Raw>(());
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_encode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.contextual_encode_mut::<Raw>(());
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_decode(value: Vec<u8>) {
        let actual = value.decode::<Raw>();
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_decode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.decode_mut::<Raw>();
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_decode(value: Vec<u8>) {
        let actual = value.contextual_decode::<Raw>(());
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_decode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.contextual_decode_mut::<Raw>(());
        assert_eq!(actual, expected);
    }
}
