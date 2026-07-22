use authenticator::UserToken as AuthUserToken;
use uuid::Uuid;

pub struct UserToken {
    pub id: Uuid,
    pub _realm: String,
}

impl From<AuthUserToken> for UserToken {
    fn from(value: AuthUserToken) -> Self {
        Self {
            id: value.id,
            _realm: value.realm,
        }
    }
}
