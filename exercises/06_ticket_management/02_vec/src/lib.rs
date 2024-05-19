// "주어진 숫자 `n`에 대해, 피보나치 수열에서 `n+1` 번째 숫자를 반환하십시오."
// You haven't provided any text to translate. Please provide the text.
// "피보나치 수열은 다음과 같이 정의됩니다:"
// 당신이 번역할 내용을 제공하지 않았습니다.
// "- 수열의 첫 번째 숫자는 0입니다."
// "- 수열의 두 번째 숫자는 1입니다."
// "- 모든 이후의 숫자는 앞선 두 숫자의 합입니다."
// You didn't provide any content to translate. Please provide the text you want to translate into Korean.
// "그래서 순서는 다음과 같습니다: 0, 1, 1, 2, 3, 5, 8, 13, 21, 그리고 이어서."
// 코멘트 내용이 없어 번역할 수 없습니다.
// "우리는 `fibonacci(0)`이 `0`을 반환하고, `fibonacci(1)`이 `1`을 반환할 것으로 예상합니다,"
// "`fibonacci(2)`가 `1`을 반환하고, 그런 식으로 계속되는 것입니다."
pub fn fibonacci(n: u32) -> u32 {
// "TODO: `fibonacci` 함수를 구현하십시오"
// You didn't provide any text to translate. Please provide the text you want to be translated into Korean.
// "힌트: 이미 계산한 결과를 메모이제이션하기 위해 `Vec`을 사용하세요."
// "그래서 여러 번 다시 계산할 필요가 없습니다."
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
