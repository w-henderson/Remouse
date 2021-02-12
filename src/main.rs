mod client;
mod keyboard;
mod server;

fn show_help_text() {
    println!("Usage:\n  remouse connect [IP]\n  remouse serve");
}

fn connect(args: &Vec<String>) {
    println!("connecting...");

    let ip = &args[2];
    let client_option = client::init(ip.clone());

    if let Some(mut client) = client_option {
        println!("connected, streaming input");
        let allow_movement = args.len() == 4 && args[3] == "--no-override-movement";
        client::run(&mut client, !allow_movement);
    } else {
        println!("connection failed, are you on the same version?");
    }
}

fn serve() {
    println!("starting server...");
    let mut server = server::init();
    println!("waiting for client...");
    server::run(&mut server);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 && args[1] == "connect" {
        connect(&args);
    } else if args.len() == 2 && args[1] == "serve" {
        serve();
    } else {
        return show_help_text();
    }
}
