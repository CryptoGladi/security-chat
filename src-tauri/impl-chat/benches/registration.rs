use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use tokio::runtime::Builder;
use rand::{distributions::Alphanumeric, Rng};

fn random_string_arr(n_strings: i32, min_str_length: i32, max_str_length: i32) -> Vec<String>{
    let mut string_arr = vec![];
    
    for x in 0..n_strings{
        let mut count =  rand::thread_rng().gen_range(min_str_length..max_str_length);
        println!("rand char count is:{}", count.to_string());
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(count)
            .map(char::from)
            .collect();
        // iter_batched TODO

        string_arr.push(s)
    }
    string_arr
 }

fn criterion_benchmark(c: &mut Criterion) {
    let size: usize = 1024;

    c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
        b.to_async(Builder::new_multi_thread().enable_all().build().unwrap())
            .iter(|| async move {
                let nickname = size.to_string();
                impl_chat::client::Client::registration(nickname)
                    .await
                    .unwrap()
            });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
