use outro_02::Order;

// "`tests` 디렉토리 내의 파일들은 테스트를 실행할 때만 컴파일됩니다."
// "결과적으로, 조건부 컴파일을 위한 `#[cfg(test)]` 속성이 필요 없게 되었습니다—그것은"
// "시사하는."

#[test]
fn test_order() {
    let mut order = Order::new("Rusty Book".to_string(), 3, 2999);

    assert_eq!(order.product_name(), "Rusty Book");
    assert_eq!(order.quantity(), &3);
    assert_eq!(order.unit_price(), &2999);
    assert_eq!(order.total(), 8997);

    order.set_product_name("Rust Book".to_string());
    order.set_quantity(2);
    order.set_unit_price(3999);

    assert_eq!(order.product_name(), "Rust Book");
    assert_eq!(order.quantity(), &2);
    assert_eq!(order.unit_price(), &3999);
    assert_eq!(order.total(), 7998);
}

// "유효성 검사"
#[test]
#[should_panic]
fn test_empty_product_name() {
    Order::new("".to_string(), 3, 2999);
}

#[test]
#[should_panic]
fn test_long_product_name() {
    Order::new("a".repeat(301), 3, 2999);
}

#[test]
#[should_panic]
fn test_zero_quantity() {
    Order::new("Rust Book".to_string(), 0, 2999);
}

#[test]
#[should_panic]
fn test_zero_unit_price() {
    Order::new("Rust Book".to_string(), 3, 0);
}
