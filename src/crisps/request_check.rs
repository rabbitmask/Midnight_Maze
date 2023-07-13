use reqwest::StatusCode;
use tokio::runtime::Runtime;

pub(crate) fn request_check(url: &str) {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        match reqwest::get(url).await {
            Ok(response) => {
                let status = response.status();
                // println!("请求成功,响应状态码: {:?}", status);
            }
            Err(err) => {
                // eprintln!("发送请求时发生错误: {}", err);
            }
        }
    });
}
