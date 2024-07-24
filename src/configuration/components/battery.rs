use std::fs;

use serde::Deserialize;
use sysinfo::System;

use crate::{component::section::Component, configuration::converter::Converter};

const NAME: &str = "BAT";
const ICON_FULL: &str = "󰁹";
const ICON_MEDIUM: &str = "󰁿";
const ICON_LOW: &str = "󰁺";
const TIME: u64 = 1000;

#[derive(Deserialize, Debug)]
pub struct Battery {
    pub time: Option<u64>,
    pub name: Option<String>,
    pub icon_full: Option<String>,
    pub icon_medium: Option<String>,
    pub icon_low: Option<String>,
    pub path: String,
}

impl Converter for Battery {
    fn convert(&self, _sys: &mut System) -> Component {
        let battery_percentage =
            fs::read_to_string(&self.path).expect("Unable to read battery percentage");

        let battery_percentage: u8 = battery_percentage
            .trim()
            .parse()
            .expect("Unable to parse battery percentage");

        let total = format!("{}%", battery_percentage);

        let name = self.name.as_deref().unwrap_or(NAME);

        let icon = if battery_percentage > 80 {
            self.icon_full.as_deref().unwrap_or(ICON_FULL)
        } else if battery_percentage > 40 {
            self.icon_medium.as_deref().unwrap_or(ICON_MEDIUM)
        } else {
            self.icon_low.as_deref().unwrap_or(ICON_LOW)
        };
        Component {
            name,
            icon,
            value: total,
        }
    }

    fn get_time(&self) -> u64 {
        self.time.unwrap_or(TIME)
    }
}

impl Default for Battery {
    fn default() -> Self {
        Self {
            time: Some(TIME),
            name: Some(String::from(NAME)),
            icon_full: Some(String::from(ICON_FULL)),
            icon_medium: Some(String::from(ICON_MEDIUM)),
            icon_low: Some(String::from(ICON_LOW)),
            path: String::from("/sys/class/power_supply/BAT0/capacity"),
        }
    }
}
