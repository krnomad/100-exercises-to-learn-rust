// "TODO: 정수 벡터가 주어지면, 이를 두 개의 반으로 분할하십시오."
// "그리고 각 반의 합계를 별도의 스레드에서 계산하십시오."
// "어떠한 힙 할당도 수행하지 마십시오. 메모리를 어떤 형태로도 유출하지 마십시오."

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
