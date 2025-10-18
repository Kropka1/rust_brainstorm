use sea_orm::{DatabaseConnection, IntoActiveModel};
use crate::entity::friends::{Status, ActiveModel, Model, self};
use sea_orm::{Set, EntityTrait, ActiveModelTrait, QueryFilter, ColumnTrait};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::errors::infra::InfraError;

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
    async fn find_fr(&self, requester: i64, reciever: i64) -> Result<Option<Model>, InfraError>{
        let friend_request = friends::Entity::find()
            .filter(friends::Column::UserReciever.eq(reciever))
            .filter(friends::Column::UserRequestor.eq(requester))
            .one(&self.db)
            .await?;
        Ok(friend_request)
    }
    pub async fn delete_record(self, requester: i64, reciever: i64) -> Result <(), InfraError>{
        let fr_m = self.find_fr(requester, reciever).await?.unwrap();
        fr_m.into_active_model().delete(&self.db).await?;
        Ok(())
        
    }
    pub async fn new_record(self, requester: i64, reciever: i64) -> Result<ActiveModel, InfraError>{
        let friend_request = friends::ActiveModel{
        created_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
        updated_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
        user_requestor: Set(requester),
        user_reciever: Set(reciever),
        status: Set(Status::Pending.to_string()),
        ..Default::default()
        };
        let friend_m = friends::Entity::insert(friend_request).exec_with_returning(&self.db).await?;
        Ok(friend_m)

    }
    
    pub async fn change_status(self, requester: i64, reciever: i64, status: String) -> Result<ActiveModel, InfraError>{
        let Some(model) = self.find_fr(requester, reciever).await? else{
            return Err(InfraError::DataBase(sea_orm::DbErr::RecordNotFound(
                        "not found".into()
            )));
        };
        let mut active: ActiveModel = model.into();
        //update at add
        active.status = Set(status);
        Ok(active.update(&self.db).await?)
      
    }
    
}
