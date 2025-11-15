use std::collections::HashMap;
use std::io;

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

struct Bank{
    accounts: HashMap<u32, Account>
}

impl Bank{
    fn create_account(&mut self, account_no:u32, name: String){
        let user= Account{
            account_number: account_no,
            name: name.clone(),
            balance: 0.0
        };
        self.accounts.insert(account_no, user);

        println!("Account created successfully...");
        println!("Account number: {} and Customer Name: {}", account_no, name);
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
        if let Some(_acc)= self.accounts.get(&account_no){
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
}

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


fn main() {
    let mut bank= Bank{
        accounts: HashMap::new()
    };
    let mut account_serial= 1;
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
        let user_choice= match read_u32(user_choice){
            Some(val)=> val,
            None=> {
                println!("Wrong User Input !!");
                continue;
            }
        }
        

        match user_choice{
            0 => break,
            1 => {
            
            println!("Enter the user name: ");
            let mut user_name= String::new();
            if let Err(_)= io::stdin().read_line(&mut user_name){
                println!("Input taking error !!");
                continue;
            }
            let user_name= user_name.trim();
            bank.create_account(account_serial, user_name.to_string());
            account_serial+=1;
            },
            
            2 => {
                println!("Enter the user account number: ");
                let mut account_number= String::new();
                if let Err(_)= io::stdin().read_line(&mut account_number){
                    println!("Input taking error...");
                    continue;
                }
                let account_number: u32= match read_u32(account_number){
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
                let amount: f64= match read_f64(amount) {
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
                let account_number: u32= match read_u32(account_number){
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
                let account_number: u32= match read_u32(account_number){
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
                let amount: f64= match read_f64(amount){
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
                let account_number: u32= match read_u32(account_number){
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
        
    }

    println!("\nClosing Bank. See you soon...");
}
