use api_high_level::{client::impl_message::Message, prelude::Client, test_utils::get_client};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Instant;

struct PairClient {
    pub client_to: Client,
    pub client_from: Client,
}

#[allow(clippy::single_call_fn)]
async fn get_pair_client() -> PairClient {
    let (_, _, mut client_to) = get_client().await;
    let (_, _, mut client_from) = get_client().await;

    client_to
        .send_crypto(client_from.get_nickname())
        .await
        .unwrap();
    client_from.accept_all_cryptos().await.unwrap();
    client_to.refresh_cryptos().await.unwrap();

    PairClient {
        client_to,
        client_from,
    }
}

fn criterion_benchmark(criterion: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    criterion.bench_function("registration", |bencher| {
        bencher.to_async(&runtime).iter_custom(|iters| async move {
            let init_args = get_client().await.1;

            let start = Instant::now();
            for _i in 0..iters {
                let nickname = fcore::test_utils::get_rand_string(20);
                black_box(
                    Client::registration(&nickname, init_args.clone())
                        .await
                        .unwrap(),
                );
            }
            start.elapsed()
        });
    });

    criterion.bench_function("send_message", |bencher| {
        bencher.to_async(&runtime).iter_custom(|iters| async move {
            let mut clients = get_pair_client().await;

            let start = Instant::now();
            for _i in 0..iters {
                let nickname = clients.client_from.get_nickname();
                let message = Message::new(fcore::test_utils::get_rand_string(160));

                clients
                    .client_to
                    .send_message(nickname, message)
                    .await
                    .unwrap();
            }
            start.elapsed()
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
