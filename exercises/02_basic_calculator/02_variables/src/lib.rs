// "👇 아래에 있는 `///`로 시작하는 줄들을 **문서 주석**이라고 합니다."
// "그들은 그들 뒤에 따라오는 항목에 문서를 첨부합니다. 이 경우에는 `speed` 함수입니다."
// "이 연습의 디렉토리에서 `cargo doc --open`을 실행하면, 러스트가 생성합니다."
// "이러한 주석에서 HTML 문서를 만들어 브라우저에서 열어보세요."

/// `speed` function calculates the average speed of a journey.
///
/// # Arguments
///
/// * `start` - A u32 that holds the starting point of the journey.
/// * `end` - A u32 that holds the ending point of the journey.
/// * `time_elapsed` - A u32 that holds the time taken to complete the journey.
///
/// # Returns
///
/// * A u32 representing the average speed of the journey.
///
/// # Example
///
/// ```
/// use variables::speed;
/// let start = 0;
/// let end = 10;
/// let time_elapsed = 10;
/// let avg_speed = speed(start, end, time_elapsed);
/// assert_eq!(avg_speed, 1);
/// ```
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
// "TODO: 테스트가 통과하도록 적절한 값으로 `distance`라는 변수를 정의하시오."
// "`distance`의 유형을 주석으로 달아야 하나요? 왜 그러거나 그렇지 않은가요?"
    let distance = end - start;

// "아래 줄은 변경하지 마세요"
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
