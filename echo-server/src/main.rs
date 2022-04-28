use async_std::{io, task};
use async_std::net::TcpListener;
use async_std::prelude::*;
use anyhow::Result;

#[async_std::main]
async fn main()->Result<()> {
    #[cfg(target_os = "linux")]
    {
        println!("set limit max open file:{}", 200000);
        rlimit::Resource::NOFILE.set(200000, 200000)?;
    }

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(async move{
            let (reader, writer) = &mut (&stream, &stream);
            let _=io::copy(reader, writer).await;
        });
    }

    Ok(())
}
