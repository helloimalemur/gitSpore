use std::{process, thread};
use std::io::Read;
use std::process::Stdio;
use std::thread::JoinHandle;
use git2::Repository;
use reqwest::header::HeaderMap;
use serde::*;

#[derive(Deserialize, Debug)]
// struct to match on JSON reponse
pub struct Repo {
    pub(crate) name: String,
    pub(crate) id: i32,
    pub(crate) html_url: String,
}

pub trait RepoText {
    fn get_repo_text(repo: Repo) -> String;
    fn get_repo_id(repo: Repo) -> i32;
}

impl RepoText for Repo {
    fn get_repo_text(repo: Repo) -> String {
        repo.name
    }

    fn get_repo_id(repo: Repo) -> i32 {
        repo.id
    }
}

pub async fn get_repos(_user: &String, auth_key: &String) -> Vec<Repo> {
    // set request url
    let request_url = "https://api.github.com/user/repos?visibility=all".to_string();
    // println!("{}", request_url);

    let auth_header = format!("Bearer {}", auth_key);

    //set headers
    let mut headers: HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("User-Agent: ME"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_bytes(auth_header.as_bytes()).unwrap(),
    );

    // println!("{:?}", headers);

    // create reqwest client object
    let client = match reqwest::Client::builder().default_headers(headers).build() {
        Ok(k) => k,
        Err(_e) => std::process::exit(2),
    };
    // println!("{:?}", client);

    // get response
    let response = match client.get(&request_url).send().await {
        Ok(t) => t,
        Err(_e) => std::process::exit(2),
    };
    // println!("{:?}", response);

    //handle response
    let response_text = match response.text().await {
        Ok(ok) => ok,
        Err(_err) => panic!("error handling response"),
    };
    // println!("{:?}", response_text);

    let repos: Vec<Repo> = match serde_json::from_str(response_text.clone().as_ref()) {
        Ok(r) => r,
        Err(_e) => panic!("{}", response_text),
    };

    // println!("{:?}", repos);
    repos
}

pub fn download_repo(repo_url: String, output_path: String, token: String) -> JoinHandle<()> {
    let repo_name = repo_url.split("/").last().unwrap();
    let final_output_path = format!("{}{}/", output_path, repo_name.clone());

    let git_addr = repo_url.split("://").last().unwrap();

    let git_command = format!("https://oauth2:{}@{}", token, git_addr);
    // println!("{}", git_command);
    let repo_name_bind = repo_name.clone().to_string();
    let handle = thread::spawn(move || {
        // let result = Repository::clone(repo_url.as_str(), final_output_path);
        let mut result_string = String::new();
        let result = process::Command::new("git")
            .arg("clone")
            .arg(git_command)
            .arg(final_output_path)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap().stdout.unwrap().read_to_string(&mut result_string);

        if result.is_ok() {
            println!("SUCCESS: {}", repo_name_bind)
        } else {
            println!("FAILURE: {}", repo_name_bind)
        }
    });
    handle
}
