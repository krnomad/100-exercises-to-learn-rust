// "TODO: 이 크레이트에 있는 각 모듈에서 해야 할 일이 있습니다!"
mod description;
mod status;
mod title;

// "러스트에서 일반적인 패턴은 코드를 여러 개의 (비공개) 모듈로 분할하는 것입니다."
// "그리고 나서 해당 모듈의 공용 부분을 크레이트의 루트에서 다시 내보냅니다."
// There are no phrases or sentences provided to translate into Korean. Please provide specific text for translation.
// "이것은 사용자에게는 크레이트의 내부 구조를 숨기면서도 아직"
// "원하는 대로 코드를 정리할 수 있게 해줍니다."
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// "이제 필드를 프라이빗으로 설정할 필요가 없어졌습니다!"
// "각 필드가 자체 검증 로직을 캡슐화하기 때문에, 위험성이 없습니다."
// "`Ticket`의 사용자가 필드를 변경하여 깨지게 하는 방식"
// "struct의 불변자."
// 문장을 비워놓으셨네요. 번역할 영어 문장을 알려주세요.
// "그러나 주의하세요: 만약 여러 필드에 걸쳐 있는 불변값이 있다면, 당신은"
// "그 인변량이 계속 유지되는지 확인하고 다시 돌아가야 합니다"
// "필드를 비공개로 만드는 것."
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
