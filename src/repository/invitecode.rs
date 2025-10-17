use sea_orm::DatabaseConnection;
use crate::entity::invitecode;
use crate::errors::invitecode::InviteCodeError;
use sea_orm::{EntityTrait, QueryFilter, ActiveModelTrait, Set, ColumnTrait};
use std::time::{UNIX_EPOCH, SystemTime};


#[derive(Clone)]
pub struct InviteCodeRepository{
    db: DatabaseConnection,
}


impl InviteCodeRepository{
    pub fn new(db: DatabaseConnection) -> Self{
        Self{
            db
        }
    }
    async fn generate_random_code(&self) -> String{
        let value: String = (0..6).map(|_| char::from(
                match rand::random_range(0..3){
                    1 => rand::random_range(48..58),
                    2 => rand::random_range(65..91),
                    _ => rand::random_range(97..123)

                }
        )).collect();
        value

    }
    pub async fn get_by_code(&self, code: String) -> Result<invitecode::ActiveModel, InviteCodeError>{
        let invite_code = invitecode::Entity::find()
            .filter(invitecode::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(|_| InviteCodeError::DataBaseError)?;
        match invite_code{
            Some(invite_code_entity) => Ok(invite_code_entity.into()),
            None => Err(InviteCodeError::NotExistError),
        }
    }
    pub async fn update_code_status(self, code: String, is_used: i8) -> Result<invitecode::ActiveModel, InviteCodeError>{
        let mut invite_code: invitecode::ActiveModel = self.get_by_code(code).await?;
        invite_code.is_used = Set(is_used);
        invite_code.clone().update(&self.db).await
            .map_err(|_| InviteCodeError::DataBaseError)?;
        Ok(invite_code)
        
    }
    pub async fn create_code(self, user_id: i64) -> Result<invitecode::ActiveModel, InviteCodeError>{
        let code: String = self.generate_random_code().await;
        let invite_code = invitecode::ActiveModel{
              created_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
              updated_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
              user_id: Set(user_id),
              code: Set(code),
              is_used: Set(0),
              ..Default::default()
        };
        let res = invitecode::Entity::insert(invite_code)
            .exec_with_returning(&self.db)
            .await
            .map_err(|_| InviteCodeError::DataBaseError)?;
        Ok(res)
    }
}
