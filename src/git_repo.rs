use anyhow::Result;
use git2::Repository;

pub struct GitRepo {
    pub repo: Repository,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub is_head: bool,
}

impl GitRepo {
    pub fn init() -> Result<Self> {
        let repo = Repository::discover(".")?;
        Ok(GitRepo { repo })
    }

    pub fn list_local_branches(&self) -> Vec<Branch> {
        let branches = self
            .repo
            .branches(Some(git2::BranchType::Local))
            .expect("Get branches");

        let mut branches: Vec<Branch> = branches
            .into_iter()
            .map(|branch| {
                let (branch, _) = branch.unwrap();
                let name = branch.name().unwrap().unwrap().to_string();
                Branch {
                    name,
                    is_head: branch.is_head(),
                }
            })
            .collect();

        branches.sort_by(|a, b| {
            if a.is_head {
                return std::cmp::Ordering::Less;
            } else if b.is_head {
                return std::cmp::Ordering::Greater;
            }

            a.name.cmp(&b.name)
        });

        branches
    }

    pub fn get_branch_at(&self, index: usize) -> Result<Branch> {
        let (branch, _) = self
            .repo
            .branches(Some(git2::BranchType::Local))
            .expect("Get branches")
            .nth(index)
            .unwrap()?;

        let branch = Branch {
            name: branch.name()?.unwrap().to_string(),
            is_head: branch.is_head(),
        };

        Ok(branch)
    }

    pub fn checkout_branch(&self, branch_name: &str) -> Result<()> {
        let (object, reference) = self.repo.revparse_ext(branch_name)?;

        self.repo.checkout_tree(&object, None)?;

        let _ = match reference {
            Some(gref) => self.repo.set_head(gref.name().unwrap()),
            None => self.repo.set_head_detached(object.id()),
        };

        Ok(())
    }

    pub fn delete_branch_at(&self, index: usize) -> Result<()> {
        let branches = self.list_local_branches();
        let branch = branches.iter().nth(index);

        match branch {
            Some(branch) => {
                let branch_name = branch.name.as_str();

                self.repo
                    .find_branch(branch_name, git2::BranchType::Local)?
                    .delete()?;

                Ok(())
            }
            None => Err(anyhow::anyhow!("Branch not found")),
        }
    }
}
