use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// "서버 스레드를 생성하여 시스템을 시작하세요."
// "그것은 사용할 수 있는 `Sender` 인스턴스를 반환합니다."
// "서버와 상호작용하기 위해 하나 이상의 클라이언트에 의해."
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// "TODO: 서버 작업은 **절대로** 중단되어서는 안됩니다."
// "루프를 시작하세요: 명령이 나타날 때까지 기다리세요"
// "채널을 선택한 다음 실행하고, 그런 다음 기다리기 시작하세요."
// "다음 명령어를 위해."
pub fn server(receiver: Receiver<Command>) {}
