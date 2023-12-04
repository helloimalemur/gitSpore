extern crate core;

use std::borrow::Borrow;
use std::thread;
use std::time::Duration;

mod get_repos;
use get_repos::get_repos;
use crate::get_repos::{Repo, RepoText};


#[tokio::main]
async fn main() {
    let user = "helloimalemur";
    let mut repos: Vec<Repo> = Vec::new();

    //TODO: params / options

    // print stargazers for each repo, sleeping 2s between repo
    for (int, repo) in get_repos(user).await.iter().enumerate() {
        println!("{:?}", repo);

        tokio::time::sleep(Duration::from_secs(2)).await;

        // TODO: download repo to desired location
    }

}
