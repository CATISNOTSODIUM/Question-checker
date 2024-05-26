use axum::{body::Body, http::header, response::IntoResponse};
use async_graphql::{Object,Result,Context};
use tokio_util::io::ReaderStream;

use markdown_parser::{
    read_file, Error
};


#[derive(Default)]
pub struct FetchContent;

#[Object]
impl FetchContent {
    
    pub async fn fetch_content(&self, ctx: &Context<'_>, file_path: String) -> Result<String>
    { 
        let root = project_root::get_project_root().unwrap();
        let target_dir = root.join("assets/materials").join(file_path.as_str());
        let md = read_file(target_dir.as_os_str())?;
        let content = md.content();
        Ok(format!("{}",content.clone()))

    }
}