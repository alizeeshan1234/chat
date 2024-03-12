pub mod ChatUser {
pub mod User {
    #[derive(Debug)]
    pub struct user_details {
        pub user_name : String,
        pub user_id : u32,
    }
}

pub mod Message {
    #[derive(Debug)]
    pub struct user_message {
        pub sender : String,
        pub content : String,
        pub timestamp : String,
    }
}
}