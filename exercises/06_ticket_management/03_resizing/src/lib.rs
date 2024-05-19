#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

// "새로운 수용력이 얼마가 될지 추측해 볼 수 있을까요?"
// "표준 라이브러리가 어떤 것에 대해서도 보장하지 않는다는 점을 주의하세요."
// "벡터의 크기를 조정하기 위해 사용된 알고리즘, 따라서 이것은 향후에 변경될 수 있습니다."
        assert_eq!(v.capacity(), todo!());
    }
}
