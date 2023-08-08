use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::{distributions::Alphanumeric, Rng};
use std::fmt::Display;
use tokio::runtime::{Builder, Runtime};

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

fn create_async_runtime() -> Runtime {
    Builder::new_multi_thread().enable_all().build().unwrap()
}

#[derive(Debug, Clone)]
struct TestAccount {
    nickname: String,
    authkey: String,
}

impl Display for TestAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nickname: {}; authkey: {}", self.nickname, self.authkey)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("registration", |b| {
        b.to_async(create_async_runtime()).iter_batched(
            || {
                rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(20)
                    .map(char::from)
                    .collect::<String>()
            },
            |rand_string| async move {
                let _client = lower_level::client::Client::registration(
                    &rand_string,
                    ADDRESS_SERVER.parse().unwrap(),
                )
                .await
                .unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    let test_account = create_async_runtime().block_on(async {
        let nickname = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>();

        let client =
            lower_level::client::Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
                .await
                .unwrap();

        TestAccount {
            nickname: client.data.nickname,
            authkey: client.data.auth_key,
        }
    });

    c.bench_with_input(
        BenchmarkId::new("login", test_account.clone()),
        &test_account,
        |b, s| {
            b.to_async(create_async_runtime()).iter(|| async {
                lower_level::client::Client::check_valid(
                    &s.nickname,
                    &s.authkey,
                    ADDRESS_SERVER.parse().unwrap(),
                )
                .await
                .unwrap()
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
