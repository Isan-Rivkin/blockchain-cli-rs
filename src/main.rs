#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    
    print!("difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("difficulty must be an integer");

    println!("generating a new gensis block!" );
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(),diff);

    loop {
        println!("Menu:" );
        println!("1) new transaction" );
        println!("2) mine block!" );
        println!("3) change difficulty" );
        println!("4) change reward" );
        println!("0) exit" );
        println!("enter your choice:" );    
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("" );

        match choice.trim().parse().unwrap(){
            0 =>{
                println!("exiting!" );
                process::exit(0);
            },
            1 =>{
                let mut sender = String::new();
                let mut reciever = String::new();
                let mut amount = String::new();
                print!("enter sender address:");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("enter reciever address:" );
                io::stdout().flush();
                io::stdin().read_line(&mut reciever);
                print!("enter amount:" );
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), 
                reciever.trim().to_string(), 
                amount.trim().parse().unwrap());

                match res {
                    true => println!("tx was added! " ),
                    false => println!("tx failed!" )
                }
            },
            2 =>{
                println!("generating block" );
                let res = chain.generate_new_block();
                match res {
                    true => println!("block generated successfuly" ),
                    false => println!("block generation failed" ),
                }
            },
            3 =>{
                let mut new_diff = String::new();
                print!("enter new difficulty:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("updated difficulty! " ),
                    false => println!("failed update difficulty! " ),
                }
            },
            4 =>{
                let mut new_reward = String::new();
                print!("enter new reward:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_difficulty(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("updated reward! " ),
                    false => println!("failed update reward! " ),
                }
            },
            _ =>{
                println!("\t Invalid options please retry\t");
            },
        }
    }   
}
