use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set,
};
use crate::entity::user::{
    ActiveModel,
    Column,
    Entity,
    Role,
    Model,
};
use crate::errors::infra::InfraError;

#[derive(Clone)]
pub struct UserRepository{
    db: DatabaseConnection,
}

impl UserRepository{
    pub fn new(db: DatabaseConnection) -> Self{
        Self{
            db,
        }
    }

    pub async fn get_user(
        &self,
        user_id: i64,
    ) -> Result<Option<Model>, InfraError>{
        let user = Entity::find()
            .filter(Column::Id.eq(user_id))
            .one(&self.db)
            .await?;
        Ok(user)
    }

    pub async fn update_role(
        self,
        user_id: i64,
        role: Role
    ) -> Result<ActiveModel, InfraError>{
        let user = self.get_user(user_id)
            .await?
            .unwrap();
        let mut user = user.into_active_model();
        user.role = Set(role.to_string());
        Ok(user) 
    }

    pub async fn update_file(
        self,
        user_id: i64,
        file_name: String,
        ) -> Result<ActiveModel, InfraError>{
        let user = self.get_user(user_id)
            .await?
            .unwrap();
        let mut user: ActiveModel = user.into_active_model();
        user.logo = Set(file_name);
        Ok(user.update(&self.db).await?)

    } 

    pub async fn update_username(
        self,
        user_id: i64,
        username: String,
    ) -> Result<ActiveModel, InfraError>{
        let user = self.get_user(user_id)
            .await?
            .unwrap();
        let mut user: ActiveModel = user.into_active_model();
        user.username = Set(username);
        Ok(user.update(&self.db).await?)
    }

    
    
}
