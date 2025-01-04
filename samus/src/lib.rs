mod store;
mod store_value;
mod server;

use crate::server::Server;
use crate::store::Store;

pub fn init_and_start_server(port: i32) {
  let test_key = "test_key".to_string();
  let test_value = "test_value".to_string();
  let test_ttl = 0;
  let mut store = Store::new();
  store.set(&test_key, &test_value, &test_ttl).unwrap();
  let mut server = Server::new(port, &mut store);
  server.start().unwrap();
}



