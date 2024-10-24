use rt_nds::{CacheEntry, Server};

fn main() {
    let mut server = Server::new();
    let name = String::from("/data/1");

    server.cache.set(CacheEntry::new(
        &name,
        vec![0x52, 0x54, 0x4E, 0x44, 0x53],
        60,
        5,
        123,
    ));

    println!("{:?}", server.cache.get(&name).unwrap().data);
}
