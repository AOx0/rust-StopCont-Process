use std::process::Command;
use std::io::{self, Write};
use sysinfo::{ProcessExt, System, SystemExt};
use std::env;

pub fn kill_shortcut(flag: &str, msg: &str) {
    let args: Vec<String> = env::args().collect();
    let target_name: &str = &(args[1..].join(" ").to_lowercase());

    if target_name.len() == 0 { return; }
    
    for (pid, process) in System::new_all().get_processes() {
        let process_path = process.exe().display().to_string().to_lowercase();
        let process_name = process.name().to_lowercase();
        if process_name == target_name || (process_path.contains(target_name) && process_path.contains("contents/macos") && process_path.matches("/").count() < 7) {
            let pid = pid.to_string();
            let pid: &str = &pid;
            let command = Command::new("kill")
            .args(&[flag, pid ])
            .output()
            .expect("Fail");
            io::stdout().write_all(&command.stdout).unwrap();
            io::stderr().write_all(&command.stderr).unwrap();
            println!("{2} {1} with PID {0}", pid, process.name(), msg);
            break;
        }
    }
}
