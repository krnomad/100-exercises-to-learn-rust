// "TODO: `echo` 서버는 비동기적 원시 값이 아닌 것을 사용합니다."
// "테스트를 실행할 때, 이것이 매달리는 것을 관찰해야 합니다, 그 이유는"
// "호출자와 서버 간의 교착 상태."
// "문제를 해결하려면 `echo` 내부에서 `spawn_blocking`을 사용하세요."
use std::io::{Read, Write};
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (socket, _) = listener.accept().await?;
        let mut socket = socket.into_std()?;
        socket.set_nonblocking(false)?;
        let mut buffer = Vec::new();
        socket.read_to_end(&mut buffer)?;
        socket.write_all(&buffer)?;
    }
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
        let (listener, addr) = bind_random().await;
        tokio::spawn(echo(listener));

        let requests = vec![
            "hello here we go with a long message",
            "world",
            "foo",
            "bar",
        ];
        let mut join_set = JoinSet::new();

        for request in requests {
            join_set.spawn(async move {
                let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                let (mut reader, mut writer) = socket.split();

// "요청을 보내세요"
                writer.write_all(request.as_bytes()).await.unwrap();
// "소켓의 쓰기 측을 닫으십시오"
                writer.shutdown().await.unwrap();

// "응답을 읽어보세요."
                let mut buf = Vec::with_capacity(request.len());
                reader.read_to_end(&mut buf).await.unwrap();
                assert_eq!(&buf, request.as_bytes());
            });
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
