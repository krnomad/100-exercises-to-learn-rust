fn compute(a: u32, b: u32) -> u32 {
// "TODO: 컴파일러 오류를 수정하고 테스트를 통과시키기 위해 아래 줄을 변경하세요."
    a + b * 4u8
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
