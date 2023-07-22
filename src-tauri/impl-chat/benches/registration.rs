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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
