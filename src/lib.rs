#![no_std]
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "This is an error message")]
pub struct MyError;
