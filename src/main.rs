mod read_ext;
mod store;

use std::fs::File;

fn main() {
    println!("Hello, world!");

    let mut file = File::open("/Users/leobernard/Desktop/store.db").unwrap();
    // let header = store::Header::read_from(&mut file);
    // let block0 = store::Block0::read_from(&mut file);
    let store = store::Store::read_from(&mut file).unwrap();

    println!("{:#?}", store);
}
