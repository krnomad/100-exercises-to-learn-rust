fn intro() -> &'static str {
// "TODO: 제게 수정이 필요합니다 👇"
    "I'm ready to __!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to start modelling a software ticket!");
    }
}
