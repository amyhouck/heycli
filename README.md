# heycli
A CLI app that interacts with Hey.Café with commands. Why? Why not! I'm also learning how to use Clap to write commands with this project.

## Installation
Currently, as no release build is given yet, you must have Rust installed. [Click here](http://www.rust-lang.org) to go to Rust's website.

After installing Rust/Cargo, run `cargo install --git https://github.com/amyhouck/heycli.git`

## Commands
- `heycli` - Will display either the version number, or basic notification information depending on if an API key is set
- `heycli connect` - Gives instructions on obtaining an API key and adding it to heycli.
- `heycli disconnect` - Removes the API key

- `heycli notif` - Displays up to 10 unread notifications. This will not mark them as seen.
- `heycli notif remove [--all] [--id #]` - Marks one or all notifications as seen. You can only use 1 flag.
    - `-a/--all` Marks all notifications as seen.
    - `-i#/--id #` Marks one notification as seen with the ID given. The ID is the number in which it appears in `heycli notif`.