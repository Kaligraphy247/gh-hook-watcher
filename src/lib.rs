use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub name: String,
    pub email: String,
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub user_view_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    #[serde(rename = "private")]
    pub r#priv: bool,
    pub owner: Owner,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: u64,
    pub updated_at: String,
    pub pushed_at: u64,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: Option<String>,
    pub size: u64,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_discussions: bool,
    pub forks_count: u32,
    pub mirror_url: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: u32,
    pub license: Option<String>,
    pub allow_forking: bool,
    pub is_template: bool,
    pub web_commit_signoff_required: bool,
    pub topics: Vec<String>,
    pub visibility: String,
    pub forks: u32,
    pub open_issues: u32,
    pub watchers: u32,
    pub default_branch: String,
    pub stargazers: u32,
    pub master_branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pusher {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub user_view_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubPayloadBody {
    #[serde(rename = "ref")] // GitHub uses "ref" which is a keyword in Rust
    pub reference: String,
    #[serde(rename = "before")]
    pub before_commit: String,
    #[serde(rename = "after")]
    pub after_commit: String,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: Sender,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub compare: String,
    pub commits: Vec<CommitDetails>,
    pub head_commit: CommitDetails,
}

impl GitHubPayloadBody {
    pub fn convert_to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}
