use std::{fs::*, io::{Write}};
use models::model as model;


mod render;
mod models;


fn insert_program(program_name: String, program_vendor:String) -> std::io::Result<()>{

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("timeclicker.config")
        .unwrap();

    write!(file,"\n{}: {}",program_name, program_vendor)
 
}

fn click_time() -> std::io::Result<Vec<model::Program>>{

    let programs = render::render::render_programs(&GetConfig().unwrap())?;
    Ok(programs)
}

fn add_program() -> std::io::Result<Vec<model::Program>> {

    println!("Enter Program Name:\n");

    let mut program_name = String::new();
    std::io::stdin().read_line(&mut program_name).expect("Failed to read line");
    println!("Enter Program Type:\n");

    let mut program_vendor = String::new();
    std::io::stdin().read_line(&mut program_vendor).expect("Failed to read line");

    program_vendor = str::replace(&program_vendor, "\n","");
    program_name = str::replace(&program_name, "\n", "");


    insert_program(program_name, program_vendor).expect("Unable to insert program");

    let programs_rendered = render::render::render_programs(&GetConfig().unwrap())?;
    Ok(programs_rendered)
}

fn update_program() -> std::io::Result<()> {
    println!("Update a program");
    Ok(())
}

fn remove_program() -> std::io::Result<()> {
    println!("Remove a program");
    Ok(())
}

fn cancel() -> std::io::Result<()> {
    println!("Cancel");
    Ok(())
}

fn return_invalid_input() -> std::io::Result<Vec<model::Program>> {
    println!("Invalid input");
    let empty_program = Vec::new();
    Ok(empty_program)
}


fn main() {




    println!("Welcome to timeclicker.\n
        0 - Click time
        1 - Add a program
        2 - Update a program
        3 - Remove a program
        4 - Cancel\n\n----------------------\nSelect an option then press enter" );

       let mut input = String::new();

       std::io::stdin().read_line(&mut input).expect("Failed to read line");
       match input.trim().parse::<i32>() {
           Ok(0) => click_time(),
           Ok(1) => add_program(),
           //Ok(2) => update_program(),
           //Ok(3) => remove_program(),
           //Ok(4) => cancel(),
           Err(e) => return_invalid_input(),
           Ok(_)=> return_invalid_input(),
       };

    //    println!("Select a program then press enter");
    //    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    //    match input.trim().parse::<i32>() {

    //    }
}

fn GetConfig() -> std::io::Result<String> {
    println!("Initializing");

    let time_clicker_config_file: String;

    match std::env::var("TIME_CLICKER_CONFIG_FILE"){
        Ok(val) => time_clicker_config_file = val,
        Err(_) => time_clicker_config_file = "timeclicker.config".to_string(),
    }

    Ok(time_clicker_config_file)
}
