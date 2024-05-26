use async_graphql::{Object,Result,Context};

use markdown_parser::read_file;


#[derive(Default)]
pub struct FetchContent;

#[Object]
impl FetchContent {
    
    pub async fn fetch_content(&self, _ctx: &Context<'_>, file_path: String) -> Result<String>
    { 
        // Read file
        let root = project_root::get_project_root().unwrap();
        let target_dir = root.join("assets/materials").join(file_path.as_str());
        // Parse markdown file to string
        let md = read_file(target_dir.as_os_str())?;
        let content = md.content();
        Ok(format!("{}",content.clone()))
    }
}