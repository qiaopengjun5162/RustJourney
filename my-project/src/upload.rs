use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result}; 
use futures::StreamExt; 
use rand::distributions::Alphanumeric; 
use rand::{thread_rng, Rng}; 
use std::fs::{create_dir_all, File}; 
use std::io::Write; 
use std::path::{Path, PathBuf}; 

async fn upload(mut payload: web::Payload, req: HttpRequest) -> Result<HttpResponse> { // 检查上传目录是否存在，不存在则创建 
    let upload_dir = "./uploads"; 
    if !Path::new(upload_dir).exists() { 
        create_dir_all(upload_dir)?; 
    } 
    // 生成一个随机文件名 
    let filename: String = thread_rng() .sample_iter(&Alphanumeric) .take(16) .map(char::from) .collect(); 
    let extension = Path::new(req.headers().get("content-type").unwrap().to_str().unwrap()) .extension() .unwrap() .to_str() .unwrap(); 
    let filepath = format!("{}/{}.{}", upload_dir, filename, extension); 
    let mut file = File::create(&filepath)?; 
    // 将上传的文件写入磁盘 
    while let Some(chunk) = payload.next().await { 
        let data = chunk?; file.write_all(&data)?; 
    } Ok(
        HttpResponse::Ok().body(format!("Uploaded: {}", filepath))
    ) 
} 

#[actix_web::main] 
async fn main() -> std::io::Result<()> { 
    HttpServer::new(|| App::new().route("/upload", web::post().to(upload))).bind("127.0.0.1:8080")?.run().await 
} 

// 上面的API接口运行在本地的8080端口，用户可以通过 POST 请求将文件上传到接口。
// 文件将被保存到 uploads 目录中，文件名由16个随机字母数字字符组成。在上传完成后，API接口将返回上传的文件路径
