use std::hash::Hash;
use std::os::windows::process::CommandExt;
use serde::Serialize;

#[derive(Debug, Eq, Serialize)]
pub struct Command {
    pub local_path: String,
    pub remote_path: String,
    pub priority: Option<usize>,
}

impl Command {
    #[cfg(target_os = "windows")]
    pub fn spawn_rclone_command(&self, cloud: &str, root: &str, speed: f32) {
        let _ = std::process::Command::new("rclone")
            .arg("copy")
            .arg("--bwlimit")
            .arg(format!("{}K", speed.to_string()))
            .arg(&self.local_path)
            .arg(format!("{}:{}\\{}", cloud, root, self.remote_path))
            .creation_flags(0x08000000)
            .output();
    }
}

impl Hash for Command {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.local_path.hash(state);
        self.remote_path.hash(state);
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.local_path == other.local_path && self.remote_path == other.remote_path
    }
}