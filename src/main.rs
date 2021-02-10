mod client;
mod server;

fn show_help_text() {
    println!("Usage:\n  remouse connect [IP]\n  remouse serve");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 && args[1] == "connect" {
        let ip = &args[2];
        let mut client = client::init(ip.clone());
        client::run(&mut client);
    } else if args.len() == 2 && args[1] == "serve" {
        let mut server = server::init();
        server::run(&mut server);
    } else {
        return show_help_text();
    }
}
