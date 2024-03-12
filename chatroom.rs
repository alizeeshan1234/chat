pub mod ChatRoom {
    use std::collections::HashMap;
    use crate::user::ChatUser::Message::user_message;
    use crate::user::ChatUser::User::user_details;
    use crate::usermanagement::UserManagement::CommonErrors;

    #[derive(Debug)]
    pub struct chat_room {
        pub messages : HashMap<String , user_message>,
    }

    impl chat_room {
        pub fn new_chat_room() -> chat_room {
            chat_room {
                messages : HashMap::new(),
            }
        }

        pub fn add_user(&mut self , name : user_details , user_message : user_message){
            self.messages.insert(name.user_name , user_message);
        }

        pub fn send_message(&mut self , user_name : String , message : user_message) -> Result<() , CommonErrors>{
            for (user , _) in &mut self.messages {
                if user == &user_name {
                    self.messages.insert(user_name, message);
                    return Ok(());
                }else {
                    return Err(CommonErrors::UserNotFound);
                }
            }
            Err(CommonErrors::InvalidMessageContent)
        }
    }
}