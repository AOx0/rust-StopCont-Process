use std::process::Command;
use std::io::{self, Write};
use sysinfo::{ProcessExt, System, SystemExt};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_name: &str = &(args[1..].join(" ").to_lowercase());

    if target_name.len() == 0 { return; }
    
    let s = System::new_all();
    for (pid, process) in s.get_processes() {
        let process_path = process.exe().display().to_string().to_lowercase();
        let process_name = process.name().to_lowercase();
        if process_name == target_name || (process_path.contains(target_name) && process_path.contains("contents/macos") && process_path.matches("/").count() < 7) {
            let s = pid.to_string();
            let _s: &str = &s;
            let a = Command::new("kill")
            .args(&["-9", _s ])
            .output()
            .expect("Fail");
            io::stdout().write_all(&a.stdout).unwrap();
            io::stderr().write_all(&a.stderr).unwrap();
            println!("Killed {1} with PID {0}", pid, process.name());
            break;
        }
    }
}