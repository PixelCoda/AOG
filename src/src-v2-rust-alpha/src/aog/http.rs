// Copyright (c) 2020-2021 Caleb Mitchell Smith (PixelCoda)
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.



use std::path::{Path};


use rouille::Response;
use rouille::post_input;
use rouille::session;
use rouille::try_or_400;

use std::sync::Mutex;



use std::sync::Arc;






use serde::{Serialize, Deserialize};


use crate::aog;

use ::aog::Config;



// Add Debug Flag and use ./www/ instead of installed dir

pub fn init(){


    let config = Arc::new(Mutex::new(Config::load(0).unwrap()));
    let cert = std::fs::read("/opt/aog/crt/default/aog.local.cert").unwrap();
    let pkey = std::fs::read("/opt/aog/crt/default/aog.local.key").unwrap();
    
    rouille::Server::new_ssl("0.0.0.0:8443", move |request| {
        {
            session::session(request, "SID", 3600, |session| {
                let session_id: &str = session.id();
                let mut session_authenticated = false;
                let mut sessions :Vec<::aog::Session> = Vec::new();
    
    
                if Path::new("/opt/aog/dat/sessions.bin").exists() {
                    sessions = ::aog::Sessions::load(0).unwrap().sessions;
                }
    
                for session in &sessions{
                    if session.id.contains(session_id){
                        session_authenticated = true;
                    } 
                }
    
 
    
                let edit_aog_config = &mut *config.lock().unwrap();
    
    
    
                if request.url() == "/authenticate"{
                
                    let input = try_or_400!(post_input!(request, {
                        input_username: String,
                        input_password: String,
                    }));
                    if input.input_username == *"admin" && input.input_password == edit_aog_config.encrypted_password {
                                            let session = ::aog::Session {
                                                id: session_id.to_string(),
                                                delta: 0
                                            };
                                            sessions.push(session);
                    
                                            let _session_save_file = ::aog::Sessions {
                                                sessions: sessions.clone()
                                            };
                    
                                            // save_file("/opt/aog/dat/sessions.bin", 0, &session_save_file).unwrap();
                                            
                                        }
                  
                    let response = Response::redirect_302("/index.html");
                    return response;
    
    
                }
    

                
    
                if request.url().contains("/api/dat/"){


                    if let Some(request) = request.remove_prefix("/api/dat/") {
                        return rouille::match_assets(&request, "/opt/aog/dat").with_additional_header("Access-Control-Allow-Origin", "*").with_no_cache();
                    } else {
                        return Response::text("err".to_string())
                            .with_additional_header("Access-Control-Allow-Origin", "*");
                    }


                }
    
                if request.url() == "/api/stats"{
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    struct WebApiStats {
                        pm25: String,
                        pm10: String,
                        co2: String,
                        tvoc: String,
                        temp: String,
                        hum: String
                    }
                   
                    let response = Response::json(&WebApiStats { co2: crate::aog::sensors::get_value("co2"), tvoc: crate::aog::sensors::get_value("tvoc"), temp: crate::aog::sensors::get_value("temp"), hum: crate::aog::sensors::get_value("hum"), pm25: crate::aog::sensors::get_value("pm25"), pm10: crate::aog::sensors::get_value("pm10") });
                    return response;
                }
    
    
                // catchall regardless of auth status
                if request.url() == "/login.html" || request.url().contains(".css") || request.url().contains(".js") || request.url().contains(".png") || request.url().contains(".jpg") || request.url().contains(".tff") || request.url().contains(".woff") || request.url().contains(".woff2") {
                    let response = rouille::match_assets(request, "/opt/aog/www/");
                    if response.is_success() {
                        return response.with_additional_header("Access-Control-Allow-Origin", "*").with_no_cache();
                    } else {
                        return Response::html("404 error").with_status_code(404).with_additional_header("Access-Control-Allow-Origin", "*");
                    }
                }
            
            
    
    
                if session_authenticated{
                    let response = rouille::match_assets(request, "/opt/aog/www/");
                    if response.is_success() {
                        response.with_additional_header("Access-Control-Allow-Origin", "*").with_no_cache()
                    } else {
                        Response::html("404 error").with_status_code(404).with_additional_header("Access-Control-Allow-Origin", "*")
                    }
                } else {
                    //unathuenticated
                    
                    Response::redirect_302("/login.html")
                }
    
          
    
      
            })
    
    
        }
    }, cert, pkey).unwrap().run();
    
}


// TODO - Add Security flag to only allow connections from localhost
pub fn init_command_api(){



    let _config = Arc::new(Mutex::new(Config::load(0).unwrap()));
    let cert = std::fs::read("/opt/aog/crt/default/aog.local.cert").unwrap();
    let pkey = std::fs::read("/opt/aog/crt/default/aog.local.key").unwrap();
    
    rouille::Server::new_ssl("0.0.0.0:9443", move |request| {
        {
       
            let input = try_or_400!(post_input!(request, {
                input_command: String,
            }));
            if input.input_command == *"admin" {
                
            }


            let _ = aog::command::run(input.input_command);


            #[derive(Serialize, Deserialize, Debug, Clone)]
            struct CommandStatus {
                status: String
            }
            // let arduino_response = crate::aog::sensors::get_arduino_raw();
            let response = Response::json(&CommandStatus { status: "success".to_string() });
            return response;


        }
    }, cert, pkey).unwrap().run();
    
}
