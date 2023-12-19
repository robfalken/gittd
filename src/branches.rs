use anyhow::Result;
use git2::Repository;

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub current: bool,
}
