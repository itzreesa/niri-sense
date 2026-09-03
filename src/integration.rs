use buttplug_client::{connector::ButtplugRemoteClientConnector, serializer::ButtplugClientJSONSerializer, ButtplugClient, ButtplugClientDevice, ButtplugClientError, ButtplugClientEvent};
use buttplug_core::errors::ButtplugError;
use buttplug_transport_websocket_tungstenite::ButtplugWebsocketClientTransport;
use futures_util::StreamExt;
use std::time::Duration;
use buttplug_client::device::{ClientDeviceOutputCommand};
use buttplug_core::message::{InputType};
use log::{debug, error, info};
use niri_ipc::Event;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::config::{SenseConfig, SenseEventConfig};
use crate::util::{SenseVibrationLength, SenseVibrationStrength};
pub struct Integration {
  client: ButtplugClient,
  config: SenseConfig,
  paused: Mutex<bool>,
  device_selected: Mutex<i8>,
  skip_event_count: u8,
  pub quit_flag: bool
}

impl Integration {
  pub async fn connect(&mut self) -> anyhow::Result<()> {
    let addr = format!("ws://{}:{}", self.config.server.address, self.config.server.port);

    let connector = ButtplugRemoteClientConnector::<
      ButtplugWebsocketClientTransport,
      ButtplugClientJSONSerializer,
    >::new(ButtplugWebsocketClientTransport::new_insecure_connector(
      &addr.as_str(),
    ));

    self.client = ButtplugClient::new("Niri Sense Client");

    println!("connecting to {}", addr);

    if let Err(e) = self.client.connect(connector).await {
      return match e {
        ButtplugClientError::ButtplugConnectorError(error) => {
          error!("can't connect, exiting! {}", error);
          Ok(())
        }
        ButtplugClientError::ButtplugError(error) => match error {
          ButtplugError::ButtplugHandshakeError(error) => {
            error!("handshake issue, exiting! {}", error);
            Ok(())
          }
          error => {
            error!("error! {}", error);
            Ok(())
          }
        },
        _ => {
          Ok(())
        }
      }
    }

    println!("connected!");

    Ok(())
  }

  pub async fn start_scan(&mut self) {
    if !self.client.connected() {
      info!("not connected to a server, launch a server and type connect");
      return
    }

    let _ = self.client.start_scanning().await;

    let mut events = self.client.event_stream();
    tokio::spawn(async move {
      while let Some(event) = events.next().await {
        match event {
          ButtplugClientEvent::DeviceAdded(device) => {
            info!("new device: {}", device.name());
          }
          ButtplugClientEvent::DeviceRemoved(info) => {
            info!("disconnect: {}", info.name());
          }
          ButtplugClientEvent::ScanningFinished => {
            info!("device scan finished.");
          }
          _ => {}
        }
      }
    });

    info!("scan started.");
    sleep(Duration::from_millis(100)).await; // give it time ig
  }

  pub async fn stop_scan(&mut self) {
    let _ = self.client.stop_scanning().await;
    info!("scan stopped.");
  }

  pub async fn get_devices(&mut self) {
    if !self.client.connected() {
      info!("not connected to a server, launch a server and type connect");
      return
    }

    for (_, device) in self.client.devices() {
      println!("\n-- Index: {}", device.index());
      println!("  Display Name: {:?} / {}", device.display_name(), device.name());
      if device.input_available(InputType::Battery) {
        match device.battery().await {
          Ok(battery) => println!("  {}% battery", battery),
          Err(e) => println!("  could not read battery - {}", e),
        }
      } else {
        println!("  no battery sensor");
      }

      // Get all features for this device
      let features = device.device_features();

      println!("\nFeatures ({} total):", features.len());
      for (feature_index, feature) in features {
        let feature_def = feature.feature();
        println!("\n  Feature {}:", feature_index);
        println!("    Description: {:?}", feature_def.description());
      }
    }

  }

  pub async fn send_test(&mut self) {
    if !self.client.connected() {
      info!("not connected to a server, launch a server and type connect");
      return
    }

    if *self.paused.lock().await {
      debug!("not processing action, paused");
      return;
    }

    self.vibrate_action(SenseVibrationStrength::Low, SenseVibrationLength::Short).await;
    self.vibrate_action(SenseVibrationStrength::Medium, SenseVibrationLength::Short).await;
    self.vibrate_action(SenseVibrationStrength::High, SenseVibrationLength::Short).await;

    sleep(Duration::from_millis(500)).await;

    self.vibrate_action(SenseVibrationStrength::Low, SenseVibrationLength::Medium).await;
    self.vibrate_action(SenseVibrationStrength::Medium, SenseVibrationLength::Medium).await;
    self.vibrate_action(SenseVibrationStrength::High, SenseVibrationLength::Medium).await;

    sleep(Duration::from_millis(500)).await;

    self.vibrate_action(SenseVibrationStrength::Low, SenseVibrationLength::Long).await;
    self.vibrate_action(SenseVibrationStrength::Medium, SenseVibrationLength::Long).await;
    self.vibrate_action(SenseVibrationStrength::High, SenseVibrationLength::Long).await;
  }

  pub async fn disconnect(&mut self) {
    self.client.disconnect().await.expect("device disconnect failed");
    info!("disconnected");
  }

  fn _enum_length(&self, length: SenseVibrationLength) -> u64 {
    match length {
      SenseVibrationLength::Short => self.config.lengths.short,
      SenseVibrationLength::Medium => self.config.lengths.medium,
      SenseVibrationLength::Long => self.config.lengths.long,
      SenseVibrationLength::CustomA => self.config.lengths.custom_a,
      SenseVibrationLength::CustomB => self.config.lengths.custom_b
    }
  }

  fn _enum_strength(&self, strength: SenseVibrationStrength) -> f64 {
    match strength {
      SenseVibrationStrength::Low => (self.config.strengths.low as f64) / 100.0f64,
      SenseVibrationStrength::Medium => (self.config.strengths.medium  as f64) / 100.0f64,
      SenseVibrationStrength::High => (self.config.strengths.high  as f64) / 100.0f64,
      SenseVibrationStrength::CustomA => (self.config.strengths.custom_a  as f64) / 100.0f64,
      SenseVibrationStrength::CustomB => (self.config.strengths.custom_b  as f64) / 100.0f64
    }
  }

  pub async fn vibrate_action(&mut self,
                              sense_vibration_strength: SenseVibrationStrength,
                              sense_vibration_length: SenseVibrationLength)
  {
    if !self.client.connected() {
      debug!("not processing action, device not connected");
      return;
    }

    if *self.paused.lock().await {
      debug!("not processing action, paused");
      return;
    }

    let strength = self._enum_strength(sense_vibration_strength);
    let length = self._enum_length(sense_vibration_length);

    debug!("running vibration of strength {} for {}", strength, length);

    self._set_multiple(strength).await;
    sleep(Duration::from_millis(length)).await;
    self._set_multiple(0.0f64).await;
  }

  async fn _set_multiple(&self, value: f64) {
    let device_index = self.device_selected.lock().await;
    let devices: Vec<ButtplugClientDevice> = self.client.devices().into_values().collect();

    let idx = *device_index as usize;

    if *device_index != -1 {
      if devices.len() >= idx + 1 {
        let dev = &devices[idx];
        let _ = dev.run_output(&ClientDeviceOutputCommand::Vibrate(value.into()), ).await;
      }
      return;
    }

    // if selected index is invalid or explicitly selected to -1, then vibrate all
    for dev in &devices {
      let _ = dev.run_output(&ClientDeviceOutputCommand::Vibrate(value.into()), ).await;
    }
  }

  async fn _run_mapped_event(&mut self, e: SenseEventConfig, check_for_skip: bool) {
    if check_for_skip && self.config.events.repeated_event_prevention {
      if self.skip_event_count > 0 {
        self.skip_event_count -= 1;
        return;
      }
    }

    //debug!("_run_mapped_event {} {:?} {:?}", e.state, e.strength, e.length);
    if !e.state {
      return;
    }
    self.vibrate_action(e.strength, e.length).await;

    self.skip_event_count = 3;
  }

  pub(crate) async fn event_to_integration(&mut self, e: Event) {
    let cfg = &self.config.events;
    match e {
      Event::WorkspaceActivated { .. } => self._run_mapped_event(cfg.workspace_focus_changed.clone(), false).await,
      Event::WorkspacesChanged { .. } => self._run_mapped_event(cfg.workspace_changed.clone(), false).await,

      Event::WindowOpenedOrChanged { .. } => self._run_mapped_event(cfg.window_opened_or_changed.clone(), true).await,
      Event::WindowClosed { .. } => self._run_mapped_event(cfg.window_closed.clone(), false).await,
      Event::WindowFocusChanged { id } => {
        if id.is_none() {
          return;
        }
        self._run_mapped_event(cfg.window_focus_changed.clone(), false).await
      },
      Event::WindowLayoutsChanged { .. } => self._run_mapped_event(cfg.window_layout_changed.clone(), true).await,
      Event::WindowUrgencyChanged { .. } => self._run_mapped_event(cfg.window_urgency_changed.clone(), false).await,

      Event::KeyboardLayoutsChanged { .. } => self._run_mapped_event(cfg.keyboard_layout_changed.clone(), false).await,

      Event::OverviewOpenedOrClosed { is_open} => {
        if is_open {
          self._run_mapped_event(cfg.overview_opened.clone(), false).await;
        } else {
          self._run_mapped_event(cfg.overview_closed.clone(), false).await;
        }
      }

      Event::ScreenshotCaptured { .. } => self._run_mapped_event(cfg.screenshot_captured.clone(), false).await,
      Event::CastStartedOrChanged { .. } => self._run_mapped_event(cfg.cast_started_or_changed.clone(), true).await,
      Event::CastStopped { .. } => self._run_mapped_event(cfg.cast_stopped.clone(), false).await,

      Event::ConfigLoaded { .. } => self._run_mapped_event(cfg.niri_config_reloaded.clone(), false).await,
      _ => {}
    }

    debug!("response from niri: {:?}", e);
  }

  pub async fn stop_all(&self) {
    println!("stopping all devices");
    let _ = self.client.stop_all_devices().await;
  }

  pub async fn select_device(&self, new_id: i8) {
    let mut id = self.device_selected.lock().await;
    *id = new_id;
  }

  pub async fn toggle_pause(&self) {
    let mut p = self.paused.lock().await;
    *p = !*p;
    println!("pause is now: {}", p);
  }

  pub async fn reload_config(&mut self) {
    println!("reloading config! will there be an error, the defaults will be set");
    self.config = SenseConfig::load_or_save_default().unwrap();
  }

  pub async fn exit(&mut self) {
    let _ = self.client.disconnect().await;
    self.quit_flag = true;
  }

  pub fn new(config: SenseConfig) -> Integration {
    let client = ButtplugClient::new("Niri Sense Client");

    Integration {
      client,
      config,
      paused: Mutex::new(false),
      device_selected: Mutex::new(-1),
      skip_event_count: 0,
      quit_flag: false
    }
  }
}