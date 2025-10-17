use crate::repository::GameScoreRepository;
use crate::errors::gamescore::GameScoreError;
use crate::entity::gamescore::{ActiveModel, Model};


#[derive(Clone)]
pub struct GameScoreService{
    repository: GameScoreRepository,
}


impl GameScoreService{
    pub fn new(repository: GameScoreRepository) -> Self{
        Self{
            repository
        }
    }

    pub async fn create(
        self,
        user_id: i64,
        game_type: String,
        game_level: i8
    ) -> Result<ActiveModel, GameScoreError>{
        Ok(
            self.repository.create(
                user_id,
                game_type,
                game_level,
            ).await?
        )
    }

    pub async fn update(
        self,
        user_id: i64,
        game_level: i8,
        game_type: String,
        new_score: i32,
    ) -> Result<ActiveModel, GameScoreError>{
        Ok(
            self.repository.update_for_user(
                game_type,
                game_level,
                user_id,
                new_score
            ).await?
        )
    }

    pub async fn get_user_scores(
        self,
        game_type: String,
        user_id: i64,
    ) -> Result<Vec<Model>, GameScoreError>{
        Ok(
            self.repository.get_all_for_user(
                game_type,
                user_id
            ).await?
        )
    }

    pub async fn get_all(
        self,
        game_type: String,
        game_level: i8,
        limit: u64,
    ) -> Result<Vec<Model>, GameScoreError>{
        Ok(
            self.repository.get_scores(
                game_type,
                game_level,
                limit
            ).await?
        )
    }
    
}
