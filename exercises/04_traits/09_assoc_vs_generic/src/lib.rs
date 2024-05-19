// "TODO: `self`를 거듭제곱하는 `power`라는 메소드가 있는 `Power`라는 새로운 트레잇을 정의하십시오."
// "`n`의 거듭제곱."
// "특성 정의와 그 구현은 얻기에 충분해야 합니다."
// "컴파일하고 통과하기 위한 테스트들."
// You did not provide any text to translate. Please try again.
// "추천: 일반적인 구현을 처리하기 위해 유혹될 수 있습니다."
// "모든 경우를 한 번에 다룹니다. 그러나 이것은 상당히 복잡하며 사용이 필요합니다."
// "추가 크레이트 (즉, `num-traits`)."
// "그럼에도 불구하고, 피하려면 간단한 매크로를 사용하는 것이 더 좋을 수 있습니다."
// "고도로 일반화된 구현의 복잡성을 확인해보세요."
// ""Rust 매크로의 작은 책" (https://veykril.github.io/tlborm/)을 보시려면"
// "그것에 대해 더 배우는 데 관심이 있습니다."
// "하지만 그럴 필요는 없습니다: 세 개를 별개로 작성해도 완벽하게 괜찮습니다."
// "직접 구현하세요. 궁금한 경우에만 더 나아가세요."

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
