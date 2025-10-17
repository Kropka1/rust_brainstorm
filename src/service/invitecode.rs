use crate::repository::InviteCodeRepository;
use crate::errors::invitecode::InviteCodeError;
use crate::entity::invitecode::ActiveModel;

#[derive(Clone)]
pub struct InviteCodeService{
    repository: InviteCodeRepository,
}

impl InviteCodeService{
    pub fn new(repository: InviteCodeRepository) -> Self{
        Self{
            repository
        }
    }

    pub async fn create_new_code(self, user_id: i64) -> Result<ActiveModel, InviteCodeError>{
        let invite_code = self.repository.create_code(user_id).await?;
        Ok(invite_code)
    } 
    
    pub async fn reload_code(self, code: String) -> Result<ActiveModel, InviteCodeError>{
        let invite_code = self.repository.update_code_status(code, 0).await?;
        Ok(invite_code)
    }
}
