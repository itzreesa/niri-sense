use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use anyhow::anyhow;
use niri_ipc;
use log::{debug, error, info};
use niri_ipc::{Event, Reply, Request, Response};
use tokio::sync::Mutex;
use crate::config::SenseConfig;
use crate::integration::Integration;

pub struct SenseApp {
  niri_socket: Option<niri_ipc::socket::Socket>,
  integration: Arc<Mutex<Integration>>,
  running: bool
}
impl SenseApp {
  pub fn new(socket_addr: String, integration: Arc<Mutex<Integration>>) -> Result<Self, anyhow::Error> {
    let socket = niri_ipc::socket::Socket::connect_to(socket_addr.clone());
    return match socket {
      Ok(s) => {
        println!("-- socket connected at: {:?}", socket_addr);

        Ok(
          Self {
            niri_socket: Some(s),
            integration,
            running: false
          }
        )
      }
      Err(e) => {
        let s = format!("could not connect to niri's ipc: {}", e);
        error!("{}", s);
        return Err(anyhow!(s));
      }
    }
  }

  pub fn setup_stream(&mut self) -> Result<(), anyhow::Error> {
    let ev_req = self.niri_socket.as_mut().unwrap().send(Request::EventStream);
    match ev_req {
      Ok(n) => {
        match n {
          Ok(reply) => { self.running = true; Ok(()) },
          Err(e) => {
            let s = format!("niri error while setting up a stream: {}" ,e);
            error!("{}", s);
            return Err(anyhow!(s));
          }
        }
      },
      Err(e) => {
        let s = format!("could not request event stream from niri! {}", e);
        error!("{}", s);
        return Err(anyhow!(s));
      }
    }
  }

  pub async fn loop_stream(&mut self) {
    let mut read_event = self.niri_socket.take().unwrap().read_events();

    while self.running {
      let event = read_event();
      if matches!(event, Result::Err(..)) {
        continue
      }

      let e = event.unwrap();
      let mut int = self.integration.lock().await;
      int.event_to_integration(e).await;
    }
  }
}