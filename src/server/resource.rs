use serde::Deserialize;

#[derive(Deserialize)]
pub struct GithubRepository {
    pub ssh_url: String
}

#[derive(Deserialize)]
pub struct GithubPushWebhook {
    pub repository: GithubRepository
}

#[derive(Deserialize)]
pub struct GitlabRepository {
    pub git_ssh_url: String
}

#[derive(Deserialize)]
pub struct GitlabPushWebhook {
    pub repository: GitlabRepository
}
