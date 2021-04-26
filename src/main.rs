mod info;

use whoami;
use colored::*;
use std::env;

fn main() {
    let hostname = whoami::hostname();
    let username = whoami::username();

    let cpuinfo = info::get_cpu_info();
    let editor = info::get_editor();
    let shell = info::get_shell(&username);
    let meminfo = info::get_meminfo();

    let mut topline = String::from("");

    for i in 0..(username.len() + hostname.len() + 1) {
        if i == (username.len() + hostname.len() + 1) / 2 {
            topline.push('┬')
        } else {
            topline.push('─');
        }
    }
    
    println!("{}@{}", username.green(), hostname.green());
    println!("{}", topline);
    println!("  {} │ {}", "distro".bright_blue(), whoami::distro());
    println!("  {} │ {}", "  arch".bright_blue(), env::consts::ARCH);
    println!("  {} │ {}", "editor".bright_blue(), editor);
    println!("  {} │ {}", " shell".bright_blue(), shell);
    println!("  {} │ {}", "   cpu".bright_blue(), cpuinfo);
    println!("  {} │ {} MB", "memory".bright_blue(), (meminfo.total / 1024));
    println!("  {} │ {} MB/{} MB", " usage".bright_blue(), ((meminfo.total - meminfo.avail) / 1024), (meminfo.total / 1024));
}
