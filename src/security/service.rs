use crate::entity::user;
use crate::entity::invitecode;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait};
use crate::errors::auth::AuthError;
use super::{Claim, AuthKeys, AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use chrono::{Utc, Duration};


pub struct AuthService{
    pub db: DatabaseConnection,
    pub auth_keys: AuthKeys,
}

impl AuthService{
    pub fn new(db: DatabaseConnection, auth_keys: AuthKeys) -> Self{
        Self{
            db,
            auth_keys,
        }
    }
    pub async fn get_referer_id_and_make_invalid_ref(&self, ref_code: String) -> Result<i64, AuthError>{
        let referer = invitecode::Entity::find()
            .filter(invitecode::Column::Code.eq(ref_code))
            .filter(invitecode::Column::IsUsed.eq(0))
            .one(&self.db)
            .await
            .map_err(|_| AuthError::DataBaseError)?;
        match referer{
            Some(referer) => {
                let user_id = referer.user_id;
                let mut active_model: invitecode::ActiveModel = referer.into();
                active_model.is_used = Set(1);
                active_model.update(&self.db).await
                    .map_err(|_| AuthError::DataBaseError)?;
                Ok(user_id)
            
            }
        None => Err(AuthError::InvalidRefCode),
        }
    }
    
    pub async fn login(
        &self,
        login_request: LoginRequest,
    ) -> Result<AuthResponse, AuthError>{
        let maybe_user = user::Entity::find()
            .filter(user::Column::Username.eq(login_request.username.clone()))
            .one(&self.db)
            .await
            .map_err(|_| AuthError::DataBaseError)?;
        let user_model: user::Model = match maybe_user{
            Some(user) => user,
            None => {
                return Err(AuthError::InvalidCredentials);
            }
        };
        let hashed = user_model.hashed_password.clone();
        let ok = verify(login_request.password, &hashed)
            .map_err(|_| AuthError::HashingError)?;
        if !ok{
            return Err(AuthError::InvalidCredentials)
        }
        let user_active: user::ActiveModel = user_model.clone().into();
        let token = self.generate_token(&user_active)?;
        Ok(
            AuthResponse {
                token: (token),
                user: UserResponse { id: (user_model.id.to_string()), username: (user_model.username.clone()) },
        }
        )
       
    }

    pub async fn register(
        &self,
        register_request: RegisterRequest,
    ) -> Result<AuthResponse, AuthError>{
        let referer = self.get_referer_id_and_make_invalid_ref(register_request.referer_code).await?;
        let existing_user = user::Entity::find()
            .filter(user::Column::Username.eq(register_request.username.clone()))
            .one(&self.db)
            .await
            .map_err(|_| AuthError::DataBaseError)?;
        if existing_user.is_some(){
            return Err(AuthError::UserAlreadyExist);
        }
        let pass_hash = hash(&register_request.password, DEFAULT_COST)
            .map_err(|_| AuthError::HashingError)?;
        let user_active = user::ActiveModel{
            created_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
            updated_at: Set(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32),
            hashed_password: Set(pass_hash),
            username: Set(register_request.username.to_owned()),
            logo: Set("default.png".to_string()),
            role: Set("user".to_string()),
            referer: Set(referer),
            ..Default::default()
            
        };
        let user_model = user::Entity::insert(user_active).exec_with_returning(&self.db).await?;
        
        let token = self.generate_token(&user_model)?;
        Ok(
            AuthResponse { token: (token), user: UserResponse { id: (user_model.id.unwrap().to_string()), username: (user_model.username.clone().unwrap()) } }
        )

    }
    fn generate_token(&self, user_model: &user::ActiveModel) -> Result<String, AuthError>{
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);
        let claim = Claim{
            sub: user_model.id.clone().take().unwrap(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
            username: user_model.username.clone().take().unwrap(),
        };
        encode(&Header::default(), &claim, &self.auth_keys.encoding)
            .map_err(|_| AuthError::TokenCreationError)
    }
}
