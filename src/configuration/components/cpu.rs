use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

#[derive(Deserialize, Debug)]
pub struct Cpu {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl Converter for Cpu {
    fn convert(&self, sys: &mut System) -> Component {
        sys.refresh_cpu_usage();
        let total = sys.global_cpu_info().cpu_usage() as usize;
        let total = format!("{}%", total);
        let name = self.name.as_deref().unwrap_or("");
        let icon = self.icon.as_deref().unwrap_or("");
        Component {
            name,
            icon,
            value: total,
        }
    }

    fn get_time(&self) -> u64 {
        self.time.unwrap_or(500)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            time: Some(1000),
            name: Some(String::from("CPU")),
            icon: Some(String::from(" ")),
        }
    }
}
