use axum::{response::IntoResponse,http::StatusCode};
use std::fs::File;
use std::io::Read;


pub async fn send_question()-> impl IntoResponse {
    //get the root directory of this project
    let root = project_root::get_project_root().unwrap();
    
    //read file .json
    let file_name = "problem.json".to_string();
    let target_dir = root.join("assets/questions").join(file_name.as_str());
    let mut file = File::open(target_dir).expect("Cannot open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Invalid JSON");

    (StatusCode::OK, data.to_owned())
}