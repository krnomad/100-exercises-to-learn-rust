// "TODO: 정수 벡터가 주어지면, 그것의 힙 할당을 유출하십시오."
// "그런 다음 결과적으로 생성된 정적 슬라이스를 두 반으로 나누고"
// "각 절반을 별도의 스레드에서 합산하세요."
// "힌트: `Vec::leak`을 확인해보세요."

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
