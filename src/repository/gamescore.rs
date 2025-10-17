use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, ColumnTrait, Set};
use crate::entity::gamescore::{Entity, ActiveModel, Column, Model};
use crate::errors::infra::InfraError;
use std::time::{UNIX_EPOCH, SystemTime};

#[derive(Clone)]
pub struct GameScoreRepository{
    db: DatabaseConnection,
}

impl GameScoreRepository{
    pub fn new (db: DatabaseConnection) -> Self{
        Self{
            db
        }
    }

    pub async fn create(
        self,
        user_id: i64,
        game_type: String,
        game_level: i8
    ) -> Result<ActiveModel, InfraError>{
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;
        let a_model = ActiveModel{
            created_at: Set(now),
            updated_at: Set(now),
            user_id: Set(user_id),
            game_type: Set(game_type),
            game_level: Set(game_level),
            score: Set(0),
            ..Default::default()
        };
        
        let model = Entity::insert(a_model)
            .exec_with_returning(&self.db)
            .await?;
        Ok(model)
    }

    async fn get_for_user(
        &self,
        user_id: i64,
        game_level: i8,
        game_type: String,
    ) -> Result<Option<Model>, InfraError>{
        let score = Entity::find()
            .filter(Column::GameType.eq(game_type))
            .filter(Column::UserId.eq(user_id))
            .filter(Column::GameLevel.eq(game_level))
            .one(&self.db)
            .await?;
        Ok(score)

    }

    pub async fn update_for_user(
        self,
        game_type: String,
        game_level: i8,
        user_id: i64,
        new_score: i32,
    ) -> Result<ActiveModel, InfraError>{
        let Some(model) = self.get_for_user(
            user_id,
            game_level,
            game_type
        ).await? else{
            return Err(InfraError::DataBase(
                    sea_orm::DbErr::RecordNotFound("not found".into())
            ));
        };
        let mut active: ActiveModel = model.into();
        //update at add
        active.score = Set(new_score);
        Ok(active.update(&self.db).await?)
    }

    pub async fn get_all_for_user(
        self,
        game_type: String,
        user_id: i64,
    ) -> Result<Vec<Model>, InfraError>{
        let scores: Vec<Model> = Entity::find()
            .filter(Column::GameType.eq(game_type))
            .filter(Column::UserId.eq(user_id))
            .order_by_asc(Column::GameType)
            .all(&self.db)
            .await?;
        Ok(scores)

    }

    pub async fn get_scores(
        self,
        game_type: String,
        game_level: i8,
        limit: u64,
    ) -> Result <Vec<Model>, InfraError>{
        let scores: Vec<Model> = Entity::find()
            .filter(Column::GameType.eq(game_type))
            .filter(Column::GameLevel.eq(game_level))
            .limit(limit)
            .order_by_desc(Column::Score)
            .all(&self.db)
            .await?;
        Ok(scores)

    }
}
