use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use anyhow::Result;
use async_std::io::prelude::*;
use async_std::task;
use async_std::net::TcpStream;
use async_std::task::sleep;
use structopt::StructOpt;

const DATA:&[u8]=&[1,2,3,4,5,6,7,8,9,0];

static COUNT_SECS:AtomicU64=AtomicU64::new(0);

#[async_std::main]
async fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

    #[cfg(target_os = "linux")]
    {
        let (soft, hard) = rlimit::Resource::NOFILE.get()?;
        if soft < opt.connects {
            println!("set limit max open file:{}", opt.connects * 2);
            rlimit::Resource::NOFILE.set(opt.connects * 2, opt.connects * 2)?;
        }
    }

    let mut joins =Vec::with_capacity(opt.connects as usize);

    for connect_id in 0..opt.connects {
        let addr=opt.addrs.clone();
        joins.push(task::spawn::<_,Result<()>>(async move{
            println!("start {} connect:{}",connect_id,addr);
            let stream= Arc::new(TcpStream::connect(&addr).await?);

            let reader=stream.clone();
            let join=task::spawn::<_,Result<()>>(async move {
                let mut read = &*reader;
                let mut data =[0u8;DATA.len()];
                loop{
                    read.read_exact(&mut data).await?;
                    COUNT_SECS.fetch_add(1,Ordering::Release);
                }
            });

            let mut sender=&*stream;
            loop{
                if sender.write_all(DATA).await.is_err(){
                    break;
                }
            }

            join.await?;
            Ok(())
        }));
    }

    task::spawn(async move{
       loop{
           let count= COUNT_SECS.swap(0,Ordering::Release);
           println!("{} tps",count);
           sleep(Duration::from_secs(1)).await;
       }
    });

    for join in joins {
        join.await?;
    }
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "test echo tool")]
struct Opt {
    /// addrs: 127.0.0.1:20000
    #[structopt(short, long)]
    addrs: String,
    /// connect number
    #[structopt(short, long)]
    connects: u64,

}
