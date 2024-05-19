// "`while` 루프를 사용하여 팩토리얼 함수를 다시 작성하십시오."
pub fn factorial(mut n: u32) -> u32 {
// "`todo!()` 매크로는 컴파일러의 플레이스홀더입니다."
// "이것은 '나중에 다시 이것에 참여하겠다'로 해석된다, 따라서"
// "타입 오류를 억제하다."
// "런타임에 패닉이 발생합니다."
    let mut result = 1;
    while n > 0 {
        result *= n;
        n -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
