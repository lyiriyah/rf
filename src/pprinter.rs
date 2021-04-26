use crate::info;
use colored::*;
use whoami;
use std::env;

pub fn pprint_info(info: info::Info) {
    let mut topline = String::from("");

    for i in 0..(info.username.len() + info.hostname.len() + 1) {
        if i == (info.username.len() + info.hostname.len() + 1) / 2 {
            topline.push('┬')
        } else {
            topline.push('─');
        }
    }
    
    println!("{}@{}", info.username.green(), info.hostname.green());
    println!("{}", topline);
    println!("  {} │ {}", "distro".bright_blue(), whoami::distro());
    println!("  {} │ {}", "  arch".bright_blue(), env::consts::ARCH);
    println!("  {} │ {}", "editor".bright_blue(), info.editor);
    println!("  {} │ {}", " shell".bright_blue(), info.shell);
    println!("  {} │ {}", "   cpu".bright_blue(), info.cpu_name);
    println!("  {} │ {} MB", "memory".bright_blue(), (info.memory.total / 1024));
    println!("  {} │ {} MB/{} MB",
             " usage".bright_blue(),
             ((info.memory.total - info.memory.avail) / 1024),
             (info.memory.total / 1024));
}
