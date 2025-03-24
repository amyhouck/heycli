use crate::api;
use crate::func;
use std::fs;
use colored::Colorize;

// Return count of rooms with new messages
fn get_roomcount_new_msg(chat_list: serde_json::Value) -> i32 {
    let rooms = chat_list["response_data"]["chats"].as_array().unwrap();
    
    let mut count = 0;
    for room in rooms {
        let new_messages = room["info_new_messages"].as_i64().unwrap_or(0);
        
        if new_messages > 0 {
            count += 1;
        }
    }
    
    count
}

// Base command. What runs when no subcommand is given. Shows basic account info if key is available, otherwise it only gives the version
pub fn app_base() {
    if func::is_key_set() {
        let notifications = api::heycafe_get_notifications(true);
        
        let chats = api::heycafe_get_chat_list();
        let chat_count = get_roomcount_new_msg(chats);
        
        let user_id = std::env::var("HEYCAFE_USERID").unwrap();
        let account = api::heycafe_get_account_info(user_id);
        let follower_count = account["response_data"]["count_followers"].as_str().unwrap();
        let following_count = account["response_data"]["count_following"].as_str().unwrap();
        
        println!("heycli v0.1.0");
        println!("Notifications: {}", notifications.len());
        println!("Chat Notifications: {chat_count}");
        println!("Followers: {follower_count}");
        println!("Following: {following_count}");
    } else {
        println!("heycli v0.1.0");
        println!("{}", "No key present".red().to_string());
    }
}

// Direct user to API key URL and have them paste it into here
pub fn app_connect() {
    // Grab key
    println!("You are about to link an account on Hey.Cafe with heycli!");
    println!("Please follow the directions below to retrieve your API key.");
    println!("1. Open Hey.Cafe");
    println!("2. Go to Settings > Account Settings > Sessions");
    println!("3. Click \"Create new API key\"");
    println!("4. Write \"heycli\" for the app name and \"account linking\" for the reason. Then press \"Create\"");
    println!("5. Scroll through the sessions until you reach one titled \"heycli / account linking\" and copy the key next to \"Code\"");
    println!("6. Paste the key below:");
    print!("\nKey: ");
    
    let key = func::get_input();
    
    // Verify key
    if func::is_key_valid(&key) {
        let login_key = api::heycafe_get_account_loginkey(&key);
        let user_id = login_key["response_data"]["id"].as_str().unwrap();
        
        let key = format!("HEYCAFE_KEY={key}\nHEYCAFE_USERID={user_id}");
        
        if let Err(e) = fs::write("./heycli.env", key) {
            panic!("ERROR: An error occured trying to write the key to heycli.env! {e}");
        }
        
        println!("SUCCESS: API Key stored!");
    } else {
        println!("ERROR: Invalid API Key!");
    }
}

// Unlink the account from heycli
pub fn app_disconnect() {
    // Check if a key exists
    if std::env::var("HEYCAFE_KEY").is_err() {
        panic!("ERROR: You do not have heycli connected to Hey.Cafe!");
    }
    
    // Run the deletion process
    println!("You are about to unlink your account from Hey.Cafe!");
    print!("Are you sure you would like to do this? (y\\n): ");
    
    let answer = func::get_input().to_lowercase();
    
    match answer.as_str() {
        "y" | "yes" => {
            println!("Deleting API key...");
            
            if let Err(e) = fs::remove_file("./heycli.env") {
                panic!("ERROR: An error occurred trying to delete heycli.env! {e}");
            }
            
            println!("Done!");
        },
        "n" | "no" => return,
        _ => {
            println!("Invalid input!");
            app_disconnect();
        }
    }
}