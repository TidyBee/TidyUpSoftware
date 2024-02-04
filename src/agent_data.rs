use gethostname::gethostname;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AgentVersion {
    latest_version: String,
    minimal_version: String,
}

#[derive(Serialize, Clone)]
pub struct AgentData {
    agent_version: AgentVersion,
    machine_name: String,
    process_id: u32,
    uptime: u64,
    watched_directories: Vec<PathBuf>,
}

#[allow(dead_code)]
impl AgentData {
    pub fn build(
        latest_versionn: String,
        minimal_versionn: String,
        directories_watch_args: Vec<PathBuf>,
    ) -> AgentData {
        AgentData {
            agent_version: AgentVersion {
                latest_version: latest_versionn,
                minimal_version: minimal_versionn,
            },
            machine_name: gethostname().to_str().unwrap().to_owned(),
            process_id: sysinfo::get_current_pid().unwrap().as_u32(),
            uptime: sysinfo::System::uptime(),
            watched_directories: directories_watch_args,
        }
    }

    pub fn dump(&self) {
        info!(
            "Agent's status and his configuration below :\n
        Latest version : {}
        Minimal version : {}
        Machine name : {:?}
        Pid : {:?}
        Up time : {:?}
        Watched directories : {:?}
        ",
            self.agent_version.latest_version,
            self.agent_version.minimal_version,
            self.machine_name,
            self.process_id,
            self.uptime,
            self.watched_directories
        );
    }

    pub fn update(&mut self) {
        self.uptime = sysinfo::System::uptime();
    }
}
