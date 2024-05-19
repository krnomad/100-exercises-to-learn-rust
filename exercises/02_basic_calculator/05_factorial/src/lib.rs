// "`n`이라는 음이 아닌 정수가 주어졌을 때, `factorial`이라는 함수를 정의하십시오."
// "`n!`, 즉 `n`의 팩토리얼을 반환합니다."
// You didn't provide any text to translate. Please provide the text you want to be translated.
// "`n`의 팩토리얼은 `n`까지의 모든 양의 정수의 곱으로 정의됩니다."
// "예를 들어, `5!` (이것을 "팩토리얼 5"라고 읽습니다.)는 `5 * 4 * 3 * 2 * 1`로, 그 값은 `120`입니다."
// "`0!`은 `1`로 정의됩니다."
// You did not provide any text to translate.
// "`factorial(0)`이 `1`을 반환하고, `factorial(1)`이 `1`을 반환하는 것을 기대합니다."
// "`factorial(2)`가 `2`를 반환하고, 그런 식으로 계속하다."
// As there is no text to translate, I cannot provide a translation. Please provide a text in English to translate to Korean.
// "배운 것만 사용하세요! 아직 루프를 사용할 수 없으니, 재귀를 사용해야 합니다!"

fn factorial(p0: i32) -> i32 {
    if p0 == 0 {
        1
    } else {
        p0*factorial(p0-1)
    }
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
