use tokio_util::io::ReaderStream;
use axum::{
    body::Body, http::header, response::IntoResponse 
};



pub async fn read_markdown(file_path: String) -> impl IntoResponse {
    //Open `File`  implemented by async read
    let root = project_root::get_project_root().unwrap();
    let target_dir = root.join("assets/materials").join(file_path.as_str());
    let file = tokio::fs::File::open(target_dir.clone()).await.unwrap();
    
    let stream = ReaderStream::new(file);

    //define body from data stream
    let body = Body::from_stream(stream);

    //define header
    let content_type = match mime_guess::from_path(file_path.clone()).first_raw(){
        Some(x) => x,
        None => "text/plain",
    };

 

    let headers = [
        (header::CONTENT_TYPE, content_type),
        (header::CONTENT_DISPOSITION,
        &format!("inline; filename=\" {} \"", file_path.clone()))
    ];

    //send data
    (headers, body).into_response()

}