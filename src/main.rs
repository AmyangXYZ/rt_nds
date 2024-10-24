use rt_nds::{Client, Server};
use std::thread;
use std::time::Duration;

fn main() {
    let server_thread = thread::spawn(|| {
        let mut server = Server::new("127.0.0.1:7777");
        server.run();
    });

    let client_thread = thread::spawn(|| {
        thread::sleep(Duration::from_millis(200));

        let client = Client::new(1 as u64, "127.0.0.1:7777");
        client.set("test/test.txt", vec![1, 2, 3, 4, 5]);

        let data = client.get("test/test.txt1");
        println!("{:?}", data);
    });

    server_thread.join().unwrap();
    client_thread.join().unwrap();
}
