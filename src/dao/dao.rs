use dao::SqlConfig;
use dao::query::QueryMap;
use common::model::Composite;

pub trait Dao<E, U> where U: QueryMap {
    fn insert<T>(&SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: Composite<E>;

    fn delete(&SqlConfig, c: &mut U) -> Result<i32, String>;

    fn read<T>(&SqlConfig, c: &mut U) -> Result<T, String>
        where T: Composite<E>;

    fn update<T>(&SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: Composite<E>;

    fn list<T>(&SqlConfig, c: &mut U) -> Result<Vec<T>, String>
        where T: Composite<E>;
}
