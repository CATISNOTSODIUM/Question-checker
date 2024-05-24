use std::fs::File;
use std::io::Read;
use crate::server::format::{
    response_type::{MyResponse}
};

//get answer list (as Vec<MyResponse>)
pub async fn fetch_answer(file_path:& String) -> Result<Vec<MyResponse>, Box<dyn std::error::Error>> {
    let root = project_root::get_project_root()?;
    let target_dir = root.join("assets/materials").join(file_path.as_str());
    let mut file = File::open(target_dir)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let answers_list : Vec<MyResponse> = serde_json::from_str(data.as_str())?;
    
    Ok(answers_list)
}