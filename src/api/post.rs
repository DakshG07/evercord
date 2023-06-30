use crate::api::Instance;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub async fn list_posts(instance: Instance) -> Result<Posts> {
    let body = reqwest::get(format!("{}/api/v3/post/list?sort=Active", instance))
        .await?
        .text()
        .await?;
    let posts: Posts = serde_json::from_str(body.as_str())?;
    Ok(posts)
}

type RawDate = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Posts {
    pub posts: Vec<PostData>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PostData {
    pub post: Post,
    pub creator: User,
    pub community: Community,
    pub counts: Counts,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub body: Option<String>,
    pub creator_id: u32,
    pub community_id: u32,
    pub published: RawDate,
    pub deleted: bool,
    pub nsfw: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub banned: bool,
    pub published: RawDate,
    pub actor_id: Option<String>,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub admin: bool,
    pub bot_account: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Community {
    pub id: u32,
    pub name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub published: RawDate,
    pub deleted: bool,
    pub nsfw: bool,
    pub actor_id: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Counts {
    pub id: u32,
    pub post_id: u32,
    pub comments: u32,
    pub score: u32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub published: RawDate,
    pub newest_comment_time: RawDate,
}
