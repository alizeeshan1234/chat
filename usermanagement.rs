pub mod UserManagement {
    use std::collections::HashMap;
    use crate::user::ChatUser::User::user_details;
    #[derive(Debug)]
    pub struct UserRepository {
        pub users : HashMap<u32 , user_details>,
    }

    pub enum CommonErrors {
        UserNotFound,
        MessageNotFound,
        InvalidMessageContent
    }
}