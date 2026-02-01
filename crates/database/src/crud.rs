pub trait Crud {
    fn create_query(&self) -> &'static str;
    fn read_by_id_query(&self) -> &'static str;
    fn update_query(&self) -> &'static str;
    fn delete_by_id_query(&self) -> &'static str;
    fn as_params(&self) -> [&str];
}
