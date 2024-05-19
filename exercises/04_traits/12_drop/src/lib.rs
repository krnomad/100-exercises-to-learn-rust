// "TODO: "Drop bomb"이라고 불리는 것을 구현하라: 드랍될 때 패닉이 발생하는 타입"
// "특정 작업이 수행되지 않은 한."
// "아래의 테스트에서 예상되는 API를 볼 수 있습니다."

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
// "폭탄은 떨어질 때 패닉에 빠져야 한다"
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
// "폭탄이 떨어질 때 공황에 빠져서는 안 된다."
// "그것이 해체되었기 때문에"
    }
}
