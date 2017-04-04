use dao::SqlConfig;
use dao::query::QueryMap;
use common::model::Composite;

pub trait Dao<E> {
    fn insert<T>(&SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: Composite<E>;

    fn delete<U>(&SqlConfig, c: &mut U) -> Result<i32, String>
        where U: QueryMap;

    fn read<T, U>(&SqlConfig, c: &mut U) -> Result<T, String>
        where T: Composite<E>, U: QueryMap;

    fn update<T>(&SqlConfig, e: &T) -> Result<Vec<i32>, String>
        where T: Composite<E>;

    fn list<T, U>(&SqlConfig, c: &mut U) -> Result<Vec<T>, String>
        where T: Composite<E>, U: QueryMap;
}
