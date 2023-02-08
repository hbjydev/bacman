use crate::schema::job::JobRunError;

pub trait JobTypeImpl<T, E> {
    fn run(&self) -> Result<bool, JobRunError<E>>;
}
