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
