use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UserRole{
    Admin,
    Customer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction{
    pub timestamp: DateTime<Utc>,
    pub owner: String,
    pub action: String,
    pub amount: Decimal,
    pub price_at_time: Decimal, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub owner: String,
    pub role: UserRole,
    pub balance: Decimal,
    pub gold_balance: Decimal,
    pub interest_rate: Decimal,
}

impl Account {

    pub fn new(name: &str, rate: Decimal, role: UserRole) -> Self {
        Self {
            owner: name.to_string(),
            role: role,
            balance: dec!(0.0),
            gold_balance: dec!(0.0),
            interest_rate: rate,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.balance += amount;
        println!("Success: ${} deposited to {}'s account.", amount, self.owner);
    }

    pub fn apply_interest(&mut self) {
        let interest = self.balance * self.interest_rate;
        self.balance += interest;
    }

    pub fn withdraw(&mut self, amount: Decimal)->Result<(), String>{
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

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank {
    pub accounts: HashMap<String, Account>,
}

impl Bank{
    pub fn new() -> Self{
        Self { accounts : HashMap::new() }
    }
    pub fn add_account(&mut self, name: &str, rate: Decimal, role: UserRole){
        let name_str=name.to_string();
        let new_account= Account::new(&name_str, rate, role);
        self.accounts.insert(name_str, new_account);
    }
}

pub struct Market {
    pub gold_price_per_gram: Decimal,
}
impl Market{
    pub fn update_price(&mut self, new_price: Decimal){
        self.gold_price_per_gram=new_price;
        println!("Attention! Updated gold price: {}", self.gold_price_per_gram);
    }
}

