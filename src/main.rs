use kraken_api::Api;

fn main() {
    let mut api = Api::new();
    api.connect_to_socket().unwrap();

    loop {
        let msg = api.read_message();
        println!("{:?}", msg);
    }
}
