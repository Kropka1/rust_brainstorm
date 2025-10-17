use thiserror::Error;
use sea_orm::DbErr;



#[derive(Debug, Error)]
pub enum InfraError {
    #[error(transparent)]
    DataBase(#[from] DbErr),
}


