/*
*    Topics: Ownership, Borrowing, Functions, Mutability
*    Description: Write a program where you define a function that takes a mutable reference to a String, appends some text,
*    and prints it. Demonstrate the difference between transferring ownership and borrowing in another function.
*
* 
*/

// borrowing example
fn borrowing_function(str_var:&mut String){
    let str1= " is a good boy !!";
    
    str_var.push_str(str1);

    /*
    * Here ownership is still within the str_var declared in the main function and we still want
    * the str_var to retain the ownership and we want to also make some changes to the string as
    * well and hence we take the string parameter as mutable reference (borrowing) and also notice
    * we are not returning anything !!
    */
    
}

// transferring ownership
fn transfer_ownership_function(str_var: String)-> String{
    let mut str_value= str_var;

    str_value.push_str("\nWhat a beautiful day it is !!");
    
    return str_value;

    /*
     Ownership is transferred in this function and thus no need to pass mut in parameter.
     It will now completely depend on the variable that will take the ownership of the parameter passed and also thus the type returned is also depends on what we want to pass out from the function    
    */
}

fn main() {
    let mut str_var= String::from("Dipankar");
    
    // case when borrowing the value   
    borrowing_function(&mut str_var);  
    println!("{}", str_var);
    
    println!("\n-- Another tutorial incoming !!\n");
    
    // case when ownership is transferred
    let transferred_str= transfer_ownership_function(str_var); 

    println!("{}", transferred_str);
}
