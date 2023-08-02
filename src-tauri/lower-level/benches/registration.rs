use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{distributions::Alphanumeric, Rng};
use tokio::runtime::Builder;

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("registration", |b| {
        b.to_async(Builder::new_multi_thread().enable_all().build().unwrap())
            .iter_batched(
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

    c.bench_function("login", |b| {
        b.to_async(Builder::new_multi_thread().enable_all().build().unwrap())
            .iter(|| async {
                let test_nickname = "test_nickname";
                let authkey = "d515004d-c283-4b38-abe7-3e7403addc93";

                lower_level::client::Client::check_valid(
                    test_nickname,
                    authkey,
                    ADDRESS_SERVER.parse().unwrap(),
                )
                .await
                .unwrap()
            });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
