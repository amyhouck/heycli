use crate::api;
use clap::{Args, Subcommand};

//---------------------
// Command Structure
//---------------------
// Notification commands
#[derive(Args)]
pub struct NotifArgs {
    #[command(subcommand)]
    command: Option<NotifCommands>
}

#[derive(Subcommand)]
pub enum NotifCommands {
    /// Mark a notification as seen
    #[group(required = true, multiple = false)]
    Remove {
        /// Remove a specific notification by ID of the list.
        #[arg(short, long)]
        id: Option<usize>,
        
        /// Clear all notifications
        #[arg(short, long)]
        all: bool
    },
    
    /// View new notifications
    View
}

// Handler for notification commands
pub fn notif_handler(sub_com: NotifArgs) {
    match sub_com.command {
        Some(NotifCommands::Remove{id, all}) => {
            if let Some(id) = id {
                remove_notification(id);
            }
            
            if all {
                clear_notifications();
            }
        },
        None | Some(NotifCommands::View) => display_notifications()
    }
}

//---------------------
// Command Library
//---------------------
// Displays any new notifications. Max of 10.
pub fn display_notifications() {
    let mut notifications = api::heycafe_get_notifications(true);
    notifications.truncate(10);
    
    println!("Notifications: {}", notifications.len());
    println!("--------------------");
    if notifications.is_empty() {
        println!("No new notifications!");
    } else {
        for (i, notif) in notifications.iter().enumerate() {
            let from = notif["from"]["name"].as_str().unwrap();
            let msg = notif["content"].as_str().unwrap();
            
            println!("{}. {from} {msg}", i + 1);
        }
    }
}

// Remove a single notification
fn remove_notification(id: usize) {
    let notification = api::heycafe_get_notifications(true);
    
    // Validate the ID
    if notification.is_empty() {
        panic!("ERROR: There are no new notifications!");
    }
    
    if id > notification.len() {
        panic!("ERROR: The ID you have selected is out of index!");
    }
    
    // Do the thing
    let notification = &notification[id - 1];
    let id = notification["id"].as_str().unwrap();
    
    api::heycafe_post_account_notification_seen(Some(id), false);
    println!("Notification marked as seen!");
}

// Clear all notifications
fn clear_notifications() {
    api::heycafe_post_account_notification_seen(None, true);
    println!("All new notifications have been marked seen!");
}