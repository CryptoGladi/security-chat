use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn get_rand_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect::<String>()
}
