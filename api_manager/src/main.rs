use std::io::{self, Write};
use api_manager::objects::api::Api;

fn main(){
    println!("You can add new api!");
    print!("Please input new api url:");
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();

    print!("Please input new api method:");
    io::stdout().flush().unwrap();

    let mut method = String::new();
    io::stdin().read_line(&mut method).unwrap();

    let mut parameters:Vec<String> = Vec::new();

    loop {
        print!("Please input parameter(if you finished registering parameter please Enter)");
        io::stdout().flush().unwrap();
        let mut parameter = String::new();
        io::stdin().read_line(&mut parameter).unwrap();
        if parameter.trim()=="" {
            break
        }
        parameters.push(parameter);
    }

    println!("Assert!");
    println!("url:{}",url);
    println!("method:{}",method);
    println!("parameters:{:?}",parameters);
    println!("Is sure to register api Y/n:");
    io::stdout().flush().unwrap();
    let mut is_asserting = String::new();
    io::stdin().read_line(&mut is_asserting).unwrap();
    if is_asserting.trim().to_lowercase()=="y" {
        let api = Api::new(url,method,parameters);
        println!("{:?}",api);
        return;

    }
    println!("You don\'t register any api.")



}