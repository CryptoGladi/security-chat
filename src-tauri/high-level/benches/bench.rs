use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use high_level::{prelude::*, client::{storage_crypto::Nickname, impl_message::Message}};
use rand::{distributions::Alphanumeric, Rng};
use std::fmt::Display;
use tokio::runtime::{Builder, Runtime};

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

fn create_async_runtime() -> Runtime {
    Builder::new_multi_thread().enable_all().build().unwrap()
}

fn init() -> ClientInitConfig {
    ClientInitConfig::new("config.bin", ADDRESS_SERVER)
}

#[derive(Debug, Clone)]
struct TestAccount {
    init: ClientInitConfig,
    client_from: String,
}

impl Display for TestAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let test_account = create_async_runtime().block_on(async {
        let nickname = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>();

        let mut client_to = Client::registration(&nickname, init()).await.unwrap();

        let mut ii = init();
        ii.path_to_config_file = "config1.big".into();

        let nickname = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect::<String>();

        let mut client_from = Client::registration(&nickname, ii).await.unwrap();

        client_to.send_crypto(client_from.get_nickname()).await.unwrap();
        client_from.accept_all_cryptos().await.unwrap();
        client_to.update_cryptos().await.unwrap();
        client_to.save().unwrap();

        TestAccount {
            init: init(),
            client_from: client_from.get_nickname().0,
        }
    });

    c.bench_with_input(
        BenchmarkId::new("login", test_account.clone()),
        &test_account,
        |b, s| {
            b.to_async(create_async_runtime()).iter(|| async {
                let mut client = Client::load(test_account.init.clone()).await.unwrap();

                client.send_message(Nickname::from(test_account.client_from.clone()), Message {
                    text: "data".to_owned()
                }).await.unwrap();
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
