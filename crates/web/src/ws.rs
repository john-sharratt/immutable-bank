#![allow(unused_imports)]
use futures::stream::StreamExt;
use futures::FutureExt;
use futures::SinkExt;
use pharos::*;
use std::sync::mpsc;

#[cfg(not(target_arch = "wasm32"))]
use tokio::task::spawn_local;

use crate::sleep;

#[cfg(target_arch = "wasm32")]
use {wasm_bindgen::UnwrapThrowExt, wasm_bindgen_futures::spawn_local, ws_stream_wasm::*};

pub struct WebSocket {
    tx: futures::channel::mpsc::UnboundedSender<Vec<u8>>,
    rx: mpsc::Receiver<Vec<u8>>,
}

impl Default for WebSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocket {
    pub fn new() -> Self {
        let (tx1, mut rx1) = futures::channel::mpsc::unbounded();
        let (tx2, rx2) = mpsc::channel();

        spawn_local(async move {
            let mut backoff = 500;
            loop {
                let mut do_err = |err| {
                    log::error!("failed to connect - {}", err);
                    backoff += 500;
                    if backoff > 2000 {
                        backoff = 2000;
                    }
                    sleep::sleep(backoff)
                };

                let uri = if cfg!(debug_assertions) {
                    "ws://localhost:8001"
                } else {
                    "ws://immutable-bank.com:8001"
                };

                log::info!("connecting to {}", uri);
                let (mut ws, mut wsio) = match WsMeta::connect(uri, None).await {
                    Ok(a) => a,
                    Err(err) => {
                        do_err(err).await;
                        continue;
                    }
                };
                log::info!("connected to {}", uri);

                // Loop processing and receiving data
                let mut close_evts = ws
                    .observe(Filter::Pointer(WsEvent::is_closed).into())
                    .await
                    .expect_throw("observe");
                loop {
                    futures::select! {
                        msg = rx1.next() => {
                            if let Some(msg) = msg {
                                if let Err(err) = wsio.send(WsMessage::Binary(msg)).await {
                                    do_err(err).await;
                                    continue;
                                }
                            }
                        },
                        _ = close_evts.next().fuse() => {
                            log::error!("received close event - terminating web socket");
                            break;
                        }
                        msg = wsio.next().fuse() => {
                            match msg {
                                Some(WsMessage::Binary(data)) => {
                                    let _ = tx2.send(data);
                                }
                                Some(_) => { }
                                None => {
                                    log::error!("no more messages on the web socket - terminating");
                                    break;
                                }
                            }

                        }
                    }
                }

                log::info!("closing connection");
                futures::select! {
                    _ = ws.close().fuse() => { },
                    _ = sleep::sleep(2000).fuse() => { }
                };

                backoff = 0;
            }
        });

        Self { tx: tx1, rx: rx2 }
    }

    pub fn send(&mut self, data: Vec<u8>) {
        self.tx.unbounded_send(data).unwrap();
    }

    pub fn try_recv(&mut self) -> Option<Vec<u8>> {
        self.rx.try_recv().ok()
    }
}