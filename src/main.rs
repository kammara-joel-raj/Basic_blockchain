use blockchainlib::*;

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ], 
            outputs: vec![
                transaction::Output {
                    to_addr: "Dad".to_owned(),
                    value: 50, 
                },
                transaction::Output {
                    to_addr: "Mom".to_owned(),
                    value: 7, 
                },
            ],
        },
    ], difficulty);

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    // --- BLOCK 1 ---
    let mut block = Block::new(1, now(), last_hash.clone(), vec![ 
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Joel".to_owned(),
                    value: 58, // This is the block reward + fees
                },
            ],
        },
        Transaction {
            inputs: vec![
                // Input: Alice's 50 output from the Genesis Block
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                // FIX: Values must be <= 50 (the input value).
                transaction::Output {
                    to_addr: "Jayden".to_owned(),
                    value: 36, // Jayden gets 36
                },
                transaction::Output {
                    to_addr: "Noel".to_owned(),
                    value: 4, // Noel gets 4
                },
            ],
        }, 
    ], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add block");
    println!("\n--- FINAL BLOCKCHAIN STATE ---");
    println!("Total Blocks: {}", blockchain.blocks.len());
    println!("Successfully added {} blocks to the chain.", blockchain.blocks.len());
    println!("The chain is now validated and persisted in memory.");
}
