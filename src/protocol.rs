use std::vec;

use crate::models::{Account, Bank, Transaction, UserRole}; // Import from your other file
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct  RwaProtocol {
    pub bank: Bank,
    pub total_token_supply: Decimal,
    pub physical_vault_grams: Decimal,
    pub gold_price_inr: Decimal,
    pub history: Vec<Transaction>,
}
impl RwaProtocol{
    pub fn new()-> Self {
        Self { 
            bank: Bank::new(),
            total_token_supply: dec!(0.0), 
            physical_vault_grams: dec!(1000.00), 
            gold_price_inr: dec!(6256.00), 
            history: Vec::new()
        }
    }

    pub fn admin_add_inventory(&mut self, admin_name: &str, grams: Decimal) -> Result<() , String> {
        let admin= self.bank.accounts.get(admin_name)
            .ok_or("Admin account not found")?;

        if admin.role != UserRole::Admin {
            return Err("Access Denied: You are not authorised".to_string());
        }

        self.physical_vault_grams+=grams;

        self.history.push(Transaction { timestamp: Utc::now(), 
                                        owner: admin_name.to_string(), 
                                        action: "VAULT_REPLENISH".to_string(), 
                                        amount: grams, 
                                        price_at_time: self.gold_price_inr });

        Ok(())
    }

    pub fn perform_audit(&self) {
        let mut total_gold_in_accounts = dec!(0.0);
        
        // Sum up every user thinks they own
        for account in self.bank.accounts.values() {
            total_gold_in_accounts += account.gold_balance;
        }

        println!("\n--- PROTOCOL AUDIT REPORT ---");
        println!("Digital Supply (Ledger):  {}g", self.total_token_supply);
        println!("User Balances (Sum):      {}g", total_gold_in_accounts);
        println!("Physical Vault (Real):    {}g", self.physical_vault_grams);
        
        // Check 1: Do the accounts match the supply?
        if total_gold_in_accounts == self.total_token_supply {
            println!("INTERNAL LEDGER: Consistent.");
        } else {
            println!("INTERNAL LEDGER: Mismatch detected!");
        }

        // Check 2: Is the gold actually in the vault?
        if self.physical_vault_grams >= self.total_token_supply {
            println!("RESERVE STATUS: Fully Collateralized.");
        } else {
            println!("RESERVE STATUS: UNDER-COLLATERALIZED! (Dangerous)");
        }
    }
    
    pub fn buy_tokens(&mut self, user_name: &str, grams: Decimal) -> Result<(), String> {
        
        if self.total_token_supply + grams > self.physical_vault_grams {
            return Err("PROTOCOL ERROR: Not enough physical gold in vault!".to_string());
        }

        let account = self.bank.accounts.get_mut(user_name)
            .ok_or("Account not found")?;

        let cost = grams * self.gold_price_inr;
        let fee=cost*dec!(0.001); //simulating fee
        let tot_cost=cost+fee;
        if account.balance < tot_cost {
            return Err(format!("Insufficient INR. Need: {}, Have: {}", tot_cost, account.balance));
        }

        account.balance -= tot_cost;           
        account.gold_balance += grams;    
        self.total_token_supply += grams; 

        self.history.push(Transaction { timestamp: Utc::now(), 
                                        owner: user_name.to_string(), 
                                        action: "BUY".to_string(), 
                                        amount: grams, 
                                        price_at_time: self.gold_price_inr });

        println!("TRADE SUCCESS: {}g minted for {} at ₹{}", grams, user_name, self.gold_price_inr);
        Ok(())
    
    }

    pub fn sell_tokens(&mut self, user_name: &str, grams: Decimal) -> Result<(), String> {

        let account = self.bank.accounts.get_mut(user_name)
            .ok_or("Account not found")?;

        if account.gold_balance < grams {
            return Err(format!("Insufficient Gold Tokens. Have: {}g, Trying to sell: {}g", account.gold_balance, grams));
        }

        let payout = grams * self.gold_price_inr;

        account.gold_balance -= grams;    
        account.balance += payout;        
        self.total_token_supply -= grams; 
        
        self.history.push(Transaction { timestamp: Utc::now(), 
                                        owner: user_name.to_string(), 
                                        action: "SELL".to_string(), 
                                        amount: grams, 
                                        price_at_time: self.gold_price_inr });

        println!("LIQUIDATION SUCCESS: {}g burned for {} at ₹{}", grams, user_name, self.gold_price_inr);
        Ok(())
    }

    pub fn transfer_tokens(&mut self, from_name: &str, to_name: &str, grams: Decimal) -> Result<(), String> {
        
        if !self.bank.accounts.contains_key(from_name) { return Err("Sender not found".into()); }
        if !self.bank.accounts.contains_key(to_name) { return Err("Recipient not found".into()); }

        {
            let sender = self.bank.accounts.get_mut(from_name).unwrap();
            if sender.gold_balance < grams {
                return Err("Insufficient gold balance for transfer".into());
            }
            sender.gold_balance -= grams;
        }

        let recipient = self.bank.accounts.get_mut(to_name).unwrap();
        recipient.gold_balance += grams;

        self.history.push(Transaction {
            timestamp: Utc::now(),
            owner: from_name.to_string(),
            action: format!("TRANSFER TO {}", to_name),
            amount: grams,
            price_at_time: self.gold_price_inr,
        });

        Ok(())
    }

    pub fn generate_html_report(&self) -> std::io::Result<()> {
        
        let mut html = String::new();

        html.push_str("
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 40px; background-color: #f4f7f6; }
                h1 { color: #2c3e50; border-bottom: 2px solid #3498db; padding-bottom: 10px; }
                table { width: 100%; border-collapse: collapse; background: white; box-shadow: 0 1px 3px rgba(0,0,0,0.2); }
                th { background-color: #3498db; color: white; padding: 12px; text-align: left; }
                td { padding: 12px; border-bottom: 1px solid #ddd; }
                tr:hover { background-color: #f1f1f1; }
                .price { color: #27ae60; font-weight: bold; }
            </style>
        </head>
        <body>
        ");

        html.push_str(&format!("<h1>RWA Protocol Audit: {}</h1>", chrono::Utc::now().format("%Y-%m-%d")));
        html.push_str(&format!("<p>Current Gold Price: <span class='price'>₹{:.2}</span></p>", self.gold_price_inr));
        html.push_str(&format!("<p>Total Token Supply: {}g | Physical Vault: {}g</p>", self.total_token_supply, self.physical_vault_grams));

        html.push_str("<table>
            <tr>
                <th>Timestamp (UTC)</th>
                <th>Account</th>
                <th>Action</th>
                <th>Amount (g)</th>
                <th>Market Price</th>
            </tr>");

        for tx in &self.history {
            html.push_str(&format!("
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}g</td>
                <td>₹{:.2}</td>
            </tr>", 
            tx.timestamp.format("%H:%M:%S"), tx.owner, tx.action, tx.amount, tx.price_at_time));
        }

        html.push_str("</table></body></html>");

        std::fs::write("audit_report.html", html)?;
        Ok(())
    }
}