use serde::Serialize;

#[derive(Serialize)]
pub struct Webhook {
    pub username: String,
    pub content: Option<String>,
    pub embeds: Vec<Embed>,
}

#[derive(Serialize)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub url: String,
    pub color: i64,
    pub footer: Footer,
    pub fields: Vec<Field>,
}

#[derive(Serialize)]
pub struct Footer {
    pub text: String,
}

#[derive(Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub inline: bool,
}
