use sea_orm::DatabaseConnection;
use crate::entity::friends::{self, ActiveModel};
use sea_orm::{Set, EntityTrait, ActiveModelTrait, QueryFilter, ColumnTrait, DeleteResult};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::errors::friends::FriendError;

#[derive(Clone)]
pub struct FriendRepository{
    db: DatabaseConnection,
}


impl FriendRepository{
    pub fn new(db: DatabaseConnection) -> Self{
        Self{
            db,
        }
    }
    async fn find_fr(&self, requester: i64, reciever: i64) -> Result<ActiveModel, FriendError>{
        let friend_request = friends::Entity::find()
            .filter(friends::Column::UserReciever.eq(reciever))
            .filter(friends::Column::UserRequestor.eq(requester))
            .one(&self.db)
            .await
            .map_err(|_| FriendError::DataBaseError)?;
        match friend_request {
            Some(fr) => { 
                Ok(fr.into())
            }
            None => Err(FriendError::NoFriendRequest)
            
        }
        


    }
    pub async fn delete_record(self, requester: i64, reciever: i64) -> Result <DeleteResult, FriendError>{
        let fr_m: ActiveModel = self.find_fr(requester, reciever).await?;
        let res = fr_m.delete(&self.db).await
            .map_err(|_| FriendError::DataBaseError)?;
        Ok(res)
        
    }
    pub async fn new_record(self, requester: i64, reciever: i64) -> Result<ActiveModel, FriendError>{
        let friend_request = friends::ActiveModel{
        created_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
        updated_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
        user_requestor: Set(requester),
        user_reciever: Set(reciever),
        status: Set("pending".to_string()),
        ..Default::default()
        };
        let friend_m = friends::Entity::insert(friend_request).exec_with_returning(&self.db).await?;
        Ok(friend_m)

    }
    
    pub async fn change_status(self, requester: i64, reciever: i64, status: String) -> Result<ActiveModel, FriendError>{
        let mut fr_m: ActiveModel = self.find_fr(requester, reciever).await?;
        fr_m.status = Set(status);
        fr_m.clone().update(&self.db).await
            .map_err(|_| FriendError::DataBaseError)?; 
        Ok(fr_m)

    }
    
}
