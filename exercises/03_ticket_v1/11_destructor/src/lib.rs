// "우리는 디스트럭터에 대한 적절한 연습문제를 작성하기 위해 추가적인 기계 장치가 필요합니다."
// "우리는 특성을 다룬 후에 다시 후반의 장에서 이 개념을 다룰 것입니다."
// "내부 가변성."
fn outro() -> &'static str {
    "I have a basic understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
