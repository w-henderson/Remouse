![Banner](assets/banner.png)

# remouse
`remouse` is a quick and easy CLI to control a computer's keyboard and mouse without plugging your peripherals physically into it. Written in Rust, it uses UDP sockets to provide an instantaneous connection between two devices on your network, sharing the mouse movements, mouse clicks, scroll events, key strokes and more in a split second.

## Quick Start
1. To use `remouse`, start by downloading a binary from the [Releases page](https://github.com/w-henderson/Remouse/releases) to both your computer which you wish to control (we'll call this the server), and the computer you wish to control it with (the client).
2. Start the server by running `remouse serve`, which will open a UDP socket on port 42069.
3. On the client, run `remouse connect <ip of server>:42069` to connect to the server. Your input events will immediately start going through the network and you're good to go!
4. Press the escape key on the client to unlock your cursor once you've finished.

By default, `remouse` locks your cursor to the top left of your primary monitor in the key capture box so it doesn't unintentionally interact with the client computer. If you want to disable this behaviour, simply pass the parameter `--no-override-movement` to the `connect` command. Beware that this could prevent keystrokes from being transmitted as the key capture box does not globally hook the keyboard.

## Building from Source
If you have Rust and Cargo installed, you only need to run `cargo build` to build the program.