// SOURCE : https://github.com/GeekLaunch/blockchain-rust/tree/master/src    

use blockchainlib::{Block, Hashable};
use std::collections::HashMap;


fn main() {
    let block = Block::new(0, 0, vec![0; 32], vec![0; 32], 0, "Genesis block".to_owned());
    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);

}