// "이것은 Rust 파일입니다. `.rs` 확장자를 가진 일반 텍스트 파일입니다."
// Your comment is empty. Please provide a text to translate.
// "대부분의 현대 프로그래밍 언어와 마찬가지로 Rust도 주석을 지원합니다. 지금 바로 하나를 보고 계신 것입니다!"
// "주석은 컴파일러에 의해 무시됩니다; 코드에 메모와 설명을 추가하는데 활용할 수 있습니다."
// "Rust에서는 각각의 목적에 맞는 다양한 방법으로 주석을 작성할 수 있습니다."
// "이제 가장 일반적인 방법인 라인 코멘트를 사용하겠습니다."
// "`//`에서 줄 끝까지가 주석으로 간주됩니다." 는 한국어로 "`//`에서 줄 끝까지가 주석으로 간주됩니다." 입니다.

// "연습 문제에는 `TODO`, `todo!()` 또는 `__` 표시가 포함되어 있어, 코드를 작성해야 하는 부분을 찾는데 도움이 됩니다."
// "이러한 마커를 자신의 코드로 교체하여 연습문제를 완료해야 합니다."
// "때때로 한 줄의 코드만으로 충분할 때도 있고, 때로는 더 긴 섹션을 작성해야 할 때도 있습니다."
// The comment text is missing. Could you please provide the text to be translated?
// "연습 문제를 10분 이상 해결하지 못할 경우 트레이너를 찾아보세요! 여러분을 도와드리기 위해 여기에 있습니다!"
// "`solutions` git 브랜치에서 모든 연습문제의 해답을 찾을 수도 있습니다."
fn greeting() -> &'static str {
// "TODO: 저를 고쳐주세요 👇"
    "I'm ready to __!"
}

// "당신의 해결책은 일련의 테스트를 통해 자동으로 검증됩니다."
// "이 연습문제의 디렉터리 루트에서 터미널에 `cargo test` 명령을 직접 호출함으로써 이 테스트들을 실행할 수 있습니다."
// "이것은 `wr` 명령어가 내부적으로 수행하는 작업입니다."
// It appears you didn't provide any English comment to be translated into Korean. Could you please provide it?
// "Rust는 코드와 함께 테스트를 작성할 수 있도록 도와줍니다."
// "`#[cfg(test)]` 속성은 테스트를 실행할 때만 (즉, `cargo test`를 실행할 때만) 아래의 코드를 컴파일하라는 컴파일러에게의 지시입니다."
// "나중에 이 과정에서 속성과 테스트에 대해 더 배울 예정입니다."
// "지금은 `#[cfg(test)]` 속성을 찾아야 테스트를 찾을 수 있으며, 이 테스트들이 당신의 해결책의 정확성을 확인할 것이라는 점만 알아두세요!"
// You didn't provide any text to translate. Please provide the text you want to be translated.
// "⚠️ **테스트를 수정하지 마세요** ⚠️"
// "테스트는 니가 제시한 해결책을 검증하는데 도움이 됩니다. 바꿔야 하는 것은 테스트하는 코드뿐이며, 테스트 자체는 아닙니다."
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}