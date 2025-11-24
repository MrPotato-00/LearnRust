use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Account{
    account_number: u32,
    name: String,
    balance: f64
}

impl Account{
    fn withdraw(&mut self, amount:f64){
        if amount<=0.0{
            return;
        }
        self.balance-= amount;
        
    }

    fn deposit(&mut self, amount:f64){
        if amount<0.0{
            return;
        }
        self.balance+=amount;
    }
    fn balance(&self)->f64{
        return self.balance;
    }
}


#[derive(Serialize, Deserialize)]
struct Bank{
    accounts: HashMap<u32, Account>,
    next_account_number: u32
}

impl Bank{
    fn new()->Self{
        Bank{
            accounts: HashMap::new(),
            next_account_number: 1,
        }
    }

    fn create_account(&mut self, name: String){
        let account_number= self.next_account_number;
        let user= Account{
            account_number: account_number,
            name: name.clone(),
            balance: 0.0
        };
        self.accounts.insert(account_number, user);
        self.next_account_number+=1;
        println!("Account created successfully...");
        println!("Account number: {} and Customer Name: {}",account_number,  name);
    }

    
    fn withdraw_money(&mut self, account_no: u32, amount: f64){
        if let Some(acc)= self.accounts.get_mut(&account_no){
            if acc.balance()-amount>=0.0 {
                acc.withdraw(amount);
            
                println!("Successfully withdrawed money: {}", amount);
            }
            else{
                println!("Not enough balance available");
            }
        }
        else{
            println!("Account does not exist !!");
        }
    }
    
    fn delete_account(&mut self, account_no: u32){
        if self.accounts.contains_key(&account_no){
            self.accounts.remove(&account_no);
            println!("Removed the account number: {}", account_no);
        }
        else{
            println!("Account number {} not present", account_no);
        }
    }

    fn check_balance(&self, account_no: u32){
        if let Some(acc)= self.accounts.get(&account_no){
            println!("The current balance in account number: {} is {}", account_no, acc.balance());
        }
        else{
            println!("The account number: {} not present", account_no);
        }
    }

    fn add_money(&mut self, account_no: u32, amount: f64){
        if let Some(acc)= self.accounts.get_mut(&account_no){
            acc.deposit(amount);
            println!("Amount {} successfully credited to the account {}", amount, account_no);
        }
        else{
            println!("Account does not exist !!");
        }
    }

    fn display_all_accounts(&self){
        if self.accounts.is_empty(){
            println!("No accounts in the bank !!");
        }
        else{
            for (account_no, acc) in &self.accounts{
                println!("Account No.: {}, Name: {}, Balance: {}", account_no, acc.name, acc.balance);
            }
        }

        
    }

    fn save_to_file(&self, filename: &str)->io::Result<()>{
        let serialized= serde_json::to_string_pretty(self)?;
        let mut file= File::create(filename)?;

        file.write_all(serialized.as_bytes())?;
        println!("Data saved to {}", filename);
        return Ok(());
    }

    fn load_from_file(filename: &str)->io::Result<Self>{
        if !Path::new(filename).exists(){
            println!("No existing data file found. Starting with fresh bank");
            return Ok(Bank::new());
        }

        let mut file= File::open(filename);
        let mut contents= String::new();
        file?.read_to_string(&mut contents)?;

        let bank: Bank= serde_json::from_str(&contents)?;
        println!("Data loaded from {}", filename);
        return Ok(bank);
    }
}
// apply generic here
fn read_value<T: std::str::FromStr>(user_input: String)->Option<T>{
    return match user_input.trim().parse::<T>(){
        Ok(val)=> Some(val),
        Err(_)=> None
    };
}

/*
fn read_u32(user_input: String)-> Option<u32>{

    return match user_input.trim().parse::<u32>(){
        Ok(val)=> Some(val),
        Err(_)=> None
    };
    
}

fn read_f64(user_input: String)-> Option<f64>{
    return match user_input.trim().parse::<f64>(){
        Ok(val)=> Some(val),
        Err(_)=> None
    };
}
*/

fn main() {
    
    const DATA_FILE: &str= "bank_data.json";

    let mut bank= match Bank::load_from_file(DATA_FILE){
        Ok(bank) => bank,
        Err(e) => {
            println!("Error loading data: {}. Starting with fresh bank.", e);
            Bank::new()
        }
    };

    loop{
        println!("\n=========== Welcome to the Bank System !! ============\n");
        
        println!("Press 0: Stop Banking...");
        println!("Press 1: Create Account");
        println!("Press 2: Withdraw Money");
        println!("Press 3: Check Balance");
        println!("Press 4: Add Money");
        println!("Press 5: Delete Account");
        println!("Press 6: Show all accounts");

        let mut user_choice= String::new();
        if let Err(_)= io::stdin().read_line(&mut user_choice){
            println!("Input taking error !!");
            continue;
        }
        let user_choice:u32= match read_value::<u32>(user_choice){
            Some(val)=> val,
            None=> {
                println!("Wrong User Input !!");
                continue;
            }
        };
        

        match user_choice{
            0 => break,
            1 => {
            
                println!("Enter the user name: ");
                let mut user_name= String::new();
                if let Err(_)= io::stdin().read_line(&mut user_name){
                    println!("Input taking error !!");
                    continue;
                }
                let user_name= user_name.trim().to_string();
                bank.create_account(user_name);
                
            },
            
            2 => {
                println!("Enter the user account number: ");
                let mut account_number= String::new();
                if let Err(_)= io::stdin().read_line(&mut account_number){
                    println!("Input taking error...");
                    continue;
                }
                let account_number: u32= match read_value::<u32>(account_number){
                    Some(val)=> val, 
                    None => {
                        println!("Wrong User Input !!");
                        continue;
                    }
                };
                
                println!("Enter the amount to withdraw: ");
                let mut amount= String::new();
                if let Err(_)= io::stdin().read_line(&mut amount){
                    println!("Wrong User Input...");
                    continue;
                }
                let amount: f64= match read_value::<f64>(amount) {
                    Some(val) => val,
                    None=> {
                        println!("Wrong User Input !!");
                        continue;
                    }
                };

                if amount>=0.0 {
                    bank.withdraw_money(account_number, amount);
                }
                else {
                    println!("Invalid amount entered. Please check !!");
                }
                
            },

            3 => {
                println!("Enter the account number: ");
                let mut account_number= String::new();
                if let Err(_)= io::stdin().read_line(&mut account_number){
                    println!("Input taking error...");
                    continue;
                }
                let account_number: u32= match read_value::<u32>(account_number){
                    Some(val)=> val,
                    None=> {
                        println!("Wrong User Input !!");
                        continue;
                    }
                };

                bank.check_balance(account_number);
            },
            
            4 => {
                println!("Enter the account number: ");
                let mut account_number= String::new();
                if let Err(_)= io::stdin().read_line(&mut account_number){
                    println!("Input taking error...");
                    continue;

                }
                let account_number: u32= match read_value::<u32>(account_number){
                    Some(val)=> val,
                    None=> {
                        println!("Wrong User Input !!");
                        continue;
                    }
                };
                
                println!("Enter the amount: ");
                let mut amount= String::new();
                if let Err(_)= io::stdin().read_line(&mut amount){
                    println!("Wrong User Input...");
                    continue;
                }
                let amount: f64= match read_value::<f64>(amount){
                    Some(val)=> val,
                    None=> {
                        println!("Wrong User Input !!");
                        continue;
                    }
                };

                if amount<0.0 {
                    println!("Amount is not valid !!");
                }
                else{
                    bank.add_money(account_number, amount);
                }
            }
            
            5 => {
                println!("Enter the account number: ");
                let mut account_number= String::new();
                if let Err(_)= io::stdin().read_line(&mut account_number){
                    println!("Input taking error...");
                    continue;
                }
                let account_number: u32= match read_value::<u32>(account_number){
                    Some(val)=> val,
                    None => {
                        println!("Wrong User Input !!");
                        continue;
                    }
                };
                
                bank.delete_account(account_number);
            }

            6 => {
                println!("Displaying all the bank accounts !!");
                bank.display_all_accounts();
            }

            _ => println!("Not a valid operation !!"),
        };

        if let Err(e)= bank.save_to_file(DATA_FILE){
            println!("Warning: Could not save data: {}", e);
        }
        
    }

    if let Err(e)= bank.save_to_file(DATA_FILE){
        println!("Warning: Could not save data: {}", e);
    }

    println!("\nClosing Bank. See you soon...");
}
