// https://maxuuell.com/blog/how-to-concatenate-strings-in-rust
// https://docs.github.com/en/rest/guides/basics-of-authentication?apiVersion=2022-11-28
// https://docs.github.com/en/rest?apiVersion=2022-11-28
// https://virtualapi.checkoutchamp.com/leads/import/?loginId=v2devapi&password=v2devapi&campaignId=344&firstName=James&lastName=Koonts&emailAddress=test@me.com
// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
// https://rcos.io/static/internal_docs/reqwest/struct.ClientBuilder.html
use reqwest::header::HeaderMap;
use serde::*;
use serde_json;
use std::{env, process, thread};
use std::io::Read;
use std::time::Duration;
use serde_json::Value;


#[derive(Deserialize, Debug)]
// struct to match on JSON reponse
pub struct Repo {
    name: String,
    id: i32,
    html_url: String,
}

pub trait RepoText {
    fn get_repo_text(repo: Repo) -> String;
}

impl RepoText for Repo {
    fn get_repo_text(repo: Repo) -> String {
        return repo.name
    }

}

pub async fn get_repos(mut user: String, auth_key: String) -> Vec<Repo> {

    // set request url
    let request_url = format!(
        "https://api.github.com/user/repos?visibility=all",
    );
    println!("{}", request_url);

    let auth_header = format!("Bearer {}", auth_key);


    //set headers
    let mut headers: HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("User-Agent: ME"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github+json")
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_bytes(auth_header.as_bytes()).unwrap()
    );

    println!("{:?}", headers);

    // create reqwest client object
    let client = match reqwest::Client::builder().default_headers(headers).build() {
        Ok(k) => k,
        Err(_e) => std::process::exit(2),
    };
    // println!("{:?}", client);

    // get response
    let response = match client.get(&request_url)
        .send().await {
        Ok(t) => t,
        Err(_e) => std::process::exit(2),
    };
    // println!("{:?}", response);

    //handle response
    let response_text = match response.text().await {
        Ok(ok) => ok,
        Err(err) => panic!("error handling response")
    };
    // println!("{:?}", response_text);

    let repos: Vec<Repo> = match serde_json::from_str(response_text.clone().as_ref()) {
        Ok(r) => r,
        Err(e) => panic!("{}", response_text)
    };

    // println!("{:?}", repos);
    return repos;
}
