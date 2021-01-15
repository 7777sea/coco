use serde::{Deserialize, Serialize};

use crate::domain::git::coco_branch::CocoBranch;
use crate::infrastructure::git::git_branch::GitBranch;
use crate::infrastructure::git::GitRepository;
use crate::infrastructure::time_format::format_unix_time;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormatBranch {
    pub name: String,
    pub author: String,
    pub committer: String,
    pub first_commit_str: String,
    pub last_commit_str: String,
    pub first_commit_date: i64,
    pub last_commit_date: i64,
}

impl FormatBranch {
    pub fn from(br: CocoBranch) -> FormatBranch {
        FormatBranch {
            name: br.name,
            author: br.author,
            committer: br.committer,

            first_commit_str: format_unix_time(br.first_commit_date),
            last_commit_str: format_unix_time(br.last_commit_date),
            first_commit_date: br.first_commit_date,
            last_commit_date: br.last_commit_date,
        }
    }
}

pub fn get_repo(url: &str) -> String {
    let repo = GitRepository::clone(url);
    let mut branches = vec![];

    for br in GitBranch::list(repo) {
        branches.push(FormatBranch::from(br));
    }

    let output = serde_json::to_string_pretty(&branches).unwrap();
    return output;
}

#[cfg(test)]
mod test {
    use crate::app::git_analysis::FormatBranch;
    use crate::domain::git::coco_branch::CocoBranch;

    #[test]
    fn should_output_really_date() {
        let branch = FormatBranch::from(CocoBranch {
            name: "master".to_string(),
            first_commit_date: 1610519809,
            last_commit_date: 1610541520,
            duration: 21711,
            author: "GitHub".to_string(),
            committer: "Phodal HUANG".to_string(),
        });

        assert_eq!("2021-01-13 06:36:49", branch.first_commit_str);
        assert_eq!("2021-01-13 12:38:40", branch.last_commit_str);
    }
}
