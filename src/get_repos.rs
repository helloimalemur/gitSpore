use anyhow::Error;
use reqwest::header::HeaderMap;
use serde::*;
use std::io::Read;
use std::process::Stdio;
use std::thread::JoinHandle;
use std::{process, thread};
use std::time::Duration;

#[derive(Deserialize, Debug)]
// struct to match on JSON response
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

pub async fn get_repos(user: &str, auth_key: &str) -> Result<Vec<Repo>, Error> {
    #[allow(unused)]
    let mut gitsporest_url = String::new();
    // set gitsporest url

    if auth_key.eq_ignore_ascii_case("none") {
        println!("{}", "public!");
        gitsporest_url = format!("https://api.github.com/users/{}/repos?visibility=all", user);
    } else {
        gitsporest_url = "https://api.github.com/user/repos?visibility=all".to_string();
    }
    println!("{}", gitsporest_url);
    println!("{}", auth_key);

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

    if !auth_key.eq_ignore_ascii_case("none") {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_bytes(auth_header.as_bytes())?,
        );
    }

    // println!("{:?}", headers);

    let mut pagination: bool = true;
    #[allow(unused)]
    let mut git_url = String::new();
    let mut repos: Vec<Repo> = vec![];
    let mut page: i32 = 1;
    #[allow(unused)]
    let mut check_header = String::new();
    git_url = String::from(&gitsporest_url);

    while pagination {
        // create reqwest client object
        let client = match reqwest::Client::builder()
            .default_headers(headers.clone())
            .build()
        {
            Ok(k) => k,
            Err(_e) => std::process::exit(2),
        };
        // println!("{:?}", client);

        // get response
        let response = match client.get(&git_url).send().await {
            Ok(t) => t,
            Err(_e) => std::process::exit(2),
        };
        // println!("{:?}", response);

        if let Some(header) = response.headers().get("link") {
            if let Ok(h) = header.to_str() {
                if h.contains("next") && !h.contains("first")  {
                    page += 1;
                } else {
                    pagination = false;
                }
            } else {
                pagination = false;
            }
            let page_param = format!("&page={}", page);
            git_url = format!("{gitsporest_url}{page_param}");
        } else {
            pagination = false
        }

        //handle response
        let response_text = match response.text().await {
            Ok(ok) => ok,
            Err(_err) => panic!("error handling response"),
        };
        // println!("{:?}", response_text);

        let new_repos: Vec<Repo> = match serde_json::from_str(response_text.clone().as_ref()) {
            Ok(r) => r,
            Err(_e) => panic!("{}", response_text),
        };

        for entry in new_repos {
            println!("{}", entry.name);
            repos.push(entry);
        }
        if auth_key.eq_ignore_ascii_case("none") {
            tokio::time::sleep(Duration::from_millis(3000)).await;
        }
    }
    // println!("{:?}", repos);
    Ok(repos)
}

pub fn download_repo(
    repo_url: String,
    repo_name: String,
    final_output_path: String,
    token: String,
) -> JoinHandle<()> {
    println!("Downloading: {:?}", final_output_path);
    let git_addr = repo_url.split("://").last().unwrap();

    let git_command = format!("https://oauth2:{}@{}", token, git_addr);
    // println!("{}", git_command);
    let _repo_name_bind = repo_name.to_string();
    let handle = thread::spawn(move || {
        // let result = Repository::clone(repo_url.as_str(), final_output_path);
        let mut result_string = String::new();
        let result = process::Command::new("git")
            .arg("clone")
            .arg("--quiet")
            .arg(git_command)
            .arg(final_output_path.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // .spawn();
            .spawn()
            .unwrap();

        let error = result
            .stderr
            .unwrap()
            .read_to_string(&mut result_string)
            .unwrap();
        let _out = result
            .stdout
            .unwrap()
            .read_to_string(&mut result_string)
            .unwrap();

        if error > 0 {
            println!("FAILURE: {:?}", result_string)
        } else {
            println!("SUCCESS: {:?}", final_output_path)
        }
    });
    handle
}

pub fn update_repo(repo_path: String) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut result_string = String::new();
        let result = process::Command::new("git")
            .arg("-C")
            .arg(repo_path.clone())
            .arg("pull")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // .spawn();
            .spawn()
            .unwrap();

        let error = result
            .stderr
            .unwrap()
            .read_to_string(&mut result_string)
            .unwrap();
        let _out = result
            .stdout
            .unwrap()
            .read_to_string(&mut result_string)
            .unwrap();

        if error > 0 {
            println!("UPDATE SUCCESS: {}", repo_path)
        } else {
            println!("NO CHANGE: {:?}", repo_path)
        }
    });
    handle
}
