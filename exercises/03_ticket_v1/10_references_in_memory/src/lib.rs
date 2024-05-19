pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// "TODO: 이 섹션에서 배운 내용을 바탕으로 `todo!()`를 대체하십시오"
// "해당 유형에 맞는 올바른 **스택 크기**."
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), todo!());
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&u64>(), todo!());
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), todo!());
    }
}
