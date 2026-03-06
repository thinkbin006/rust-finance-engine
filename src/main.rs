use std::os::windows::io::IntoRawHandle;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// fn main(){
//     let principals = vec![dec!(1000.00), dec!(150000), dec!(16542)];
//     let rates= vec![dec!(0.04), dec!(0.05), dec!(0.045)];
//     let years= 10;
//     let period=12;

//     let n=Decimal::from(period);
//     let t= Decimal::from(years);
//     println!("--- Financial Report ---");
//     for i in 0..rates.len() {
//         let prin=principals[i];
//         let gr=dec!(1)+(rates[i]/n);
//         let tot_period=period*years;
        
//         let fin=prin*gr.powi(tot_period as i64);
//         let interest=fin-prin;

//         println!("Initial Investment: ${}", prin);
//         println!("After {} years at {}% interest:", years, rates[i] * dec!(100));
//         println!("Final Balance: ${:.2}", fin); 
//         println!("Interest Earned: {}", interest);
//     }
    
    
    
    
// }


struct Account {
    owner: String,
    balance: Decimal,
    interest_rate: Decimal,
}

impl Account {
    // 1. A "Constructor" to create a new account easily
    fn new(name: &str, rate: Decimal) -> Self {
        Self {
            owner: name.to_string(),
            balance: dec!(0.0), // Starts at zero
            interest_rate: rate,
        }
    }

    // 2. A "Method" to deposit money
    // '&mut self' means "I need permission to CHANGE this account"
    fn deposit(&mut self, amount: Decimal) {
        self.balance += amount;
        println!("Success: ${} deposited to {}'s account.", amount, self.owner);
    }

    // 3. A "Method" to apply interest
    fn apply_interest(&mut self) {
        let interest = self.balance * self.interest_rate;
        self.balance += interest;
    }

    fn withdraw(&mut self, amount: Decimal){
        if amount>self.balance {
            println!("Insufficient funds\n Current balane: {}", self.balance);
        }else{
            self.balance-=amount;
            println!("Withdraw successful! \nCurrent balance: {}", self.balance);
        }
    }
}

fn main() {
    // Create Alice's account with 5% interest
    let mut alice_acct = Account::new("Alice", dec!(0.05));

    // Let's do some banking!
    alice_acct.deposit(dec!(1000.00));
    alice_acct.apply_interest();

    println!("--- Statement ---");
    println!("Owner: {}", alice_acct.owner);
    println!("New Balance after interest: ${:.2}", alice_acct.balance);
    alice_acct.withdraw(dec!(500.00));
}



