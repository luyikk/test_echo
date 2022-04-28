use anyhow::Result;
use structopt::StructOpt;

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



    for connect_id in 0..opt.connects {}
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
