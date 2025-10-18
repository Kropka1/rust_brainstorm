use axum::body::Bytes;

use crate::errors::user::UserError;
use crate::entity::user::{ActiveModel, Model, Role};
use crate::repository::UserRepository;
use std::path::PathBuf;


#[derive(Clone)]
pub struct UserService{
    repository: UserRepository,
}

impl UserService{
    pub fn new(repository: UserRepository) -> Self{
        Self{
            repository
        }
    }

    pub async fn get_user(
        self,
        user_id: i64,
    ) -> Result<Model, UserError>{
        let user = self.repository.get_user(
            user_id
        ).await?
        .unwrap();
        Ok(user)
        
    }

    pub async fn change_role(
        self,
        user_id: i64,
        role: Role,
    ) -> Result<ActiveModel, UserError>{
        let user = self.repository.update_role(
            user_id,
            role,
        ).await?;
        Ok(user)
    }

    pub async fn update_username(
        self,
        user_id: i64,
        username: String,
    ) -> Result<ActiveModel, UserError>{
        let user = self.repository.update_username(
            user_id,
            username
        ).await?;
        Ok(user)
    }

    pub async fn upload_file(
        self,
        bytes: Bytes,
        user_id: i64,
    ) -> Result<(), UserError>{
        let kind = infer::get(&bytes);
        let allowed = matches!(
            kind.map(|k| k.mime_type()),
            Some("image/png") | Some("image/jpeg") | Some("image/webp")
            | Some("image/gif")
        );

        if !allowed{
            return Err(UserError::NotAllowedImageType);
        }

        let ext = match kind.unwrap().mime_type(){
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/webp" => "webp",
            "image/gif" => "gif",
            _ => "bin",
        };

        let filename = format!("{}.{}", user_id, ext);
        //conf path for images
        let mut path = PathBuf::from("./uploads");
        let _ = tokio::fs::create_dir_all(&path).await.map_err(
        |_| UserError::InternalError
        );
        path.push(filename.clone());

        let _ = tokio::fs::write(&path, &bytes).await.map_err(
        |_| UserError::InternalError
        );

        let _ = self.repository.update_file(
            user_id,
            filename,
        ).await?;

        Ok(())

    }
}
