use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    
}

pub type Result<V> = std::result::Result<V, Error>;