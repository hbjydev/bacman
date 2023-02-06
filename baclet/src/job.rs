#[derive(Debug)]
pub struct JobRunError<E> {
    pub error: E,
}

pub trait JobType<T, E> {
    fn run(&self) -> Result<bool, JobRunError<E>>;
}
