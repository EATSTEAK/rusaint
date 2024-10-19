use crate::UniffiCustomTypeConverter;


#[derive(uniffi::Record)]
/// uniffi 지원을 위한 u32 Pair입니다.
pub struct UnsignedIntPair {
    first: u32,
    second: u32,
}


type U32Pair = (u32, u32);

uniffi::custom_type!(U32Pair, UnsignedIntPair);

impl UniffiCustomTypeConverter for U32Pair {
    type Builtin = UnsignedIntPair;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok((val.first, val.second))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        UnsignedIntPair {
            first: obj.0,
            second: obj.1,
        }
    }
}