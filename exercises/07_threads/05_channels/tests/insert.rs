use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
// "스레드가 더 이상 실행되지 않으면, 이것은 패닉 상태가 될 것입니다."
// "왜냐하면 채널이 폐쇄될 것이기 때문입니다."
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
// "이 연습에서는 자동으로 확인할 수 있는 것이 거의 없습니다."
// "우리 서버는 어떠한 **읽기** 동작도 드러내지 않습니다."
// "삽입이 실제로 일어나고 있는지, 그리고 그것들이 실제로 일어나고 있는지 우리가 알 방법이 없습니다."
// "정확하게 진행되고 있습니다."
// "이 연습을 끝낸 것 같으면 `move_forward`를 `true`로 설정하세요."
// "해결책을 확인하려면 언제든지 강사에게 전화하실 수 있습니다!"
    let move_forward = false;

    assert!(move_forward);
}
