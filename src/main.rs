use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc}; 
use std::thread;
use std::time::Duration;
use rand::Rng;

mod models;
mod protocol;

use crate::models::{Account, Bank, Market, UserRole};
use crate::protocol::RwaProtocol;

pub fn start_market_oracle(protocol: Arc<Mutex<RwaProtocol>>){
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        
        loop {
            thread::sleep(Duration::from_secs(10));

            let shift: f64 = rng.gen_range(-1.0..1.0);
            let shift_dec = Decimal::from_f64(shift).unwrap_or(dec!(0.0));

            let mut data = protocol.lock().unwrap();
            data.gold_price_inr += shift_dec;
            
            if data.gold_price_inr < dec!(10.0) { data.gold_price_inr = dec!(60.0); }

            println!("\n[ORACLE UPDATE] New Gold Price: ${:.2}", data.gold_price_inr);
            print!("> "); 
            std::io::stdout().flush().unwrap();
        }
    });
}

pub fn start_yield_engine(protocol: Arc<Mutex<RwaProtocol>>) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60)); // simulating interest 

            let mut data = protocol.lock().unwrap();
            let interest_rate = dec!(0.0001); // 0.01% per minute

            for account in data.bank.accounts.values_mut() {
                if account.balance > dec!(0.0) {
                    let interest = account.balance * interest_rate;
                    account.balance += interest;
                }
            }
            
            println!("\n[YIELD] Interest distributed to all accounts.");
            print!("> ");
            io::stdout().flush().unwrap();
        }
    });
}

fn save_bank_vault(protocol: &RwaProtocol) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(protocol).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Serde error: {}", e))
    })?;
    
    let mut file = File::create("bank_vault.json")?;
    file.write_all(json.as_bytes())?;
    file.sync_all()?; // <--- This forces the OS to actually write the bits to the drive
    Ok(())
}

fn load_bank_vault() -> Option<RwaProtocol> {
    let file = File::open("bank_vault.json").ok()?;
    let protocol: RwaProtocol = serde_json::from_reader(file).ok()?;
    Some(protocol)
}

fn main() {
    println!("Rust finance");
    let protocol= load_bank_vault().unwrap_or_else(|| RwaProtocol::new());
    let shared_protocol = Arc::new(Mutex::new(protocol));

    start_market_oracle(Arc::clone(&shared_protocol));
    start_yield_engine(Arc::clone(&shared_protocol));
    
    loop {
        
        {
            let platform = shared_protocol.lock().unwrap();
            println!("\n--- MARKET STATUS ---");
            println!("Live Gold Price: ${:.2} | Global Supply: {}g", 
            platform.gold_price_inr, platform.total_token_supply);
        }

        print!("\n(1)-> Create Account (2)-> Deposit (3)-> Buy Gold (4)-> Sell Gold (5)-> Balance (6)-> Transfer Gold (7)-> Audit Log (8)-> Add Inventory (9)-> Generate report (10)-> Save & Exit\n");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        match choice.trim(){

            "1" => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                print!("Is this an admin? (y/n): ");
                io::stdout().flush().unwrap();
                let mut admin_choice = String::new();
                io::stdin().read_line(&mut admin_choice).unwrap();

                let role = if admin_choice.trim().to_lowercase() == "y" {
                    UserRole::Admin
                } else {
                    UserRole::Customer
                };
                let mut platform = shared_protocol.lock().unwrap();
                platform.bank.add_account(name, dec!(0.05), role.clone());
                println!("Account created for {} as {:?}", name, role);

            }

            "2" => {
                print!("Enter Account name: ");
                io::stdout().flush().unwrap();
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                
                let mut platform = shared_protocol.lock().unwrap();
                if let Some(account) = platform.bank.accounts.get_mut(name) {
                    print!("Amount to deposit: ");
                    io::stdout().flush().unwrap();
                    let mut amt = String::new();
                    io::stdin().read_line(&mut amt).unwrap();
                    
                    // Convert string to Decimal
                    if let Ok(amount) = Decimal::from_str(amt.trim()) {
                        account.deposit(amount);
                        println!("Deposited! New balance: ${}", account.balance);
                        save_bank_vault(&platform).expect("Failed to save!");
                    }
                } else {
                    println!("Account not found!");
                }
            }

            "3" => {
                print!("Enter Account Name: ");
                io::stdout().flush().unwrap();
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                print!("Amount to buy(grams): ");
                io::stdout().flush().unwrap();
                let mut amt = String::new();
                io::stdin().read_line(&mut amt).unwrap();

                let mut platform = shared_protocol.lock().unwrap();
                if let Ok(amount) = Decimal::from_str(amt.trim()) {
                    match platform.buy_tokens(&name, amount) {
                        Ok(_) => println!("Transaction confirmed on the ledger."),
                        Err(e) => println!("Transaction REJECTED: {}", e),                       
                    }
                    save_bank_vault(&platform).expect("Failed to save!");
                }

                
            }

            "4" =>{
                print!("Enter Account Name: ");
                io::stdout().flush().unwrap();
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                
                print!("Amount to sell(grams): ");
                io::stdout().flush().unwrap();
                let mut amt = String::new();
                io::stdin().read_line(&mut amt).unwrap();

                let mut platform = shared_protocol.lock().unwrap();
                if let Ok(amount) = Decimal::from_str(amt.trim()) {
                    match platform.sell_tokens(&name, amount) {
                        Ok(_) => println!("Tokens successfully liquidated for INR."),
                        Err(e) => println!("Transaction REJECTED: {}", e),
                    }
                    save_bank_vault(&platform).expect("Failed to save!");
                }
            }

            "5" => {
                let mut platform = shared_protocol.lock().unwrap();
                for (name, acc) in &platform.bank.accounts {
                    println!("{}: ${} | {}g gold", name, acc.balance, acc.gold_balance);
                }
            }

            "6" =>{
                print!("Sender Name: "); io::stdout().flush().unwrap();
                let mut from = String::new(); io::stdin().read_line(&mut from).unwrap();
                
                print!("Recipient Name: "); io::stdout().flush().unwrap();
                let mut to = String::new(); io::stdin().read_line(&mut to).unwrap();
                
                print!("Grams to transfer: "); io::stdout().flush().unwrap();
                let mut amt = String::new(); io::stdin().read_line(&mut amt).unwrap();

                if let Ok(amount) = Decimal::from_str(amt.trim()) {
                    let mut platform = shared_protocol.lock().unwrap();
                    match platform.transfer_tokens(from.trim(), to.trim(), amount) {
                        Ok(_) => println!("Successfully transferred {}g from {} to {}", amount, from.trim(), to.trim()),
                        Err(e) => println!("Transfer failed: {}", e),
                    }
                }
            }

            "7"=> {
                let platform=shared_protocol.lock().unwrap();
                println!("\n--- PROTOCOL AUDIT LOG ---");
                println!("{:<20} | {:<10} | {:<10} | {:<10} | {:<10}", "Timestamp", "User", "Action", "Amount", "Price");
                println!("{}", "-".repeat(70));
                for tx in &platform.history {
                // .format("%Y-%m-%d %H:%M:%S") makes it human-readable
                println!("{:<20} | {:<10} | {:<10} | {:<10} | ₹{:<10.2}", 
                    tx.timestamp.format("%Y-%m-%d %H:%M:%S"), 
                    tx.owner, 
                    tx.action, 
                    tx.amount, 
                    tx.price_at_time);
                }
                println!("{}", "_.".repeat(35));
            }
            
            "8" => {
                print!("Admin Name: "); io::stdout().flush().unwrap();
                let mut name = String::new(); io::stdin().read_line(&mut name).unwrap();
                
                print!("Grams of physical gold added to vault: "); io::stdout().flush().unwrap();
                let mut amt = String::new(); io::stdin().read_line(&mut amt).unwrap();

                if let Ok(amount) = Decimal::from_str(amt.trim()) {
                    let mut platform = shared_protocol.lock().unwrap();
                    match platform.admin_add_inventory(name.trim(), amount) {
                        Ok(_) => println!("Physical Audit Complete."),
                        Err(e) => println!("Access Denied: {}", e),
                    }
                }
            }
            "9" => {
                let platform = shared_protocol.lock().unwrap();
                match platform.generate_html_report() {
                    Ok(_) => println!("Report generated! Open 'audit_report.html' in your browser."),
                    Err(e) => println!("Failed to generate report: {}", e),
                }
            }
            "10" => {
                let mut platform = shared_protocol.lock().unwrap();
                save_bank_vault(&platform).expect("Failed to save!");
                println!("Vault locked. Goodbye!");
                break; // This exits the loop
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

// #[cfg(test)]
// mod tests{
//     use std::result;

//     use super::*;
//     use rust_decimal_macros::dec;

//     #[test]
//     fn test_gold_purchase_logic() {
//         let mut alchemsit = Account::new("Alchemist", dec!(0.0));

//         alchemsit.deposit(dec!(100.0));
//         let market= Market{ gold_price_per_gram: dec!(50.00)};

//         let result=alchemsit.buy_gold(dec!(1.0), &market);

//         assert!(result.is_ok());
//         assert_eq!(alchemsit.gold_balance, dec!(1.0));
//         assert_eq!(alchemsit.balance, dec!(50.00));
//     }
//     #[test]
//     fn test_insufficient_funds_for_gold() {
//         let mut bond = Account::new("Bond", dec!(0.0));
//         bond.deposit(dec!(10.00)); // Bob only has $10
//         let market = Market { gold_price_per_gram: dec!(50.00) };

//         // Bob tries to buy 1g ($50)
//         let result = bond.buy_gold(dec!(1.0), &market);

//         assert!(result.is_err());
//         assert_eq!(bond.gold_balance, dec!(0.0)); // Gold shouldn't have changed
//         assert_eq!(bond.balance, dec!(10.00));    // Money shouldn't have been taken
//     }

//     #[test]
//     fn test_transfer_logic() {
//         let mut alchemsit = Account::new("Alchemist", dec!(0.0));
//         let mut bond = Account::new("Bond", dec!(0.0));
//         alchemsit.deposit(dec!(100.00));
//         let result= transfer(&mut alchemsit, &mut bond, dec!(50.00));

//         assert!(result.is_ok());
//         assert_eq!(alchemsit.balance, dec!(50.00));
//         assert_eq!(bond.balance, dec!(50.00));
//     }
// }




