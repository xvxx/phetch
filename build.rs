use std::{env, process};

fn main() {
    println!("cargo:rustc-env=PLATFORM={}", env::var("TARGET").unwrap());
    println!("cargo:rustc-env=BUILD_DATE={}", sh("date +%Y-%m-%d"));
    println!(
        "cargo:rustc-env=GIT_REF={}",
        sh("git rev-parse --short HEAD")
    )
}

fn sh(args: &str) -> String {
    let args: Vec<&str> = args.split(" ").collect();
    let cmd = args[0];
    let args: Vec<_> = args.iter().skip(1).collect();

    if let Ok(output) = process::Command::new(cmd).args(&args).output() {
        if !output.status.success() {
            eprintln!("Error running {} {:?}", cmd, args);
            return "???".to_string();
        }
        String::from_utf8(output.stdout).unwrap()
    } else {
        String::new()
    }
}
