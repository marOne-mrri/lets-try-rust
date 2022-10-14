use ferris_says::say;
use std::io::{stdin, stdout, BufWriter};
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..100);
    let str_secret = secret.to_string();
    print!("enter something {}", secret);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read");
    let stdout = stdout();
    let msg = input.push_str(&str_secret);
    let message = String::from(msg); // ! prb
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
