use async_std::{io, task};
use async_std::net::TcpListener;
use async_std::prelude::*;
use anyhow::Result;

#[async_std::main]
async fn main()->Result<()> {
    #[cfg(target_os = "linux")]
    {
        println!("set limit max open file:{}", 5000);
        rlimit::Resource::NOFILE.set(5000, 5000)?;
    }

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn::<_,Result<()>>(async move{
            let (reader, writer) = &mut (&stream, &stream);

            loop {
                let mut b = [0u8];
                if reader.read(&mut b).await? == 0 {
                    break;
                }
                writer.write(&b).await?;
            }

            Ok(())
            //let _=io::copy(reader, writer).await;
        });
    }

    Ok(())
}
