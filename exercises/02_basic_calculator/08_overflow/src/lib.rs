// "`dev` 프로필을 오버플로우 시에 감싸도록 커스터마이즈하세요."
// "Cargo의 문서를 확인하여 올바른 구문을 찾아보세요:"
// "https://doc.rust-lang.org/cargo/reference/profiles.html"
// "우리가 나중에 설명할 이유로, 커스터마이징은 `Cargo.toml`에서 이루어져야 합니다."
// "저장소의 루트에 있고, 연습문제의`Cargo.toml`에는 없습니다."

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
// "20! 은 2432902008176640000인데, 이는 u32에 들어갈 수 있을 만큼의 크기가 아닙니다."
// "기본 개발 프로필로는 `cargo test`를 실행할 때 패닉이 발생합니다."
// "우리는 대신 그것이 둘러싸도록 하고 싶습니다"
        assert_eq!(factorial(20), 2_192_834_560);
// "☝️"
// "가독성을 높이기 위해 밑줄을 사용하는 큰 숫자 리터럴!"
    }

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
