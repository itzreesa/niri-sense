use std::fs::File;
use std::path::Path;
use std::ptr::addr_eq;
use toml::{Table, Value};
use dirs;
use dirs::config_dir;
use log::{debug, error};
use serde::Deserialize;
use crate::util::{SenseVibrationLength, SenseVibrationStrength};

#[derive(Clone)]
pub struct SenseEventConfig {
  pub state: bool,
  pub strength: SenseVibrationStrength,
  pub length: SenseVibrationLength,
}

pub struct SenseConfig {
  pub server: SenseConfigServer,
  pub lengths: SenseConfigLengths,
  pub strengths: SenseConfigStrengths,
  pub events: SenseConfigEvents
}

pub struct SenseConfigServer {
  pub address: String,
  pub port: u16,
}

pub struct SenseConfigLengths {
  pub short: u64,
  pub medium: u64,
  pub long: u64,
  pub custom_a: u64,
  pub custom_b: u64,
}

pub struct SenseConfigStrengths {
  pub low: u8,
  pub medium: u8,
  pub high: u8,
  pub custom_a: u8,
  pub custom_b: u8,
}

pub struct SenseConfigEvents {
  pub repeated_event_prevention: bool,
  pub workspace_focus_changed: SenseEventConfig,
  pub workspace_changed: SenseEventConfig,
  pub window_opened_or_changed: SenseEventConfig,
  pub window_closed: SenseEventConfig,
  pub window_focus_changed: SenseEventConfig,
  pub window_layout_changed: SenseEventConfig,
  pub window_urgency_changed: SenseEventConfig,
  pub keyboard_layout_changed: SenseEventConfig,
  pub overview_opened: SenseEventConfig,
  pub overview_closed: SenseEventConfig,
  pub screenshot_captured: SenseEventConfig,
  pub cast_started_or_changed: SenseEventConfig,
  pub cast_stopped: SenseEventConfig,
  pub niri_config_reloaded: SenseEventConfig,
}

fn dump_default_config() {
  const DEFAULT_CONFIG_STR: &'static str = "[server]
address = \"127.0.0.1\"
port = 12345

[lengths]
# length presets in ms,
# 1000ms = 1s; minimum: 10, maximum: 1000
short = 80
medium = 160
long = 240

custom_a = 320
custom_b = 480

[strengths]
# strength value in %
# minimum: 1, maximum: 100
low = 20
medium = 50
high = 80

custom_a = 10
custom_b = 100

[events]
# if enabled, for every:
# - Window Open or Change event,
# - Window Layout Change event,
# - Cast Started or Changed event,
# happened, a cooldown is set,
# that doesn't procecss the next 3 of these events.
repeated_event_prevention = true
# === configure different events below

# event_name  = [state, strength, length]
# state = \"off\", \"on\"
# strength = \"low\", \"medium\", \"high\", \"custom_a\", \"custom_b\"
# length = \"short\", \"medium\", \"long\", \"custom_a\", \"custom_b\"

blank = [\"on\", \"low\", \"short\"]

# workspace
workspace_focus_changed = [\"on\", \"low\", \"short\"]
workspace_changed = [\"on\", \"low\", \"short\"]

# windows
window_opened_or_changed = [\"on\", \"low\", \"short\"]
window_closed = [\"on\", \"high\", \"medium\"]
window_focus_changed = [\"on\", \"low\", \"short\"]
window_layout_changed = [\"on\", \"medium\", \"short\"]
window_urgency_changed = [\"on\", \"high\", \"long\"]

# keyboard
keyboard_layout_changed = [\"on\", \"low\", \"short\"]

# overview
overview_opened = [\"on\", \"high\", \"medium\"]
overview_closed = [\"on\", \"medium\", \"medium\"]

# casts
screenshot_captured = [\"on\", \"medium\", \"short\"]
cast_started_or_changed = [\"on\", \"medium\", \"short\"]
cast_stopped = [\"on\", \"low\", \"short\"]

# niri's config relaod not sense's!
niri_config_reloaded = [\"off\", \"low\", \"short\"]";

  let config_location: String = format!("{}/niri/sense.toml", config_dir().unwrap().into_string().unwrap());

  let res = std::fs::write(config_location, DEFAULT_CONFIG_STR);
}

fn deserialize_server(table: Table) -> Option<SenseConfigServer> {
  debug!("des server");

  let address = table.get("address")?.as_str()?;
  let port = table.get("port")?.as_integer()?;
  Some(SenseConfigServer{
    address: address.parse().unwrap(),
    port: port as u16
  })
}
fn deserialize_lengths(table: Table) -> Option<SenseConfigLengths> {
  debug!("des lengths");

  let short = table.get("short")?.as_integer()?;
  let medium = table.get("medium")?.as_integer()?;
  let long = table.get("long")?.as_integer()?;
  let custom_a = table.get("custom_a")?.as_integer()?;
  let custom_b = table.get("custom_b")?.as_integer()?;

  Some(SenseConfigLengths{
    short: short as u64,
    medium: medium as u64,
    long: long as u64,
    custom_a: custom_a as u64,
    custom_b: custom_b as u64
  })
}
fn deserialize_strengths(table: Table) -> Option<SenseConfigStrengths> {
  debug!("des strengths");

  let low = table.get("low")?.as_integer()?;
  let medium = table.get("medium")?.as_integer()?;
  let high = table.get("high")?.as_integer()?;
  let custom_a = table.get("custom_a")?.as_integer()?;
  let custom_b = table.get("custom_b")?.as_integer()?;

  Some(SenseConfigStrengths{
    low: low as u8,
    medium: medium as u8,
    high: high as u8,
    custom_a: custom_a as u8,
    custom_b: custom_b as u8
  })
}

fn _des_match_str_bool(s: &str) -> Option<bool> {
  match s {
    "on" => Some(true),
    "off" => Some(false),
    _ => None
  }
}
fn _des_match_str_sense_vibration_length(s: &str) -> Option<SenseVibrationLength> {
  match s {
    "short" => Some(SenseVibrationLength::Short),
    "medium" => Some(SenseVibrationLength::Medium),
    "long" => Some(SenseVibrationLength::Long),
    "custom_a" => Some(SenseVibrationLength::CustomA),
    "custom_b" => Some(SenseVibrationLength::CustomB),
    _ => None
  }
}
fn _des_match_str_sense_vibration_strength(s: &str) -> Option<SenseVibrationStrength> {
  match s {
    "low" => Some(SenseVibrationStrength::Low),
    "medium" => Some(SenseVibrationStrength::Medium),
    "high" => Some(SenseVibrationStrength::High),
    "custom_a" => Some(SenseVibrationStrength::CustomA),
    "custom_b" => Some(SenseVibrationStrength::CustomB),
    _ => None
  }
}
fn _des_array_to_sense_event_config(vec: &Vec<Value>) -> Option<SenseEventConfig>{
  let state = _des_match_str_bool(vec[0].as_str()?)?;
  let strength = _des_match_str_sense_vibration_strength(vec[1].as_str()?)?;
  let length = _des_match_str_sense_vibration_length(vec[2].as_str()?)?;

  debug!("des ev cfg {:?} -> {} {:?} {:?}", vec, state, strength, length);

  Some(SenseEventConfig{
    state, strength, length
  })
}
fn deserialize_events(table: Table) -> Option<SenseConfigEvents> {
  debug!("des events");

  Some(SenseConfigEvents{
    repeated_event_prevention: table.get("repeated_event_prevention")?.as_bool()?,
    workspace_focus_changed: _des_array_to_sense_event_config(table.get("workspace_focus_changed")?.as_array()?)?,
    workspace_changed: _des_array_to_sense_event_config(table.get("workspace_changed")?.as_array()?)?,
    window_opened_or_changed: _des_array_to_sense_event_config(table.get("window_opened_or_changed")?.as_array()?)?,
    window_closed: _des_array_to_sense_event_config(table.get("window_closed")?.as_array()?)?,
    window_focus_changed: _des_array_to_sense_event_config(table.get("window_focus_changed")?.as_array()?)?,
    window_layout_changed: _des_array_to_sense_event_config(table.get("window_layout_changed")?.as_array()?)?,
    window_urgency_changed: _des_array_to_sense_event_config(table.get("window_urgency_changed")?.as_array()?)?,
    keyboard_layout_changed: _des_array_to_sense_event_config(table.get("keyboard_layout_changed")?.as_array()?)?,
    overview_opened: _des_array_to_sense_event_config(table.get("overview_opened")?.as_array()?)?,
    overview_closed: _des_array_to_sense_event_config(table.get("overview_closed")?.as_array()?)?,
    screenshot_captured: _des_array_to_sense_event_config(table.get("screenshot_captured")?.as_array()?)?,
    cast_started_or_changed: _des_array_to_sense_event_config(table.get("cast_started_or_changed")?.as_array()?)?,
    cast_stopped: _des_array_to_sense_event_config(table.get("cast_stopped")?.as_array()?)?,
    niri_config_reloaded: _des_array_to_sense_event_config(table.get("niri_config_reloaded")?.as_array()?)?,
  })
}

fn deserialize(content: String) -> Option<SenseConfig> {
  let root = content.parse::<Table>().unwrap();

  let server: SenseConfigServer;
  let lengths: SenseConfigLengths;
  let strengths: SenseConfigStrengths;
  let events: SenseConfigEvents;

  debug!("start deserialize");

  if let Some(server_table) = root.get("server")?.as_table() {
    server = deserialize_server(server_table.clone())?;
  } else { return None; }
  if let Some(lengths_table) = root.get("lengths")?.as_table() {
    lengths = deserialize_lengths(lengths_table.clone())?;
  } else { return None; }
  if let Some(strengths_table) = root.get("strengths")?.as_table() {
    strengths = deserialize_strengths(strengths_table.clone())?;
  } else { return None; }
  if let Some(events_table) = root.get("events")?.as_table() {
    events = deserialize_events(events_table.clone())?;
  } else { return None; }

  debug!("deserialize oki!");

  Some(SenseConfig {
    server,
    lengths,
    strengths,
    events
  })
}

impl SenseConfig {
  pub fn load_or_save_default() -> Option<Self> {
    let config_location: String = format!("{}/niri/sense.toml", config_dir().unwrap().into_string().unwrap());

    let content: String;
    let res = std::fs::read_to_string(config_location.clone());
    match res {
      Ok(str) => content = str,
      Err(e) => {
        println!("could not find the config at {}, saving a new one.", config_location);
        dump_default_config();
        return Some(Self::default());
      }
    }

    if let Some(des) = deserialize(content) {
      Some(des)
    } else {
      eprintln!("an error occurred while parsing the config");
      Some(Self::default())
    }
  }
}

impl Default for SenseConfig {
  fn default() -> Self {
    Self {
      server: SenseConfigServer {
        address: "127.0.0.1".parse().unwrap(),
        port: 12345,
      },

      lengths: SenseConfigLengths {
        short: 80,
        medium: 160,
        long: 240,
        custom_a: 320,
        custom_b: 480,
      },

      strengths: SenseConfigStrengths {
        low: 20,
        medium: 50,
        high: 80,
        custom_a: 10,
        custom_b: 100,
      },

      events: SenseConfigEvents {
        repeated_event_prevention: true,
        workspace_focus_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        workspace_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        window_opened_or_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        window_closed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        window_focus_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        window_layout_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        window_urgency_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        keyboard_layout_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        overview_opened: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        overview_closed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        screenshot_captured: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        cast_started_or_changed: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        cast_stopped: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
        niri_config_reloaded: SenseEventConfig { state: true, length: SenseVibrationLength::Short, strength: SenseVibrationStrength::Low },
      }

    }
  }
}