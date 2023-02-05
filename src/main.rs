use std::{str::FromStr};
use secp256k1::{PublicKey, SecretKey};
use anyhow::Result;
use web3::{self, Web3, transports::Http, types::{Address, TransactionParameters, H256}};
use tokio;

use fltk::{app::{App, channel}, button::Button, enums::{Color, Font, FrameType}, window::Window, frame::Frame, input::Input, prelude::*};

use crate::wallet_lib::{create_transaction_object, sign_and_send};

mod wallet_lib;

const URL: &str = "https://eth-goerli.g.alchemy.com/v2/r7OrFgAJkJ3km6iC2NOz7I8Ie72gofdt";

#[derive(Clone, Debug)]
pub enum WalletMessage {
    NewWallet,
    Send,
}

#[tokio::main]
async fn main() {
    let app: App = App::default();
    let mut window: Window = Window::default().with_size(1600, 900).with_label("Simple-Wallet");
    let mut btn_create_wallet: Button = Button::new(195, 450, 120, 45, "Create Wallet");
    let mut btn_send: Button = Button::new(200, 300, 100, 35, "SEND");
    let mut input1: Input = Input::new(200, 200, 225, 35, "To: ");
    let mut frame: Frame = Frame::default()
                                    .with_size(600, 300)
                                    .with_label("0 Wallets");
    {
        window.set_color(Color::DarkCyan);
        
        btn_create_wallet.set_color(Color::White);
        btn_create_wallet.set_label_color(Color::DarkMagenta);
        btn_create_wallet.set_label_font(Font::TimesBold);
        btn_create_wallet.set_frame(FrameType::FlatBox);
        btn_create_wallet.clear_visible_focus();
        
        btn_send.set_color(Color::White);
        btn_send.set_label_color(Color::DarkMagenta);
        btn_send.set_label_font(Font::TimesBold);
        btn_send.set_frame(FrameType::FlatBox);
        btn_send.clear_visible_focus();
        
        input1.set_frame(FrameType::FlatBox);
        
        frame.set_label_color(Color::White);
        frame.set_label_font(Font::TimesBold);
        frame.set_label_size(24);
    }

    window.end();
    window.show();

    let (sender, receiver) = channel::<WalletMessage>();

    btn_create_wallet.emit(sender.clone(), WalletMessage::NewWallet);
    btn_send.emit(sender, WalletMessage::Send);

    let web3: Result<Web3<Http>> = wallet_lib::establish_web3_connection(URL);
    let mut keypairs: Vec<(PublicKey, SecretKey)> = Vec::new();

 
    while app.wait() {
        if let Some(message) = receiver.recv() {
            match message {
                WalletMessage::NewWallet => {
                    let (secret_key, public_key) = match wallet_lib::create_keypair() {
                        Ok(value) => value,
                        Err(_error) => unimplemented!(),
                    };

                    keypairs.push((public_key, secret_key));

                    frame.set_label(&format!("{} Wallets", keypairs.len()));

                    println!("keypairs {:?}", keypairs);
                },

                WalletMessage::Send => {
                    let to_address = Address::from_str(&input1.value().as_str());
                    let transaction_object: Result<TransactionParameters> = create_transaction_object(to_address.unwrap(), 7);

                    let web3_clone: Web3<Http> = web3.as_ref().unwrap().to_owned();
                    // let web3_clone2 = web3.as_ref().to_owned().unwrap();

                    let result = sign_and_send(web3_clone, transaction_object.unwrap(), keypairs[0].1).await;

                    match result {
                        Ok(value) => frame.set_label(&format!("{}", value)),
                        Err(error) => frame.set_label(&format!("{}", error))
                    };
                }
            };
        };
    }

    app.run().unwrap();
}