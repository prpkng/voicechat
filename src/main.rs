mod app;
mod net;

use std::{rc::Rc, time::Duration};

use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{Multiaddr, SwarmBuilder, ping};
use libp2p::{noise, tcp, yamux};

use crate::app::VcApp;

fn main() {
    env_logger::init();

    let window_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    let runtime = Rc::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap(),
    );

    runtime.block_on(async move {
        let mut swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .unwrap()
            .with_behaviour(|_| ping::Behaviour::default())
            .unwrap()
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();

        swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .unwrap();

        if let Some(addr) = std::env::args().nth(1) {
            let remote: Multiaddr = addr.parse().unwrap();

            swarm.dial(remote).unwrap();

            println!("Dialed {addr}");
        }

        loop {
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                SwarmEvent::Behaviour(event) => println!("{event:?}"),
                _ => {}
            }
        }
    });

    // let rt1 = runtime.clone();
    // let rt2 = runtime.clone();

    // eframe::run_native(
    //     "Voice Chat",
    //     window_options,
    //     Box::new(|ctx| {
    //         let mut app = Box::<VcApp>::default();

    //         app.set_host(move || {
    //             rt1.spawn(async move {});
    //         });

    //         app.set_join(move || {
    //             rt2.spawn(async move {});
    //         });

    //         Ok(app)
    //     }),
    // )
    // .expect("Failed to create eframe app!");

    // println!(
    //     "Runtime is running {} tasks",
    //     runtime.handle().metrics().num_alive_tasks()
    // );

    // while runtime.handle().metrics().num_alive_tasks() > 0 {
    //     std::thread::sleep(Duration::from_millis(1000));
    //     println!("Waiting...");
    // }
    // tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async move {
    //         let router = start_accept_side().await.unwrap();

    //         let node_addr = router.endpoint().node_addr().initialized().await.unwrap();

    //         tokio::spawn(async move {
    //             connect_side(node_addr).await.unwrap();
    //         })
    //         .await
    //         .unwrap();

    //         router.shutdown().await.unwrap();
    //     });
}
