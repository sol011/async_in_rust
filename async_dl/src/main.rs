use std::{path::{PathBuf}, time};
use reqwest::Response;


async fn download(r: Response, cwd: PathBuf) {
    let url =  r.url().to_string().to_owned();
    let filename = r
        .url()
        .path_segments()
        .and_then(|e| e.last())
        .unwrap_or("temp");
    let f_path = cwd.clone().join(filename); 
    if f_path.exists() {
        println!("{:?} exists already. not downloading file.", f_path);
        return;
    }
    let mut file = std::fs::File::create(f_path.clone()).unwrap();
    let bytes = r.bytes().await;
    if let Result::Err(e) = bytes {
        println!("{:?}", e);
        return;
    }
    let mut content = std::io::Cursor::new(bytes.unwrap());
    if let Result::Err(_e) = std::io::copy(&mut content, &mut file) {}
    println!("downloaded {:?} at {:?}", url, f_path);
}


#[tokio::main]
async fn main() {
    let cwd = std::env::current_dir().unwrap().join("temp");
    // let cwd = PathBuf::from("/home/cybereagle3-1/root_only/temp");
    std::fs::create_dir(cwd.clone())
    .or_else(|e| {
        if let std::io::ErrorKind::AlreadyExists = e.kind() { 
            println!("using existing path {:?}", cwd);
            Ok(())
        } else { Err(e) }
    })    
    .expect("could not create a temp directory");

    let client = reqwest::Client::builder()
        .build().expect("could not build client");
    
    // added an extra 5GB file url to quickly verify async download
    let urls = [
        "https://file-examples-com.github.io/uploads/2020/03/file_example_WEBM_480_900KB.webm",
        "http://speedtest-sgp1.digitalocean.com/5gb.test",
        "https://file-examples-com.github.io/uploads/2020/03/file_example_WEBM_1920_3_7MB.webm",
        "https://file-examples-com.github.io/uploads/2017/04/file_example_MP4_480_1_5MG.mp4",
        "https://file-examples-com.github.io/uploads/2017/11/file_example_OOG_1MG.ogg",
        "https://file-examples-com.github.io/uploads/2020/03/file_example_SVG_30kB.svg",
        "https://file-examples-com.github.io/uploads/2017/02/file_example_JSON_1kb.json",
        "https://file-examples-com.github.io/uploads/2017/02/file_example_CSV_5000.csv",
        "https://file-examples-com.github.io/uploads/2017/10/file-sample_150kB.pdf",
        "https://file-examples-com.github.io/uploads/2017/10/file_example_PNG_500kB.png",
        "https://file-examples-com.github.io/uploads/2020/03/file_example_WEBP_50kB.webp",
        "https://file-examples-com.github.io/uploads/2017/11/file_example_WAV_1MG.wav",
        "https://file-examples-com.github.io/uploads/2017/02/file-sample_1MB.doc",
        "https://file-examples-com.github.io/uploads/2017/08/file_example_PPT_250kB.ppt",
        "https://file-examples-com.github.io/uploads/2017/02/index.html"
    ];
    
    let t1 = time::Instant::now();
    let res = futures::future::join_all(
        urls
        .iter()
        .map(|url| {
            println!("downloading {:?}", url);
            client.get(url.to_string()).send()
        }
    )).await;
    let maybe = res.into_iter()
        .filter_map(|e| e.ok());
    futures::future::join_all(maybe
        .filter(|e| e.status() == reqwest::StatusCode::OK)
        .map(|r| download(r, cwd.clone()))).await;

    println!("finished in {:?}", t1.elapsed());
}
