use high_level::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    nickname: String,
}

#[tokio::main]
async fn main() {
    println!("Simple client!");
    let opt = Opt::from_args();

    let mut client = Client::registration(
        &opt.nickname,
        ClientInitConfig::new("config.bin", "http://[::1]:2052"),
    )
    .await
    .unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == ""
    }
}
