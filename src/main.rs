use data_encoding::HEXLOWER;
use log::LevelFilter;
use rust_blockchain::{
    Blockchain, Transaction, UTXOSet,
    command::{Command, Opt},
    config::GLOBAL_CONFIG,
    server::{CENTRAL_NODE, Server, send_tx},
    util,
    wallet::{
        ADDRESS_CHECK_SUM_LEN, Wallets,
        wallet_util::{convert_address, hash_pub_key, validate_address},
    },
};
use structopt::StructOpt;

const MINE_TRUE: usize = 1;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let opt = Opt::from_args();
    match opt.command {
        Command::Createblockchain { address } => {
            let blockchain = Blockchain::create_blockchain(address.as_str());
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            println!("Done!");
        }
        Command::Createwallet => {
            let mut wallet = Wallets::new();
            let address = wallet.create_wallet();
            println!("Your new address: {address}")
        }
        Command::GetBalance { address } => {
            let address_valid = validate_address(address.as_str());
            if !address_valid {
                panic!("ERROR: Address is not valid")
            }
            let payload = util::base58_decode(address.as_str());
            let pub_key_hash = &payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN];

            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            let utxos = utxo_set.find_utxo(pub_key_hash);
            let mut balance = 0;
            for utxo in utxos {
                balance += utxo.get_value();
            }
            println!("Balance of {address}: {balance}");
        }
        Command::ListAddresses => {
            let wallets = Wallets::new();
            for address in wallets.get_addresses() {
                println!("{address}")
            }
        }
        Command::Send {
            from,
            to,
            amount,
            mine,
        } => {
            if !validate_address(from.as_str()) {
                panic!("ERROR: Sender address is not valid")
            }
            if !validate_address(to.as_str()) {
                panic!("ERROR: Recipient address is not valid")
            }
            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain.clone());

            let transaction =
                Transaction::new_utxo_transaction(from.as_str(), to.as_str(), amount, &utxo_set);

            if mine == MINE_TRUE {
                let coinbase_tx = Transaction::new_coinbase_tx(from.as_str());

                let block = blockchain.mine_block(&[transaction, coinbase_tx]);

                utxo_set.update(&block);
            } else {
                send_tx(CENTRAL_NODE, &transaction);
            }
            println!("Success!")
        }
        Command::Printchain => {
            let mut block_iterator = Blockchain::new_blockchain().iterator();
            loop {
                let option = block_iterator.next();
                if option.is_none() {
                    break;
                }
                let block = option.unwrap();
                println!("Pre block hash: {}", block.get_pre_block_hash());
                println!("Cur block hash: {}", block.get_hash());
                println!("Cur block Timestamp: {}", block.get_timestamp());
                for tx in block.get_transactions() {
                    let cur_txid_hex = HEXLOWER.encode(tx.get_id());
                    println!("- Transaction txid_hex: {cur_txid_hex}");

                    if !tx.is_coinbase() {
                        for input in tx.get_vin() {
                            let txid_hex = HEXLOWER.encode(input.get_txid());
                            let pub_key_hash = hash_pub_key(input.get_pub_key());
                            let address = convert_address(pub_key_hash.as_slice());
                            println!(
                                "-- Input txid = {}, vout = {}, from = {}",
                                txid_hex,
                                input.get_vout(),
                                address,
                            )
                        }
                    }
                    for output in tx.get_vout() {
                        let pub_key_hash = output.get_pub_key_hash();
                        let address = convert_address(pub_key_hash);
                        println!("-- Output value = {}, to = {}", output.get_value(), address,)
                    }
                }
                println!()
            }
        }
        Command::Reindexutxo => {
            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            let count = utxo_set.count_transactions();
            println!("Done! There are {count} transactions in the UTXO set.");
        }
        Command::StartNode { miner } => {
            if let Some(addr) = miner {
                if !validate_address(addr.as_str()) {
                    panic!("Wrong miner address!")
                }
                println!("Mining is on. Address to receive rewards: {addr}");
                GLOBAL_CONFIG.set_mining_addr(addr);
            }
            let blockchain = Blockchain::new_blockchain();
            let sockert_addr = GLOBAL_CONFIG.get_node_addr();
            Server::new(blockchain).run(sockert_addr.as_str());
        }
    }
}
