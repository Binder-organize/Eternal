use crate::function;
use std::process::exit;
use std::{
    env,
    process::{Child, Command, Stdio},
};

impl function::MyEguiApp {
    pub fn shell(&mut self, input: String) {
        // 检查输入是否为空
        if input.trim().is_empty() {
            return;
        }

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let mut args = parts;

            match command {
                "cd" => {
                    let new_dir = args.next().unwrap_or("/");
                    let root = self.process_path(new_dir);

                    if let Err(e) = env::set_current_dir(&root) {
                        self.strs += &*format!("Error changing directory: {}\n", e);
                    }

                    previous_command = None;
                }
                "exit" => exit(0),
                "clear" => {
                    self.strs = String::new();
                    previous_command = None;
                }
                _ => {
                    if !command.is_empty() {
                        let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                            Stdio::from(output.stdout.unwrap())
                        });

                        let stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::piped() // 修改为 piped 以便捕获输出
                        };

                        let stderr = Stdio::piped(); // 捕获标准错误输出

                        match Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .stderr(stderr) // 重定向标准错误输出到标准输出
                            .spawn()
                        {
                            Ok(output) => previous_command = Some(output),
                            Err(e) => {
                                self.strs +=
                                    &*format!("Command not found: {}\nError: {}\n", command, e);
                                previous_command = None;
                            }
                        };
                    }
                }
            }
        }

        if let Some(final_command) = previous_command {
            match final_command.wait_with_output() {
                Ok(output) => {
                    if !output.stdout.is_empty() {
                        self.strs += &*String::from_utf8_lossy(&output.stdout);
                    }
                    if !output.stderr.is_empty() {
                        self.strs += &*String::from_utf8_lossy(&output.stderr);
                    }
                }
                Err(e) => self.strs += &*format!("Error executing command: {}\n", e),
            }
        }
    }

    pub fn format_path(&mut self, path: &std::path::Path) -> String {
        let home_dir = self.home_dir();
        if let Some(home) = home_dir {
            if let Ok(stripped) = path.strip_prefix(&home) {
                return format!("~/{}", stripped.display());
            }
        }
        path.display().to_string()
    }

    pub fn process_path(&mut self, path_str: &str) -> std::path::PathBuf {
        let path_str = if path_str.starts_with('~') {
            let home_dir = self.home_dir();
            home_dir
                .map(|mut home| {
                    let relative = path_str.trim_start_matches('~');
                    let relative = relative.trim_start_matches('/');
                    home.push(relative);
                    home.to_str().unwrap_or(path_str).to_string()
                })
                .unwrap_or_else(|| path_str.to_string())
        } else {
            path_str.to_string()
        };

        std::path::PathBuf::from(path_str)
    }

    pub fn home_dir(&mut self) -> Option<std::path::PathBuf> {
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .ok()
            .map(|s| std::path::PathBuf::from(s))
    }
}
