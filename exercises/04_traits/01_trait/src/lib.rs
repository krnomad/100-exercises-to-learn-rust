// "`IsEven`이라는 트레잇을 정의하고, `self`가 짝수인 경우 `true`를 반환하는 `is_even`이라는 메소드를 가지도록 합니다.
// "그렇지 않으면 `false`."
// 댓글을 입력하지 않았습니다. 번역해야 할 문장이나 문구를 입력해주세요.
// "그런 다음 `u32`와 `i32`에 대해 트레잇을 구현하세요."

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42.is_even());
        assert!(!43.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42.is_even());
        assert!(!43.is_even());
        assert!(0.is_even());
        assert!(!(-1).is_even());
    }
}
