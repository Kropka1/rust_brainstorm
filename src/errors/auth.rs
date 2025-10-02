

#[derive(Debug)]
pub enum AuthError {
    DataBaseError,
    UserAlreadyExist,
    InvalidCredentials,
    HashingError,
    InvalidToken,
    TokenCreationError,
    InvalidRefCode,
    
    
}

impl std::fmt::Display for AuthError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            Self::DataBaseError => write!(f, "DataBase Error"),
            Self::UserAlreadyExist => write!(f, "User Already Exists"),
            Self::InvalidCredentials => write!(f, "Wrong Username Or Password"),
            Self::HashingError => write!(f, "Hashing Error..."),
            Self::InvalidToken => write!(f, "Invalid Auth Token"),
            Self::TokenCreationError => write!(f, "Failed To Create Token..."),
            Self::InvalidRefCode => write!(f, "Invalid referer code"),
        }
    }
}


impl std::error::Error for AuthError{}
