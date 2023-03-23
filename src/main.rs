use base64::Engine;
use tokio::time::{sleep, Duration};


mod xml;
mod get_cgv_info;
use crate::get_cgv_info::get_cgv_info;

// #[warn(non_snake_case)]

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {

  let mut rest_cnt = 0;

  //test();

  // loop {
  //   sleep(Duration::from_secs(1)).await;

  //   let res = get_cgv_info().await;
  //   println!("{}", res);
  //   let data = parser(res);
  //   let len = {
  //     let mut cnt = 0;
  //     for i in 0..data.len() {
  //       if data[i].useable == "Y" && data[i].y != "A" {
  //         // println!("{:?}", data[i]);
  //         cnt = cnt + 1;
  //       }
  //     }
  //     cnt
  //   };
  //   if rest_cnt != len {
  //     println!("좌석 수가 {}개에서 {}개로 변경되었습니다.", rest_cnt, len);
  //     let msg = format!("좌석 수가 {}개에서 {}개로 변경되었습니다.", rest_cnt, len);
  //     //alert(msg).await;
  //     rest_cnt = len;
  //   }
  // }
}

async fn alert(msg: std::string::String) {
  let client = reqwest::Client::new();
  const URI: &str = "http://ntfy.sh/3EB7CB82-A795-497F-8A6F-C004CFFB1D5C";
  client.post(URI)
  .body(msg.to_owned())
  .send()
  .await;
}