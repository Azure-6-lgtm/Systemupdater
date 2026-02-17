use duct::cmd;
use std::io::{self, Write};

fn sysupdate() {
    println!("\n\x1b[94mUpdating system...\x1b[0m");
    cmd!("pkg", "update")
        .run()
        .unwrap();
    println!("\x1b[92m✔ Updated successfully\x1b[0m\n");

}

fn sysupgrade() {
    println!("\n\x1b[94mUpgrading system...\x1b[0m");
    cmd!("pkg", "upgrade")
        .run()
        .unwrap();
    println!("\x1b[92m✔ Upgraded successfully\x1b[0m\n");
}

fn jnkremover() {
    println!("\n\x1b[94mRemoving junk...\x1b[0m");
    cmd!("pkg", "clean")
        .run()
        .unwrap();
    cmd!("pkg","autoclean")
        .run()
        .unwrap();
    println!("\x1b[92m✔ Upgraded successfully\x1b[0m\n");
}

fn main() {
    println!("\x1b[95m▒█▀▀▀█ █░░█ █▀▀ ▀▀█▀▀ █▀▀ █▀▄▀█\x1b[0m");
    println!("\x1b[95m░▀▀▀▄▄ █▄▄█ ▀▀█ ░░█░░ █▀▀ █░▀░█\x1b[0m");
    println!("\x1b[95m▒█▄▄▄█ ▄▄▄█ ▀▀▀ ░░▀░░ ▀▀▀ ▀░░░▀\x1b[0m");

    println!("\x1b[95m█░░█ █▀▀█ █▀▀▄ █▀▀█ ▀▀█▀▀ █▀▀ █▀▀█\x1b[0m");
    println!("\x1b[95m█░░█ █░░█ █░░█ █▄▄█ ░░█░░ █▀▀ █▄▄▀\x1b[0m");
    println!("\x1b[95m░▀▀▀ █▀▀▀ ▀▀▀░ ▀░░▀ ░░▀░░ ▀▀▀ ▀░▀▀\x1b[0m");
    println!("\x1b[37mOptions available\x1b[0m");
    println!("\x1b[1;94m1\x1b[0m \x1b[37mSystem update\x1b[0m");
    println!("\x1b[1;96m2\x1b[0m \x1b[37mSystem upgrade\x1b[0m");
    println!("\x1b[1;93m3\x1b[0m \x1b[37mJunk remover\x1b[0m");
    println!("\x1b[1;91m0\x1b[0m \x1b[90mExit\x1b[0m");
    println!();
    print!("\x1b[95m>\x1b[0m ");
   
    io::stdout()
        .flush()
        .unwrap();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Not valid!");

    let choice: i64 = match choice
        .trim()
        .parse() {
            Ok(n) => n,
            Err(_) => {println!("\x1b[91mInvalid input\x1b[0m");
            return;}
        };
    match choice {
        1 => sysupdate(),
        2 => sysupgrade(),
        3 => jnkremover(),
        0 => println!("\x1b[37mBye\x1b[Om"),
        _ => println!("\x1b[37mBye\x1b[Om"),
    };
}
