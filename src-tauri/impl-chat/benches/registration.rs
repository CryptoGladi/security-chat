use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, Rng};
use tokio::runtime::Builder;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("registration", |b| {
        b.to_async(Builder::new_multi_thread().enable_all().build().unwrap())
            .iter(|| async {
                let rand_string: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(20)
                    .map(char::from)
                    .collect();

                let _client = impl_chat::client::Client::registration(&rand_string)
                    .await
                    .unwrap();
            });
    });

    c.bench_function("login", |b| {
        b.to_async(Builder::new_multi_thread().enable_all().build().unwrap())
            .iter(|| async {
                let test_nickname = "test_nickname";
                let authkey = "d515004d-c283-4b38-abe7-3e7403addc93";

                assert_eq!(
                    impl_chat::client::Client::check_valid(test_nickname, authkey)
                        .await
                        .unwrap(),
                    true
                );
            });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
