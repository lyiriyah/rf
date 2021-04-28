use crate::info;
use colored::Colorize;
use std::env;

pub fn pprint_info(info: &info::Info) {
    let mut topline = String::from("");

    for i in 0..=info.username.len() + info.hostname.len() {
        if i == (info.username.len() + info.hostname.len() + 1) / 2 {
            topline.push('┬')
        } else {
            topline.push('─');
        }
    }
    
    println!("{}@{}", info.username.green(), info.hostname.green());
    println!("{}", topline);
    println!("  {} \u{2502} {}", "distro".bright_blue(), whoami::distro());
    println!("  {} \u{2502} {}", "  arch".bright_blue(), env::consts::ARCH);
    println!("  {} \u{2502} {}", "editor".bright_blue(), info.editor);
    println!("  {} \u{2502} {}", " shell".bright_blue(), info.shell);
    println!("  {} \u{2502} {}", "   cpu".bright_blue(), info.cpu_name);
    println!("  {} \u{2502} {} MB", "memory".bright_blue(), (info.memory.total / 1024));
    println!("  {} \u{2502} {} MB/{} MB",
             " usage".bright_blue(),
             ((info.memory.total - info.memory.avail) / 1024),
             (info.memory.total / 1024));
}
