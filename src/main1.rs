use duct::cmd;
use std::io;

fn banner() {
    println!("\x1b[1;36m");
    println!("╔══════════════════════════════╗");
    println!("║        SYSTEM UPDATER        ║");
    println!("║          v1.24T              ║");
    println!("║        MADE IN RUST          ║");
    println!("║        Termux only(for now)  ║");
    println!("╚══════════════════════════════╝");
    println!("\x1b[0m");
}


fn suptade() {
    println!("\x1b[36m----System update----\x1b[0m");
    println!("\x1b[36m----Starting update----\x1b[0m");
    
    cmd!("pkg","update","-y")
        .stdout_null()
        .stderr_null()
        .run()
        .unwrap();
    
    println!("\x1b[32m----Update completed----\x1b[0m");
}

fn supgrade() {
    println!("\x1b[36m----System upgrade----\x1b[0m");
    println!("\x1b[36m----Starting upgrade----\x1b[0m");
    
    cmd!("pkg","upgrade","-y")
        .stdout_null()
        .stderr_null()
        .run()
        .unwrap();
   
    println!("\x1b[36m----Update completed----\x1b[0m");
}

fn sjunkrm() {
    println!("----System junk remover----");
    println!("----Starting junk remover----");
   
    cmd!("pkg", "autoremove", "-y")
        .stdout_null()
        .stderr_null()
        .run()
        .unwrap();
    
    cmd!("pkg", "clean")
        .stdout_null()
        .stderr_null()
        .run()
        .unwrap();

    println!("----Junk removed successfully----");
}

fn main()  {
    banner();
    println!("\x1b[36mEnter your choice:\x1b[0m");
    
    let mut choice = String::new();
    
    io::stdin()
        .read_line(&mut choice)
        .expect("Not valid choice!");

    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Encountered an error");
        return; },
    };

    match choice {
        1 => suptade(),
        2 => supgrade(),
        3 => sjunkrm(),
        _ => {println!("\x1b[31mAborting!\x1b[0m")}
    };
}
