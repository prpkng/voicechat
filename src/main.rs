mod app;
mod net;

use anyhow::Result;
use iroh::Watcher;
use tokio::runtime::{self, Runtime};

use crate::{
    app::VcApp,
    net::{connect_side, start_accept_side},
};

fn main() {
    env_logger::init();

    // let window_options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
    //     ..Default::default()
    // };

    // eframe::run_native(
    //     "Voice Chat",
    //     window_options,
    //     Box::new(|ctx| Ok(Box::<VcApp>::default())),
    // )
    // .expect("Failed to create eframe app!");
    //

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let router = start_accept_side().await.unwrap();

            let node_addr = router.endpoint().node_addr().initialized().await.unwrap();

            tokio::spawn(async move {
                connect_side(node_addr).await.unwrap();
            })
            .await
            .unwrap();

            router.shutdown().await.unwrap();
        });
}
