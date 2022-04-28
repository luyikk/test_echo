// //use async_std::{io, task};
// //use async_std::net::TcpListener;
// //use async_std::prelude::*;
// use anyhow::Result;
//
// #[tokio::main]
// async fn main()->Result<()> {
//     #[cfg(target_os = "linux")]
//     {
//         println!("set limit max open file:{}", 200000);
//         rlimit::Resource::NOFILE.set(200000, 200000)?;
//     }
//
//     // let listener = TcpListener::bind("127.0.0.1:2000").await?;
//     // let mut incoming = listener.incoming();
//     //
//     // while let Some(stream) = incoming.next().await {
//     //     let stream = stream?;
//     //     tokio::spawn(async move{
//     //         let (reader, writer) = &mut (&stream, &stream);
//     //
//     //         loop {
//     //             let mut b = [0u8];
//     //             if reader.read(&mut b).await? == 0 {
//     //                 break;
//     //             }
//     //             writer.write(&b).await?;
//     //         }
//     //         //let _=io::copy(reader, writer).await;
//     //     });
//     // }
//
//
//
//     Ok(())
// }

// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("0.0.0.0:55555").await?;
//
//     loop {
//         let (mut socket, _) = listener.accept().await?;
//
//         tokio::spawn(async move {
//             let mut buf = [0; 1];
//
//             // In a loop, read data from the socket and write the data back.
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     // socket closed
//                     Ok(n) if n == 0 => return,
//                     Ok(n) => n,
//                     Err(e) => {
//                         eprintln!("failed to read from socket; err = {:?}", e);
//                         return;
//                     }
//                 };
//
//                 // Write the data back
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//         });
//     }
// }

use anyhow::Result;
use async_channel::unbounded;
use async_executor::Executor;
use easy_parallel::Parallel;
use futures_lite::future;
use smol::{io, Async};
use std::net::{TcpListener, TcpStream};

#[inline]
async fn echo(stream: Async<TcpStream>) -> io::Result<()> {
    io::copy(&stream, &mut &stream).await?;
    Ok(())
}

fn main() -> Result<()> {
    let ex = Executor::new();
    let (signal, shutdown) = unbounded::<()>();

    Parallel::new()
        // Run four executor threads.
        .each(0..16, |_| {
            future::block_on(ex.run(async move { shutdown.recv().await.unwrap() }))
        })
        // Run the main future on the current thread.
        .finish(|| {
            future::block_on(async {
                let listener = Async::<TcpListener>::bind(([0, 0, 0, 0], 55555)).unwrap();
                loop {
                    let (stream, peer_addr) = listener.accept().await.unwrap();
                    println!("Accepted client: {}", peer_addr);
                    // Spawn a task that echoes messages from the client back to it.
                    ex.spawn(echo(stream)).detach();
                }
                drop(signal);
            });
        });

    Ok(())
}
