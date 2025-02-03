use std::{
    env,
    io::{stdin, stdout, Write},
    process::{Child, Command, Stdio},
};

pub fn shell() {
    loop {
        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
                continue;
            }
        };

        let current_path = format_path(&current_dir);

        print!("[{}] > ", current_path);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            let stdin = previous_command
                .map_or(Stdio::inherit(), |output: Child| {
                    Stdio::from(output.stdout.unwrap())
                });

            let stdout = if commands.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };

            match Command::new(command)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn()
            {
                Ok(output) => previous_command = Some(output),
                Err(e) => {
                    eprintln!("Command not found: {}", command);
                    previous_command = None;
                }
            };
        }

        if let Some(mut final_command) = previous_command {
            match final_command.wait() {
                Ok(_) => {}
                Err(e) => eprintln!("Error executing command: {}", e),
            }
        }
    }
}

pub fn format_path(path: &std::path::Path) -> String {
    let home_dir = home_dir();
    if let Some(home) = home_dir {
        if let Ok(stripped) = path.strip_prefix(&home) {
            return format!("~/{}", stripped.display());
        }
    }
    path.display().to_string()
}

pub fn home_dir() -> Option<std::path::PathBuf> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .ok()
        .map(|s| std::path::PathBuf::from(s))
}