use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    RES_CD: String,
    RES_MSG: String,
    DATA: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseBody {
    d: ResponseData,
}

pub async fn get_cgv_info() -> String {
  const URI: &str = "http://ticket.cgv.co.kr/CGV2011/RIA/CJ000.aspx/CJ_TICKET_002_PRIME_ZONE_LANGUAGE";
  let mut req_body = HashMap::new();
  req_body.insert("REQSITE", "x02PG4EcdFrHKluSEQQh4A==");
  req_body.insert("Language", "zqWM417GS6dxQ7CIf65+iA==");
  req_body.insert("TheaterCd", "LMP+XuzWskJLFG41YQ7HGA==");
  req_body.insert("PlayYMD", "7/GCKiASBqGsIJWKmBujUA==");
  req_body.insert("ScreenCd", "puE6q/PuILVnVlbgI8uHnA==");
  req_body.insert("PlayNum", "hlrIVsrgDYMr7PQdmwAA4w==");

  let client = reqwest::Client::new();
  let res = client.post(URI)
        .json(&req_body)
        .send()
        .await
        .unwrap();
  match res.status() {
    reqwest::StatusCode::OK => {
      match res.json::<ResponseBody>().await {
        Ok(parsed) => parsed.d.DATA,
        Err(_) => panic!("파싱 에러! JSON 구조를 확인해주세요"),
      }
    },
    other => {
        panic!("통신 에러\n{:?}", other);
    }
  }
}