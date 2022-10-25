use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubUrl {
    pub username: String,
    pub repos: Vec<GithubRepo>,

    #[serde(skip_serializing, skip_deserializing)]
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRepo {
    pub name: String,
    pub owner: RepoOwner,
    pub html_url: String,
    #[serde(skip_deserializing)]
    pub commits: Vec<Commit>,
    pub fork: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub sha: String,
    pub html_url: String,
    pub author: RepoAuthor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoAuthor {
    pub login: String,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoOwner {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

impl GithubUrl {
    pub fn new(username: String) -> GithubUrl {
        GithubUrl {
            username: username,
            repos: Vec::new(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_repos(&mut self) -> Result<&str, reqwest::Error> {
        let res = self
            .client
            .get(&format!(
                "https://api.github.com/users/{0}/repos",
                self.username
            ))
            .header("User-Agent", "github-socials")
            .send() // Send the req
            .await?
            .text()
            .await?; // response
        let repos: Vec<GithubRepo> = serde_json::from_str(res.as_str()).unwrap();
        self.repos.extend(repos);
        Ok("successfully fetched")
    }

    /// Returns Forked Repos
    pub fn get_forked_repos(&mut self) -> Vec<GithubRepo> {
        self.repos
            .clone()
            .into_iter()
            .filter(|voc| voc.fork == true)
            .collect()
    }
}
