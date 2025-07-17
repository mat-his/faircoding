#![allow(clippy::all, warnings)]
pub struct RepoId;
pub mod repo_id {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RepoId";
    pub const QUERY : & str = "query RepoId($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    id\n    owner {\n      __typename\n      login\n    }\n    name\n    latestRelease\n    url\n    parent {\n      name\n      owner {\n        __typename\n        login\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type URI = super::URI;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub name: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub repository: Option<RepoIdRepository>,
    }
    #[derive(Deserialize)]
    pub struct RepoIdRepository {
        pub id: ID,
        pub owner: RepoIdRepositoryOwner,
        pub name: String,
        #[serde(rename = "latestRelease")]
        pub latest_release: Option<RepoIdRepositoryLatestRelease>,
        pub url: URI,
        pub parent: Option<RepoIdRepositoryParent>,
    }
    #[derive(Deserialize)]
    pub struct RepoIdRepositoryOwner {
        pub login: String,
        #[serde(flatten)]
        pub on: RepoIdRepositoryOwnerOn,
    }
    #[derive(Deserialize)]
    #[serde(tag = "__typename")]
    pub enum RepoIdRepositoryOwnerOn {
        Organization,
        User,
    }
    #[derive(Deserialize)]
    #[serde(tag = "__typename")]
    pub enum RepoIdRepositoryLatestRelease {}
    #[derive(Deserialize)]
    pub struct RepoIdRepositoryParent {
        pub name: String,
        pub owner: RepoIdRepositoryParentOwner,
    }
    #[derive(Deserialize)]
    pub struct RepoIdRepositoryParentOwner {
        pub login: String,
        #[serde(flatten)]
        pub on: RepoIdRepositoryParentOwnerOn,
    }
    #[derive(Deserialize)]
    #[serde(tag = "__typename")]
    pub enum RepoIdRepositoryParentOwnerOn {
        Organization,
        User,
    }
}
impl graphql_client::GraphQLQuery for RepoId {
    type Variables = repo_id::Variables;
    type ResponseData = repo_id::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: repo_id::QUERY,
            operation_name: repo_id::OPERATION_NAME,
        }
    }
}
