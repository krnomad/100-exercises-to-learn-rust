pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// "TODO: 이 섹션에서 배운 내용을 바탕으로, `todo!()`를 대체하십시오."
// "해당 유형에 대한 올바른 **스택 크기**."
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), todo!());
    }

    #[test]
    fn ticket_size() {
// "이건 까다로운 질문이군요!"
// "이번에는 '직관적인' 답이 우연히 정답이었습니다."
// "그러나, 일반적으로, 구조체의 메모리 레이아웃은 더 복잡한 주제입니다."
// "궁금하시다면, Rustonomicon의 '데이터 레이아웃' 섹션을 확인해보세요."
// "https://doc.rust-lang.org/nomicon/data.html 에서 더 자세한 정보를 확인하세요."
        assert_eq!(size_of::<Ticket>(), todo!());
    }
}
