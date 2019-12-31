use std::process::Child;

pub struct Process {
    process: Child,
    is_running: bool,
}

impl Process {
    pub fn start(cmd: &str) -> Self {
        Self {
            process: std::process::Command::new(cmd)
                .spawn()
                .expect("Unable to start process"),
            is_running: true,
        }
    }

    pub fn stop(mut self) {
        self.process.kill().expect("command wasn't running");
        self.is_running = false;
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.is_running {
            self.process.kill().expect("command wasn't running");
        }
    }
}

pub fn start_lol() -> Process {
    Process::start("../target/League of Legends.exe")
}
