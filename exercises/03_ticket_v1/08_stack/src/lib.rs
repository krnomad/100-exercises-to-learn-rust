// "TODO: 이 섹션에서 배운 내용을 바탕으로 `todo!()`를 대체하세요"
// "해당 유형에 맞는 올바른 **스택 크기**."
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), todo!());
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), todo!());
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), todo!());
    }
}
