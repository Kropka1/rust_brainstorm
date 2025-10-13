use crate::{entity::friends::ActiveModel, errors::friends::FriendError, repository::FriendRepository};
use sea_orm::DeleteResult;

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
        match self.repository.new_record(request_id, reciever_id).await{
            Ok(fr) => Ok(fr),
            Err(err) => Err(err) 
        }
    }
    
    pub async fn approve_friend_request(self, request_id: i64, reciever_id: i64) -> Result<ActiveModel, FriendError>{
        match self.repository.change_status(request_id, reciever_id, "approved".to_string()).await{
            Ok(ft) => Ok(ft),
            Err(err) => Err(err)
        }

    }

    pub async fn delete_friend_request(self, request_id: i64, reciever_id: i64) -> Result<DeleteResult, FriendError>{
        match self.repository.delete_record(request_id, reciever_id).await{
            Ok(ft) => Ok(ft),
            Err(err) => Err(err)
        }
    }

}
