use std::time;

use rt_nds::Server;

fn main() {
    let mut server = Server::new();
    let name = String::from("/data/1");
    let data = vec![0x52, 0x54, 0x4E, 0x44, 0x53];

    server.cache.set(&name, data, time::Duration::from_secs(60));

    println!("{:?}", server.cache.get("/data/11"));
}
