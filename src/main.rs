use rt_nds::{Client, Server};
use std::fs::File;
use std::io::Read;
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
        let mut file = File::open("/dev/urandom").expect("Failed to open /dev/urandom");
        let mut data = [0u8; 1024]; // Adjust the size as needed
        file.read_exact(&mut data)
            .expect("Failed to read random data");

        client.set("test/test.txt", data.to_vec(), 10);

        let data = client.get("test/test.txt");
        println!("{:?}", data);
    });

    server_thread.join().unwrap();
    client_thread.join().unwrap();
}
