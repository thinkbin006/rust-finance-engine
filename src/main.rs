use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    owner: String,
    balance: Decimal,       
    gold_balance: Decimal,  
    interest_rate: Decimal,
}
struct Market {
    gold_price_per_gram: Decimal,
}
#[derive(Serialize, Deserialize, Debug)]
struct Bank {
    accounts: HashMap<String, Account>,
}

impl Bank{
    fn new() -> Self{
        Self { accounts : HashMap::new() }
    }
    fn add_account(&mut self, name: &str, rate: Decimal){
        let name_str=name.to_string();
        let new_account= Account::new(&name_str, rate);
        self.accounts.insert(name_str, new_account);
    }
}

impl Market{
    fn update_price(&mut self, new_price: Decimal){
        self.gold_price_per_gram=new_price;
        println!("Attention! Updated gold price: {}", self.gold_price_per_gram);
    }
}

impl Account {

    fn new(name: &str, rate: Decimal) -> Self {
        Self {
            owner: name.to_string(),
            balance: dec!(0.0),
            gold_balance: dec!(0.0),
            interest_rate: rate,
        }
    }

    fn deposit(&mut self, amount: Decimal) {
        self.balance += amount;
        println!("Success: ${} deposited to {}'s account.", amount, self.owner);
    }

    fn apply_interest(&mut self) {
        let interest = self.balance * self.interest_rate;
        self.balance += interest;
    }

    fn withdraw(&mut self, amount: Decimal)->Result<(), String>{
        if amount <= dec!(0) {
            return Err(String::from("Cannot withdraw zero or negative amounts"));
        }
        
        if self.balance >= amount {
            self.balance -= amount;
            Ok(()) // Return "Success"
        } else {
            Err(String::from("Insufficient funds for this transaction"))
        }
    }

    

    fn buy_gold(&mut self, grams: Decimal, market: &Market)-> Result<(), String>{
        let tot_cost=grams*market.gold_price_per_gram;
        if self.balance>=tot_cost{
            self.balance-=tot_cost;
            self.gold_balance=grams;
            println!("Purchase for {} grams successful!", grams);
            println!("Remaining balance: {}", self.balance);
            Ok(())
        }else{
            Err(format!("Insufficient funds! \nYou need ${} and you have ${}",tot_cost,self.balance))
        }

    }

    fn sell_gold(&mut self,grams: Decimal, market: &Market)-> Result<(), String>{
        let tot_cost=grams*market.gold_price_per_gram;
        if grams<=self.gold_balance {
            self.balance+=tot_cost;
            self.gold_balance-=grams;
            println!("{} grams gold sold!", grams);
            println!("Remaining gold quantitiy: {} grams", self.gold_balance);
            Ok(())
        }else{
            Err(format!("Insufficient amount, current gold balance {}", self.balance))
        }
    }
}
fn transfer(from: &mut Account, to: &mut Account, amount: Decimal) -> Result<(), String> {
        //withdraw with err check
        from.withdraw(amount)?;

        //if withdrawal worked,n ow deposit.
        to.deposit(amount);

        Ok(())
}

fn save_bank_vault(bank: &Bank) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(bank).unwrap();
    let mut file = File::create("bank_vault.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_bank_vault() -> Option<Bank> {
    let file = File::open("bank_vault.json").ok()?;
    let bank: Bank = serde_json::from_reader(file).ok()?;
    Some(bank)
}

fn main() {
    println!("Rust finance");
    let mut bank: Bank= load_bank_vault().unwrap_or_else(|| Bank::new());
    let market :Market=Market { gold_price_per_gram: dec!(1000.0) };
    loop {
        print!("\n(1)-> Create Account (2)-> Deposit (3)-> Buy Gold (5)-> Balance (6)-> Save & Exit\n");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");

        match choice.trim(){
            "1" => {
                print!(" Enter name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name= name.trim();
                bank.add_account(name, dec!(0.05));
                println!("Account created for {}", name.trim());

            }
            "2" => {
                print!("Enter Account name: ");
                io::stdout().flush().unwrap();
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                if let Some(account) = bank.accounts.get_mut(name) {
                    print!("Amount to deposit: ");
                    io::stdout().flush().unwrap();
                    let mut amt = String::new();
                    io::stdin().read_line(&mut amt).unwrap();
                    
                    // Convert string to Decimal
                    if let Ok(amount) = Decimal::from_str(amt.trim()) {
                        account.deposit(amount);
                        println!("Deposited! New balance: ${}", account.balance);
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

                if let Some(account) = bank.accounts.get_mut(name) {
                    print!("Amount to buy(grams): ");
                    io::stdout().flush().unwrap();
                    let mut amt = String::new();
                    io::stdin().read_line(&mut amt).unwrap();
                    
                    if let Ok(amount) = Decimal::from_str(amt.trim()) {
                        match account.buy_gold(amount, &market) {
                            Ok(_) => println!("Trade Successful!"),
                            Err(e) => println!("Trade failed: {}", e),
                        }
                    }
                } else{
                    println!("Account not found!");
                }
            }
            "4" =>{
                print!("Enter Account Name: ");
                io::stdout().flush().unwrap();
                let mut name= String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                if let Some(account) = bank.accounts.get_mut(name) {
                    print!("Amount to sell(grams): ");
                    io::stdout().flush().unwrap();
                    let mut amt = String::new();
                    io::stdin().read_line(&mut amt).unwrap();
                    
                    if let Ok(amount) = Decimal::from_str(amt.trim()) {
                        match account.sell_gold(amount, &market) {
                            Ok(_) => println!("Trade successful!"),
                            Err(e) => println!("Trade Failed: {}",e),
                        }
                    }
                } else{
                    println!("Account not found!");
                }
            }

            "5" => {
                for (name, acc) in &bank.accounts {
                    println!("{}: ${} | {}g gold", name, acc.balance, acc.gold_balance);
                }
            }
            "6" => {
                save_bank_vault(&bank).expect("Failed to save!");
                println!("Vault locked. Goodbye!");
                break; // This exits the loop
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

#[cfg(test)]
mod tests{
    use std::result;

    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_gold_purchase_logic() {
        let mut alchemsit = Account::new("Alchemist", dec!(0.0));

        alchemsit.deposit(dec!(100.0));
        let market= Market{ gold_price_per_gram: dec!(50.00)};

        let result=alchemsit.buy_gold(dec!(1.0), &market);

        assert!(result.is_ok());
        assert_eq!(alchemsit.gold_balance, dec!(1.0));
        assert_eq!(alchemsit.balance, dec!(50.00));
    }
    #[test]
    fn test_insufficient_funds_for_gold() {
        let mut bond = Account::new("Bond", dec!(0.0));
        bond.deposit(dec!(10.00)); // Bob only has $10
        let market = Market { gold_price_per_gram: dec!(50.00) };

        // Bob tries to buy 1g ($50)
        let result = bond.buy_gold(dec!(1.0), &market);

        assert!(result.is_err());
        assert_eq!(bond.gold_balance, dec!(0.0)); // Gold shouldn't have changed
        assert_eq!(bond.balance, dec!(10.00));    // Money shouldn't have been taken
    }

    #[test]
    fn test_transfer_logic() {
        let mut alchemsit = Account::new("Alchemist", dec!(0.0));
        let mut bond = Account::new("Bond", dec!(0.0));
        alchemsit.deposit(dec!(100.00));
        let result= transfer(&mut alchemsit, &mut bond, dec!(50.00));

        assert!(result.is_ok());
        assert_eq!(alchemsit.balance, dec!(50.00));
        assert_eq!(bond.balance, dec!(50.00));
    }
}




