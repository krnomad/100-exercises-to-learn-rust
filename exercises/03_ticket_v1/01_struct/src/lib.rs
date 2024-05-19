// "다음 필드를 가진 `Order`라는 이름의 구조체를 정의하세요:"
// "- `price`, 부호가 없는 정수"
// "- `quantity`, 부호가 없는 정수"
// Your request seems to be empty. Can you provide a text for translation?
// "또한 `is_available`이라는 메소드를 가지고 있어야 하며, 이는 수량이 있을 경우 `true`를 반환해야 합니다."
// "0보다 크면, 그렇지 않으면 '거짓'."

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
