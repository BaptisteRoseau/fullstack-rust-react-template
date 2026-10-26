use authenticator::Caller;

/// The extractor resolving the request's credential into a [`Caller`].
pub struct UserToken(Caller);

impl UserToken {
    /// The caller handed to `app_core`, which runs every access check.
    pub fn caller(&self) -> &Caller {
        &self.0
    }

    pub fn into_caller(self) -> Caller {
        self.0
    }
}

impl From<Caller> for UserToken {
    fn from(caller: Caller) -> Self {
        Self(caller)
    }
}
