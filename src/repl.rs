use std::io;
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::integration::Integration;

pub struct Repl {
  integration: Arc<Mutex<Integration>>,
  running: bool
}

impl Repl {
  pub fn new(integration: Arc<Mutex<Integration>>) -> Self {
    Self {
      integration,
      running: true,
    }
  }

  pub(crate) async fn run(&mut self) {
    let mut c = String::new();
    const PS1: &str = "$> ";

    println!("type help to see available commands.");

    while self.running {
      c.clear();

      print!("{}", PS1);
      let _ = io::stdout().flush();

      io::stdin().read_line(&mut c).expect("failed to read");
      c = c.trim_end().parse().unwrap();

      self.process_command(&c).await;
    }
  }

  async fn process_command(&mut self, cmd: &str) {
    match cmd {
      "help" => print_help(),
      "connect" => self._cmd_connect().await,
      "disconnect" => self._cmd_disconnect().await,
      "scan start" => self._cmd_scan_start().await,
      "scan stop" => self._cmd_scan_stop().await,
      "list" => self._cmd_list().await,
      "select" => self._cmd_select().await,
      "test" => self._cmd_test().await,
      "stop" => self._cmd_stop().await,
      "pause" => self._cmd_pause().await,
      "reload" => self._cmd_reload().await,
      "exit" => self._cmd_exit().await,
      _ => println!("invalid command: \"{}\"", cmd)
    }
  }

  // cmds
  async fn _cmd_connect(&self) {
    let mut i = self.integration.lock().await;
    let _ = i.connect().await;
  }
  async fn _cmd_disconnect(&self) {
    let mut i = self.integration.lock().await;
    let _ = i.disconnect().await;
  }
  async fn _cmd_scan_start(&self) {
    let mut i = self.integration.lock().await;
    let _ = i.start_scan().await;
  }
  async fn _cmd_scan_stop(&self) {
    let mut i = self.integration.lock().await;
    let _ = i.stop_scan().await;
  }
  async fn _cmd_list(&self) {
    let mut i = self.integration.lock().await;
    let _ = i.get_devices().await;
  }
  async fn _cmd_select(&self) {
    println!("enter the device index, or use -1 for all devices");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("failed to read");
    id = id.trim_end().parse().unwrap();

    let mut id: i8 = id.parse().unwrap_or(-1);
    if id < -1 {
      id = -1;
    }

    println!("selected: {}", id);

    let i = self.integration.lock().await;
    let _ = i.select_device(id).await;
  }
  async fn _cmd_test(&self) {
    let mut i = self.integration.lock().await;
    let _ = i.send_test().await;
  }
  async fn _cmd_stop(&self) {
    let i = self.integration.lock().await;
    let _ = i.stop_all().await;
  }
  async fn _cmd_pause(&self) {
    let i = self.integration.lock().await;
    let _ = i.toggle_pause()  .await;
  }
  async fn _cmd_reload(&self) {
    let mut i = self.integration.lock().await;
    let _ = i.reload_config()  .await;
  }
  async fn _cmd_exit(&mut self) {
    self.running = false;
    let mut i = self.integration.lock().await;
    let _ = i.exit().await;
    // i don't want to uh uh uh uh break niri,
    // cuz quitting with ctrl+c broke it,
    // unless it's because i'm running it under plasma bwehh
    tokio::time::sleep(Duration::from_millis(250)).await;
  }
}

fn print_help() {
  // lazy bweh :p
  println!(" - connect - connects to the server\n - disconnect - disconnects from the server\n - scan start - starts device scan\n - scan stop - stops device scan\n - list - lists devices\n - select - select a device\n - test - runs a series of vibration for testing\n - stop - stops all events\n - pause - toggles pause\n - reload - reloads config\n - exit - exits on next event.");
}