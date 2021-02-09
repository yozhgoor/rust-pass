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
    let b64 = base64::encode(digest.as_ref());

    generate_password(&b64)
}

fn generate_password(s: &str) -> String {
    let bytes = s.as_bytes();
    let symbols: &[u8] = "!?+-=*/@#$%&()[];:,.<>".as_bytes();

    format!(
        "{}{}{}{}{}",
        ("A".as_bytes()[0] + bytes[0] % 26) as char,
        ("a".as_bytes()[0] + bytes[1] % 26) as char,
        ("0".as_bytes()[0] + bytes[2] % 10) as char,
        symbols[bytes[3] as usize % symbols.len()] as char,
        s.get(4..26).unwrap(),
    )
}
