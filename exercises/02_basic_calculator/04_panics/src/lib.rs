// "/ 여행의 시작점과 종착점, 그리고 그 여행을 완료하는 데 걸린 시간을 주어,"
// "/ 여행의 평균 속도를 계산하다."
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
// "TODO: `time_elapsed`가 0이면 커스텀 메시지와 함께 패닉 발생시키기"
    if time_elapsed == 0 {
        panic!("The journey took no time at all, that's impossible!")
    }
    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
// "👇 `#[should_panic]` 주석을 사용하여 우리는 코드가 실패하길 기대한다고 주장할 수 있습니다"
// "테스트 중에 패닉이 발생합니다. 또한 `expected`를 사용하여 패닉 메시지를 확인할 수 있습니다."
// "이것은 모두 러스트의 내장된 테스트 프레임워크의 일부입니다!"
    #[should_panic(expected = "The journey took no time at all, that's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
