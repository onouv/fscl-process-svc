
mod core;
use crate::core::{ Function, Sub };

static FUNCTION_ERROR: i32 = -1;

fn main() {
    println!("FSCL process service starting up.");

    let mut func = match Function::new("=100", "Protect PAX", "none") {
        Ok(f) => f,
        Err(_) => {
            println!("Failed to create function");
            std::process::exit(FUNCTION_ERROR);
        }
    };

    let sub_func = match Function::new("=101", "Sub Function", "none") {
        Ok(f) => f,
        Err(_) => {
            println!("Failed to create sub function");
            std::process::exit(FUNCTION_ERROR);
        }
    };

    match func.add_sub(sub_func) {
        Ok(_) => println!("Sub function added successfully"),
        Err(e) => {
            println!("Failed to add sub function: {:?}", e);
            std::process::exit(FUNCTION_ERROR);
        }
    };


    
}
