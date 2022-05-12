use encryption::encrypt;

fn main() {
    let result = encrypt(String::from("xyz"));
    println!("{}", result);
}
