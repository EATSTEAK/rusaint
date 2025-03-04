#[derive(uniffi::Record)]
/// uniffi 지원을 위한 u32 Pair입니다.
pub struct UnsignedIntPair {
    first: u32,
    second: u32,
}

type U32Pair = (u32, u32);

uniffi::custom_type!(U32Pair, UnsignedIntPair, {
    remote,
    lower: |obj| UnsignedIntPair {
        first: obj.0,
        second: obj.1,
    },
    try_lift: |val| Ok((val.first, val.second))
});
