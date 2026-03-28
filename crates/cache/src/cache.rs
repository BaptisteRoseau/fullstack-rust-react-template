use async_trait::async_trait;

#[async_trait]
pub trait Cache: Send + Sync {
    // fn set<T: Serialise + Deserialise>(key: &dyn ToString, value: &T, timeout_s: Option<u32>);
    // fn get<T: Deserialise>(key: &dyn ToString) -> T;
    // fn delete<T: Deserialise>(key: &dyn ToString);

    // fn set<T: Serialise + Deserialise>(key: &dyn ToString, value: &T);
    // fn get<T: Deserialise>(key: &dyn ToString) -> T;
    // fn delete<T: Deserialise>(key: &dyn ToString);
}
