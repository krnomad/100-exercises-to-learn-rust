// "/ TODO: 아래 코드는 std의 채널을 사용하기 때문에 교착 상태에 빠질 것입니다,"
// "/ 동기화를 인지하지 못하는 것들."
// "/`tokio`의 채널 원시를 사용하도록 다시 작성하십시오 (건드려야 할 것입니다"
// "/ 테스팅 코드도, 맞아)."
// "/"
// "/ 데드락을 초래할 수 있는 이벤트 순서를 이해할 수 있나요?"
use std::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

// "/ 수신한 어떤 메시지에도 `퐁`으로 답장하면서 새로운 것을 설정하다"
// "/ 호출자와 계속 통신할 채널."
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel();
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use std::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
