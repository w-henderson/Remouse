mod client;
mod keyboard;
mod server;

fn show_help_text() {
    println!("Usage:\n  remouse connect [IP]\n  remouse serve");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 && args[1] == "connect" {
        println!("connecting...");
        let ip = &args[2];
        let mut client = client::init(ip.clone());
        println!("connected, streaming mouse input");
        let allow_movement = args.len() == 4 && args[3] == "--no-override-movement";
        client::run(&mut client, !allow_movement);
    } else if args.len() == 2 && args[1] == "serve" {
        println!("starting server...");
        let mut server = server::init();
        println!("waiting for client...");
        server::run(&mut server);
    } else {
        return show_help_text();
    }
}
