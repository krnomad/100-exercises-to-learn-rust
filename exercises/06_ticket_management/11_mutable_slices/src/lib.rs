// "TODO: 문자열 내 모든 문자를 소문자로 변환하는 `lowercase`라는 함수를 정의하세요."
// "입력값을 그 자리에서 수정하기."
// "`&mut String`이 필요한가요? `&mut [str]`은 작동하나요? 그렇다면 왜 그런가요, 아니라면 왜 아닌가요?"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = String::from("");
        lowercase(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn one_char() {
        let mut s = String::from("A");
        lowercase(&mut s);
        assert_eq!(s, "a");
    }

    #[test]
    fn multiple_chars() {
        let mut s = String::from("Hello, World!");
        lowercase(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mut_slice() {
        let mut s = "Hello, World!".to_string();
        lowercase(s.as_mut_str());
        assert_eq!(s, "hello, world!");
    }
}
