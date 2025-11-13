use std::collections::HashMap;
use std::io;

struct Account{
    account_number: u32,
    name: String,
    balance: f64
}

impl Account{
    fn withdraw(&mut self, amount:f64){
        self.balance-= amount;
        
    }

    fn deposit(&mut self, amount:f64){
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

        println!("Account created successfull...");
        println!("Account number: {} and Customer Name: {}", account_no, name);
    }

    
    fn withdraw_money(&mut self, account_no: u32, amount: f64){
        if let Some(acc)= self.accounts.get_mut(&account_no){
            if amount<=acc.balance() {
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

        for (account_no, acc) in &self.accounts{
            println!("Account No.: {}, Name: {}, Balance: {}", account_no, acc.name, acc.balance);
        }
    }
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
        io::stdin().read_line(&mut user_choice).expect("Faile to read line");
        let user_choice= user_choice.trim();
        

        match user_choice{
            "0" => break,
            "1" => {
            
            println!("Enter the user name: ");
            let mut user_name= String::new();
            io::stdin().read_line(&mut user_name).expect("Failed to read line");
            let user_name= user_name.trim();
            bank.create_account(account_serial, user_name.to_string());
            account_serial+=1;
            },
            
            "2"=> {
                println!("Enter the user account number: ");
                let mut account_number= String::new();
                io::stdin().read_line(&mut account_number).expect("Failed to read line");
                let account_number: u32= account_number.trim().parse().expect("Failed to parse the account_number");
                
                println!("Enter the amount to withdraw: ");
                let mut amount= String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64= amount.trim().parse().expect("Failed to parse the amount");

                bank.withdraw_money(account_number, amount);
            },

            "3" => {
                println!("Enter the account number: ");
                let mut account_number= String::new();
                io::stdin().read_line(&mut account_number).expect("Failed to read line");
                let account_number: u32= account_number.trim().parse().expect("Failed to parse the account_number");

                bank.check_balance(account_number);
            },
            
            "4" => {
                println!("Enter the account number: ");
                let mut account_number= String::new();
                io::stdin().read_line(&mut account_number).expect("Failed to read line");
                let account_number: u32= account_number.trim().parse().expect("Failed to parse the account_number");
                
                println!("Enter the amount: ");
                let mut amount= String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64= amount.trim().parse().expect("Failed to parse the amount");

                bank.add_money(account_number, amount);
            }
            
            "5" => {
                println!("Enter the account number: ");
                let mut account_number= String::new();
                io::stdin().read_line(&mut account_number).expect("Failed to read line");
                let account_number: u32= account_number.trim().parse().expect("Failed to parse the account_number");
                
                bank.delete_account(account_number);
            }

            "6" => {
                println!("Displaying all the bank accounts !!");
                bank.display_all_accounts();
            }

            _ => println!("Not a valid operation !!"),
        };
        
    }

    println!("\nClosing Bank. See you soon...");
}
