use client::data::{Status, TicketDraft};
use client::launch;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn insert_works() {
// "자세한 사항을 처리할 클라이언트가 생겼으니 이제 테스트가 얼마나 간단해졌는지 주목하세요!"
    let client = launch();
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone());

    let client2 = client.clone();
    let ticket = client2.get(ticket_id).unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);
}
