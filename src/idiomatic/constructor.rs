use derive_new::new;
use rand::prelude::*;

#[derive(Debug)]
pub enum Role {
    Guest,
    Viewer,
    Creator,
    Admin
}

#[derive(Debug)]
pub struct User {
    id: u32,
    pub username: String,
    pub role: Role,
}

impl User {
    pub fn new(username: String) -> Result<Self, String> {
        // check if username already exists
        if username == "testuser123" {
            return Err("username already exists!".to_owned());
        }

        Ok(Self {
            id: thread_rng().gen_range(0..9999999),
            username,
            role: Role::Creator,
        })
    }
}

impl Default for User {
    fn default() -> Self {
        let id = thread_rng().gen_range(0..9999999);
        Self {
            id,
            username: format!("guest{id}"),
            role: Role::Guest
        }
    }
}

#[derive(Debug, Default, new)]
pub struct Post {
    content: String,
    #[new(value = "vec![\"rusty\".to_owned()]")]
    tags: Vec<String>,
    #[new(default)]
    likes: u32,
}

#[test]
fn test() {
    let user1 = User::new("testuser1234".to_owned())
                        .unwrap_or_default();

    println!("{:?}", user1);

    let post1 = Post::default();

    let post2 = Post::new("example content".to_owned());

    println!("{:?}", post1);
    println!("{:?}", post2);
}
