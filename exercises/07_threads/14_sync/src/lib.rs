// "`Sync`에 대해 많이 연습할 것은 없습니다, 단지 기억해야 할 것이 있습니다."
fn outro() -> &'static str {
    "I have a good understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
