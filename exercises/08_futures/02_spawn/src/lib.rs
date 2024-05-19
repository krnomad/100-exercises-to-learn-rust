use tokio::net::TcpListener;

// "TODO: 동시에 두 리스너에서 TCP 연결을 수락하는 에코 서버를 작성하십시오."
// "여러 연결(동일한 리스너에서)은 동시에 처리되어야 합니다."
// "수신된 데이터는 클라이언트에게 다시 반영되어야 합니다."
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

// "요청을 보내세요"
                    writer.write_all(request.as_bytes()).await.unwrap();
// "소켓의 쓰기 측을 닫으십시오"
                    writer.shutdown().await.unwrap();

// "응답을 읽어보세요"
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
