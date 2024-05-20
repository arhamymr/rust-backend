use rand::distributions::Alphanumeric;
use rand::Rng;

fn randomize_string(input: &str, take: usize) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(take)
        .map(char::from)
        .collect();
    return format!("{}{}", input, rand_string);
}
pub fn generate_credentials(input: &str) -> (String, String) {
    let input_without_space = input.replace(" ", "");
    let client_id = randomize_string(&*&input_without_space, 16);
    let client_secret = randomize_string(&*&input_without_space, 32);
    (client_id, client_secret)
}
