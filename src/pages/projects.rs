use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Serialize, Deserialize, Debug)]
struct Project{
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSON{
    data: Vec<Project>,
}

fn update_page() -> Vec<Project>{
    let mut out = vec![];

    for i in 0..10{
        out.push(Project{title:format!("{}", i)});
    }
    out
}

pub fn get_context() -> Context{
    let mut ctx = Context::new();
    ctx.insert("projects", &update_page());
    ctx
}