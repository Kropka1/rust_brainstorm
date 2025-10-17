use crate::{entity::friends::{ActiveModel, Status}, errors::{friends::FriendError}, repository::FriendRepository};


#[derive(Clone)]
pub struct FriendService{
    repository: FriendRepository,
}


impl FriendService{
    pub fn new(repository: FriendRepository) -> Self{
        Self{
            repository
        }
    }

    pub async fn send_friend_request(self, request_id: i64, reciever_id: i64) -> Result<ActiveModel, FriendError>{
        Ok(
            self.repository.new_record(request_id, reciever_id).await?
        )
    }
    
    pub async fn approve_friend_request(self, request_id: i64, reciever_id: i64) -> Result<ActiveModel, FriendError>{
        Ok(
            self.repository.change_status(reciever_id, request_id, Status::Accepted.to_string()).await?
        )
    }

    pub async fn delete_friend_request(self, request_id: i64, reciever_id: i64) -> Result<(), FriendError>{
        Ok(
            self.repository.delete_record(request_id, reciever_id).await?
        )
    }

}
