use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub database_url: Cow<'static, str>,
    pub port: u16,
    pub secret: Cow<'static, str>,
    pub default_admin_username: Cow<'static, str>,
    pub default_admin_password: Cow<'static, str>,
}

impl EnvironmentVariables {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            database_url: match dotenv::var("DATABASE_URL") {
                Ok(url) => url.into(),
                Err(err) => panic!("missing DATABASE_URL: {err}"),
            },
            port: match dotenv::var("PORT") {
                Ok(port) => port.parse()?,
                _ => 8000,
            },
            secret: match dotenv::var("SECRET") {
                Ok(secret) => secret.into(),
                Err(err) => panic!("missing SECRET: {err}"),
            },
            default_admin_username: match dotenv::var("DEFAULT_ADMIN_USERNAME") {
                Ok(default_admin_username) => default_admin_username.into(),
                Err(err) => panic!("missing default admin username: {err}"),
            },
            default_admin_password: match dotenv::var("DEFAULT_ADMIN_PASSWORD") {
                Ok(default_admin_password) => default_admin_password.into(),
                Err(err) => panic!("missing default admin password: {err}"),
            },


        })
    }
}
