use std::{
  net::TcpStream,
  time::Duration,
  thread::sleep,
  thread,
  io::{Read, Write},
};

#[test]
fn integration_test_server() {
  // Spawn our server in a seperate thread
  thread::spawn(|| {
    samus::init_and_start_server(6666)
  });

  // Give our server time to wake up. This isn't ideal, but it works for now and gives me
  // confidence in the server.
  sleep(Duration::new(1, 0));

  // Start a simple client
  let mut test_stream = TcpStream::connect("127.0.0.1:6666").unwrap();
  let mut test_buffer = Vec::with_capacity(48);

  // Send a GET request to our server
  test_stream.write_all(b"GET test_key\nDELETE test_key\nSET new_key new_value 100\nGET new_key\n").unwrap();
  test_stream.shutdown(std::net::Shutdown::Write).unwrap();
  test_stream.read_to_end(&mut test_buffer).unwrap();

  let buffer_string = std::str::from_utf8(&test_buffer).unwrap();
  assert_eq!(buffer_string, "test_value\ntest_key\nnew_value\nnew_value\n__TERM__");
}
