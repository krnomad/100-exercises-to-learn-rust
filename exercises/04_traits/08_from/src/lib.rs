// "TODO: `example`을 컴파일하게 만들기 위해 `WrappingU32` 타입에 대한 `From` 트레잇을 구현하십시오."

pub struct WrappingU32 {
    value: u32,
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
