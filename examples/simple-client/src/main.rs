use high_level::{prelude::*, client::{storage_crypto::Nickname, impl_message::Message}};
use once_cell::sync::Lazy;
use structopt::StructOpt;
use simple_logger::SimpleLogger;
use log::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    nickname: String,
}

static INIT_CONFIG: Lazy<ClientInitConfig> =
    Lazy::new(|| ClientInitConfig::new("config.log", "http://[::1]:2052"));

#[tokio::main]
async fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    warn!("Starting simple client!");
    let opt = Opt::from_args();

    let mut client = match Client::have_account(&INIT_CONFIG.to_owned()).unwrap() {
        true => Client::load(INIT_CONFIG.to_owned())
            .await
            .unwrap(),
        false => Client::registration(&opt.nickname, INIT_CONFIG.to_owned())
            .await
            .unwrap(),
    };
    client.save().unwrap();
    info!("client info: {:?}", client);

    let recv_event = client.subscribe().await.unwrap();
    
    tokio::spawn(async move {
        loop {
            let notification = recv_event.recv().await.unwrap();
            match notification.event {
                Event::NewMessage(message) => println!("new message {}; from user: {}", message.body.text, notification.by_nickname),
                _ => {
                    info!("new event: {:?}", notification.event);
                }
            }
        }
    });

    loop {
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        let readline: Vec<String> = rl.readline(">> ").unwrap().split_whitespace().map(str::to_string).collect();
        
        if readline[0] == "send_key" {
            info!("sending crypto...");
            let nickname_from = Nickname::from(readline[1].clone());
            client.send_crypto(nickname_from).await.unwrap();
            info!("Done sending crypto!");
        }
        else if readline[0] == "accept_crypto" {
            info!("accepting crypto...");
            client.accept_all_cryptos().await.unwrap();
            info!("Done accepting crypto!");
        }
        else if readline[0] == "update_crypto" {
            info!("updating crypto...");
            client.update_cryptos().await.unwrap();
            info!("Done updating crypto");
        }
        else if readline[0] == "send" {
            let nickname_from = Nickname::from(readline[1].clone());
            let text = readline[2].clone();

            client.send_message(nickname_from, Message {
                text
            }).await.unwrap();
        }
    }
}
