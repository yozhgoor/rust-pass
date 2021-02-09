extern crate base64;
extern crate rpassword;
extern crate sha1;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let cmd = &args[0];

    match &args[1..] {
        [site] if site != "--help" => println!("{}", make_password(&ask_password().unwrap(), site)),
        [] => println!(
            "{}",
            make_password(&ask_password().unwrap(), &ask_site().unwrap())
        ),
        _ => usage(cmd),
    }
}

fn make_password(password: &str, site: &str) -> String {
    let digest = sha1::Sha1::from(format!("_{}_{}_", password, site))
        .digest()
        .bytes();
    let b64 = b64::encode(digest.as_ref());

    generate_password(&b64)
}
