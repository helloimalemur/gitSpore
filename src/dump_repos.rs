use anyhow::anyhow;
use anyhow::Error;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::*;
use std::process;
use std::process::Stdio;

async fn find_repos(user_name: String, page: i32) -> Result<Vec<Repo>, Error> {
    let client = Client::new();
    let mut headers: HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("User-Agent: ME"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github+json"),
    );
    let url = format!(
        "https://api.github.com/users/{}/repos?per_page=200&page={}",
        user_name, page
    );
    let resp = client
        .get(url.clone())
        .headers(headers)
        .send()
        .await
        .unwrap();
    if !resp.status().is_success() {
        return Err(anyhow!(
            "failed to fetch {}, code {}",
            url,
            resp.status().as_str()
        ));
    }
    let json_text: String = resp.text().await.unwrap();
    let repos: Vec<Repo> = serde_json::from_str(&json_text).unwrap();
    if repos.len() == 0 {
        return Err(anyhow!("null page {} , len {}.", page, repos.len()));
    }
    Ok(repos)
}

pub async fn dump_repos(user_name: String) -> Vec<Repo> {
    let mut res_repos: Vec<Repo> = Vec::new();
    for p in 1..i32::MAX {
        let repos = find_repos(user_name.clone(), p).await;
        match repos {
            Ok(repos) => {
                for repo in repos {
                    res_repos.push(repo)
                }
                println!("Read page {}.", p)
            }
            Err(_e) => {
                println!("Repo read end. count:{}.", res_repos.len());
                return res_repos;
            }
        }
    }
    return res_repos;
}

pub fn clone_all(user: String, repos: &Vec<Repo>) {
    for repo in repos {
        let cmd = format!("git clone https://github.com/{}/{}", user, repo.name);
        println!("Cloning Repo: {} , size {}", repo.name, repo.size);
        let child = process::Command::new("bash")
            .arg("-c")
            .arg(&cmd)
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap();
        let _res = child.wait_with_output();
    }
}

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub size: i64,
}
