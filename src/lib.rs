#![no_std]

pub trait Encode<IN, OUT> {
    fn encode(value: IN) -> OUT;
}

pub trait ContextualEncode<IN, CTX, OUT> {
    fn encode(value: IN, ctx: CTX) -> OUT;
}

pub trait Decode<IN, OUT> {
    fn decode(value: IN) -> OUT;
}

pub trait ContextualDecode<IN, CTX, OUT> {
    fn decode(value: IN, ctx: CTX) -> OUT;
}

pub trait EncodeExt<'a, OUT>: Sized {
    fn encode<E>(&'a self) -> OUT
    where
        E: Encode<&'a Self, OUT>;

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

pub trait ContextualEncodeExt<'a, CTX, OUT>: Sized {
    fn contextual_encode<E>(&'a self, ctx: CTX) -> OUT
    where
        E: ContextualEncode<&'a Self, CTX, OUT>;

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

pub trait DecodeExt<'a, OUT>: Sized {
    fn decode<D>(&'a self) -> OUT
    where
        D: Decode<&'a Self, OUT>;

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

pub trait ContextualDecodeExt<'a, CTX, OUT>: Sized {
    fn contextual_decode<D>(&'a self, ctx: CTX) -> OUT
    where
        D: ContextualDecode<&'a Self, CTX, OUT>;

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
