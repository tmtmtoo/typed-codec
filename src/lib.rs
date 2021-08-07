#![no_std]
#![deny(missing_docs)]

//! A highly abstracted encode/decode extension for any values.

#[cfg(test)]
#[macro_use]
extern crate std;

/// Trait that provides the encoding of the value.
pub trait Encode {
    /// Type of value to encode.
    type Target;
    /// Type of encoding output.
    type Output;

    /// Methods to implement the encoding function
    fn encode(target: Self::Target) -> Self::Output;
}

/// Trait that provides contextual encoding of value.
pub trait ContextualEncode {
    /// Type of value to encode.
    type Target;
    /// Type of encoding context.
    type Context;
    /// Type of encoding output.
    type Output;

    /// Methods to implement the contextual encoding function
    fn encode(target: Self::Target, ctx: Self::Context) -> Self::Output;
}

/// Trait that provides the decoding of the value.
pub trait Decode {
    /// Type of value to decode.
    type Target;
    /// Type of decoding output.
    type Output;

    /// Methods to implement the decoding function
    fn decode(target: Self::Target) -> Self::Output;
}

/// Trait that provides the contextual decoding of the value.
pub trait ContextualDecode {
    /// Type of value to decode.
    type Target;
    /// Type of decoding context.
    type Context;
    /// Type of decoding output.
    type Output;

    /// Methods to implement the contextual decoding function
    fn decode(value: Self::Target, ctx: Self::Context) -> Self::Output;
}

/// Trait that provides the encode method for any value.
pub trait EncodeExt<'a, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn encode<E>(&'a self) -> OUT
    where
        E: Encode<Target = &'a Self, Output = OUT>;

    /// Call this method if the value is mutable.
    fn encode_mut<E>(&'a mut self) -> OUT
    where
        E: Encode<Target = &'a mut Self, Output = OUT>;
}

impl<'a, T, OUT> EncodeExt<'a, OUT> for T {
    fn encode<E>(&'a self) -> OUT
    where
        E: Encode<Target = &'a Self, Output = OUT>,
    {
        E::encode(self)
    }

    fn encode_mut<E>(&'a mut self) -> OUT
    where
        E: Encode<Target = &'a mut Self, Output = OUT>,
    {
        E::encode(self)
    }
}

/// Trait that provides the contextual encode method for any value.
pub trait ContextualEncodeExt<'a, CTX, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn contextual_encode<E>(&'a self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<Target = &'a Self, Context = CTX, Output = OUT>;

    /// Call this method if the value is mutable.
    fn contextual_encode_mut<E>(&'a mut self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<Target = &'a mut Self, Context = CTX, Output = OUT>;
}

impl<'a, T, CTX, OUT> ContextualEncodeExt<'a, CTX, OUT> for T {
    fn contextual_encode<E>(&'a self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<Target = &'a Self, Context = CTX, Output = OUT>,
    {
        E::encode(self, ctx)
    }

    fn contextual_encode_mut<E>(&'a mut self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<Target = &'a mut Self, Context = CTX, Output = OUT>,
    {
        E::encode(self, ctx)
    }
}

/// Trait that provides the decode method for any value.
pub trait DecodeExt<'a, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn decode<D>(&'a self) -> OUT
    where
        D: Decode<Target = &'a Self, Output = OUT>;

    /// Call this method if the value is mutable.
    fn decode_mut<D>(&'a mut self) -> OUT
    where
        D: Decode<Target = &'a mut Self, Output = OUT>;
}

impl<'a, T, OUT> DecodeExt<'a, OUT> for T {
    fn decode<D>(&'a self) -> OUT
    where
        D: Decode<Target = &'a Self, Output = OUT>,
    {
        D::decode(self)
    }

    fn decode_mut<D>(&'a mut self) -> OUT
    where
        D: Decode<Target = &'a mut Self, Output = OUT>,
    {
        D::decode(self)
    }
}

/// Trait that provides the contextual decode method for any value.
pub trait ContextualDecodeExt<'a, CTX, OUT>: Sized {
    /// Call this method if the value is immutable.
    fn contextual_decode<D>(&'a self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<Target = &'a Self, Context = CTX, Output = OUT>;

    /// Call this method if the value is mutable.
    fn contextual_decode_mut<D>(&'a mut self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<Target = &'a mut Self, Context = CTX, Output = OUT>;
}

impl<'a, T, CTX, OUT> ContextualDecodeExt<'a, CTX, OUT> for T {
    fn contextual_decode<D>(&'a self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<Target = &'a Self, Context = CTX, Output = OUT>,
    {
        D::decode(self, ctx)
    }

    fn contextual_decode_mut<D>(&'a mut self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<Target = &'a mut Self, Context = CTX, Output = OUT>,
    {
        D::decode(self, ctx)
    }
}

#[cfg(test)]
mod property_based_test {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::vec::Vec;

    struct Raw<T>(T);

    impl<T> Encode for Raw<T> {
        type Target = T;
        type Output = T;

        fn encode(target: Self::Target) -> Self::Output {
            target
        }
    }

    impl<T> ContextualEncode for Raw<T> {
        type Target = T;
        type Context = ();
        type Output = T;

        fn encode(target: Self::Target, _: Self::Context) -> Self::Output {
            target
        }
    }

    impl<T> Decode for Raw<T> {
        type Target = T;
        type Output = T;

        fn decode(target: Self::Target) -> Self::Output {
            target
        }
    }

    impl<T> ContextualDecode for Raw<T> {
        type Target = T;
        type Context = ();
        type Output = T;

        fn decode(target: Self::Target, _: Self::Context) -> Self::Output {
            target
        }
    }

    #[quickcheck]
    fn equivalent_when_encode(value: Vec<u8>) {
        let actual = value.encode::<Raw<_>>();
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.encode_mut::<Raw<_>>();
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_encode(value: Vec<u8>) {
        let actual = value.contextual_encode::<Raw<_>>(());
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_encode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.contextual_encode_mut::<Raw<_>>(());
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_decode(value: Vec<u8>) {
        let actual = value.decode::<Raw<_>>();
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_decode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.decode_mut::<Raw<_>>();
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_decode(value: Vec<u8>) {
        let actual = value.contextual_decode::<Raw<_>>(());
        let expected = &value;
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_contextual_decode_mut(mut value: Vec<u8>) {
        let expected = &mut value.clone();
        let actual = value.contextual_decode_mut::<Raw<_>>(());
        assert_eq!(actual, expected);
    }
}
