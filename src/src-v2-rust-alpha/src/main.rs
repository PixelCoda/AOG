pub mod setup;
pub mod aog;


use std::error::Error;
use std::thread;
use std::time::Duration;

use std::io::{stdin,stdout,Write};

use std::path::{Path};

use rppal::gpio::Gpio;

use serde::{Serialize, Deserialize};
use shuteye::sleep;

use std::env;


extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;


const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


fn main() {

    aog::cls();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
       
    } else {
        // Running on screen
        aog::print_stats();


        if !Path::new("/opt/aog/").exists() {
            setup::install();
        }
    
    
        // Does config file exist and is it valid?
        // Config can become invalid with software updates
        if Path::new("/opt/aog/").exists() {
            let aog_config = load_file("/opt/aog/config.bin", 0);
    
            if aog_config.is_ok() {
                let cfg: aog::Config = aog_config.unwrap();
                if cfg.version_installed != VERSION.unwrap_or("unknown").to_string(){
                    println!("An old A.O.G. install was detected.");
                    setup::update();
                }
            } else {
                println!("A.O.G. config is corrupt....");
                println!("Deleting config and re-initializing setup...");
                setup::uninstall();
                setup::install();
            }
        }
    
    }

 



    // UV-Light thread
    thread::spawn(|| {
        loop{
            // let aog_config = load_file("/opt/aog/config.bin", 0);
            // if aog_config.is_ok() {
    
            // }
        }

    });
    
    // Secondary-Tank Water Pump Thread
    thread::spawn(|| {
        loop{
            let raw = aog::sensors::get_arduino_raw();

            if raw.contains("TOP_TANK_OVERFLOW: NONE"){
                aog::command::run("gpio on 17".to_string());
            } else {
                aog::command::run("gpio off 17".to_string());
            }
 
        }

    });

    // Retrieve the GPIO pin and configure it as an output.
    // let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    loop {
        // pin.toggle();
        // thread::sleep(Duration::from_millis(500));

        let mut s=String::new();
        print!("> ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        aog::command::run(s);
        

        // if s.contains("Y") || s.contains("y") {
        //     aog_config.power_type = "solar";
        // } else {
        //     aog_config.power_type = "grid";
        // }


    }




}