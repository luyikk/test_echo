use anyhow::Result;
use structopt::StructOpt;

#[async_std::main]
async fn main()->Result<()> {
    let opt:Opt = Opt::from_args();


    for connect_id in 0..opt.connects {



    }
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "test echo tool")]
struct Opt{
    /// addrs: 127.0.0.1:20000
    #[structopt(short, long)]
    addrs:String,
    /// connect number
    #[structopt(short, long)]
    connects:u64
}