use serde::{Serialize, Deserialize};

use std::collections::{BTreeMap, HashMap};
use std::{fs,io};

use reqwest;

use reqwest::{header::USER_AGENT};

mod model;
use crate::model::{SubscriptionProxies,SubscriptionInfo};
use serde_yaml_bw::Value;

/// 注意：本程序生成的 YAML 文件还是无法使用的，这只是个半成品。
/// 因为本人对 YAML 语法和 Clash 配置文件格式都不太熟悉，问题无法解决。
fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let client =  reqwest::blocking::Client::new();
    let file = client.get("https://你的订阅连接")
    .header(USER_AGENT, "verge-clash/v2.4.2")
    .send()?;
    let header = file.headers().clone();
    let body = file.text()?;
    // println!("Content fetched: {}", &body[..4000]); 
    
    let proxies_parsed = SubscriptionProxies::new(body);
    // println!("{:?}", proxies_parsed );
    if let Some(index) =  proxies_parsed.find("name", "官网", 0){
        proxies_parsed.delete(index);
    }
    
    if let Ok(template_file) = fs::read_to_string("sub-template.yaml"){
        let template = template_file.replacen("proxies:", &proxies_parsed.to_string_with_proxies(), 1);
        // println!("{}", proxies_parsed.to_string_with_proxies());
        match fs::write("output.yaml", template) {
            Err(e) => println!("Failed to write to output.yaml: {}", e),
            Ok(_) => println!("Successfully wrote to output.yaml"),   
        }
    }
    
    Ok(())
}
