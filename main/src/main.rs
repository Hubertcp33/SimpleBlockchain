use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChian::new_blockchain();

    println!("starting mining ...");
    thread::sleep(Duration::from_secs(3));
    bc.add_block(String::from("a -> b : 5 btc"));
    println!(" produce a block!");

    println!("");
    println!("starting mining ...");
    thread::sleep(Duration::from_secs(3));
    bc.add_block("b -> c : 10 btc".to_string());
    println!(" produce a block!");
    for b in bc.blocks {
        println!("+++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
