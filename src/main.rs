use std::str::from_utf8;
use tokio::{io::AsyncReadExt, select, sync::mpsc::channel};

//FIXME: Fixa så att KrakenAPI fungerar, denna skall sedan importeras
//i mina andra project, fullt fungerande API?! hur?
//
#[tokio::main]
async fn main() {
    let mut api = kraken_api::Api::new();
    api.connect_to_socket().unwrap();
    //HACK: needed becuase read_message() dont yield and no messages are coming in
    api.subscribe("BTC/USD");

    let (tx, mut rx) = channel::<String>(10);
    let (command_tx, mut command_rx) = channel::<String>(10);
    // take input from the user to be able to subscribe

    tokio::spawn(async move {
        loop {
            select! {

                msg = api.read_message() => {
                    tx.send(msg.to_text().unwrap().to_string()).await.unwrap();
                }
                command = command_rx.recv() => {
                    api.subscribe(command.unwrap().as_str())
                }

            }
        }
    });

    let (tx2, mut rx2) = channel::<String>(10);
    tokio::spawn(async move {
        loop {
            let msg = read_user_input().await;
            tx2.send(msg).await.unwrap();
        }
    });

    loop {
        select! {
            message_from_api = rx.recv() => {
                println!("message: {}", message_from_api.unwrap());
            }
            input = rx2.recv() => {
                //TODO: Parse the string messaqges -command -args
                //subscribe btc/USD
                //
                command_tx.send(input.unwrap()).await.unwrap();

            }
        }
    }
}

async fn read_user_input() -> String {
    let mut buf: [u8; 512] = [0; 512];
    let end = tokio::io::stdin().read(&mut buf).await.unwrap();
    from_utf8(&buf[..end - 1]).unwrap().to_string()
}
