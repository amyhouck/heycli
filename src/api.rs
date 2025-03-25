use serde_json::Value;
use crate::{func::{get_hey_cafe_api, post_hey_cafe_api}, API_URL};

//---------------------
// GET - No Key
//---------------------
pub fn heycafe_get_account_info(user_id: String) -> Value {
    let url = format!("{API_URL}get_account_info?query={user_id}");
    
    get_hey_cafe_api(url)
}

//---------------------
// GET - Key Required
//---------------------
pub fn heycafe_get_notifications(new: bool) -> Vec<Value>{
    let seen = match new {
        true => "no",
        false => "yes"  
    };
    
    let url = format!("{API_URL}get_account_notifications?seen={seen}&convert_numeric=notifications");
    let notifications = get_hey_cafe_api(url);
    
    let notifications_vec = notifications["response_data"]["notifications"]
        .as_array()
        .unwrap_or(&Vec::new())
        .to_vec();
        
    notifications_vec
}

pub fn heycafe_get_chat_list() -> serde_json::Value {
    let url = format!("{API_URL}get_chat_list?convert_numeric=chats");
    let chat_list = get_hey_cafe_api(url);
    
    chat_list
}

pub fn heycafe_get_account_loginkey(key: &str) -> serde_json::Value {
    let url = format!("{API_URL}get_account_loginkey?query={key}");
    let loginkey = get_hey_cafe_api(url);
    
    loginkey
}

//---------------------
// POST - Key Required
//---------------------
pub fn heycafe_post_account_notification_seen(
    id: Option<&str>,
    all: bool
) {
    let url = match all {
        true => format!("{API_URL}post_account_notification_seen?type=all"),
        false => format!("{API_URL}post_account_notification_seen?notification={}", id.unwrap())
    };
    
    post_hey_cafe_api(url);
}