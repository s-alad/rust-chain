use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}