use kraken_api::Api;
fn main() {
    let mut api = kraken_api::Api::new();
    api.connect_to_socket();
}
