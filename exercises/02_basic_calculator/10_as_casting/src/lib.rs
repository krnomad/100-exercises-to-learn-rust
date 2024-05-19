// "TODO: 이 섹션에서 배운 내용을 바탕으로 `todo!()`를 다음으로 교체하십시오."
// "변환 후의 정확한 값."

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        assert_eq!(47u16 as u32, todo!());
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u8_to_i8() {
        assert_eq!(255 as i8, todo!());
    }

    #[test]
    fn bool_to_u8() {
        assert_eq!(true as u8, todo!());
    }
}
