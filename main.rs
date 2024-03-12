mod user;
mod usermanagement;
mod chatroom;

fn main() {
    use crate::user::ChatUser::{User::* , Message::*};
    use crate::chatroom::ChatRoom::*;

    let mut new_chatroom = chat_room::new_chat_room();

    let new_user1 = user_details {
        user_name : "Harry".to_string(),
        user_id : 1,
    };

    let new_user_1_message = user_message {
        sender : "Harry".to_string(),
        content : "Hello World!".to_string(),
        timestamp : "12/03/2024".to_string(),
    };

    new_chatroom.add_user(new_user1, new_user_1_message);

    new_chatroom.send_message("Harry".to_string(), user_message {
        sender : "Harry".to_string(),
        content : "I am Harry".to_string(),
        timestamp : "12/03/2024".to_string(),
    });

    new_chatroom.send_message("Harry".to_string(), user_message {
        sender : "Harry".to_string(),
        content : "I am looking to connect with rust developers".to_string(),
        timestamp : "12/03/2024".to_string(),
    });

    let new_user2 = user_details {
        user_name : "Hermione".to_string(),
        user_id : 2,
    };

    let new_user_2_message = user_message {
        sender : "Hermione".to_string(),
        content : "Hello Harry!".to_string(),
        timestamp : "12/03/2024".to_string(),
    };

    new_chatroom.add_user(new_user2, new_user_2_message);
    new_chatroom.send_message("Hermione".to_string(), user_message {
        sender : "Hermione".to_string(),
        content : "Hello Harry lets connect!".to_string(),
        timestamp : "12/03/2024".to_string(),
    });

    println!("{:?}" , new_chatroom);
}
