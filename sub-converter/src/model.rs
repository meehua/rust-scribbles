use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_yaml_bw::{value, Mapping, Value};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone)]
pub struct SubscriptionProxies {
    content: Value
}

impl SubscriptionProxies {
    pub fn new(yaml:String) -> Self {
        let de = serde_yaml_bw::Deserializer::from_str(&yaml);
        let value = Value::deserialize(de).unwrap();// 咳咳，这里先unwrap了，后续再改
        match value.get(Value::from("proxies")) { 
            Some(v) => {
                // println!("Proxies found: {:?}", v);
                Self { content: v.clone() }
            }
            None => {
                println!("No proxies found in the YAML content");
                Self { content: Value::Null(None) }
            }
        }
    }
    /// 查找匹配的元素下标（位置）
    /// 
    /// key: 被匹配的目标字段（如 "name"）
    /// keyword: 用于匹配的关键词（模糊匹配）
    /// nth 用于指定是第几个匹配项（0 起始）
    pub fn find( &self,key: &str, keyword: &str, nth: usize) -> Option<usize> {
        let data = self.content.clone();
        
        if let Value::Sequence(seq) = &data {
            let matches: Vec<usize> = seq.iter()
                .enumerate()
                .filter(|(_, item)| {
                    item.get(key)
                        .and_then(|v| v.as_str())
                        .map_or(false, |s| s.contains(keyword))
                })
                .map(|(i, _)| i)
                .collect();
            
            println!("Found {} matches for '{}' in key '{}'", matches.len(), keyword, key);
            matches.get(nth).copied()
        } else {
                println!("No sequence found under 'proxies'");
                None
            }
    }
    /// 修改指定位置的某个字段
    pub fn update(mut self, index: usize, key: &str, new_value: &str) -> Self {
        if let Some(seq) = self.content.as_mapping_mut()
            .and_then(|m| m.get_mut(&Value::from("proxies")))
            .and_then(|v| v.as_sequence_mut())
        {
            if let Some(item) = seq.get_mut(index) {
                if let Some(map) = item.as_mapping_mut() {
                    map.insert(Value::from(key), Value::from(new_value));
                }
            }
        }
        self
    }
    /// 删除指定位置的索引项
    pub fn delete(&self, index: usize) -> Self {
        let mut content = self.content.clone();
        if let Some(seq) = content.as_mapping_mut()
            .and_then(|m| m.get_mut(&Value::from("proxies")))
            .and_then(|v| v.as_sequence_mut())
        {
            if index < seq.len() {
                seq.remove(index);
            }
        }
        Self { content: content }
    }
    /// 在指定位置添加新项；如果 index 为 None，则添加到末尾
    pub fn add(mut self, index: Option<usize>, new_item: Mapping) -> Self {
        if let Some(seq) = self.content.as_mapping_mut()
            .and_then(|m| m.get_mut(&Value::from("proxies")))
            .and_then(|v| v.as_sequence_mut())
        {
            let val = Value::Mapping(new_item);
            match index {
                Some(i) if i <= seq.len() => seq.insert(i, val),
                _ => seq.push(val),
            }
        }
        self
    }

    pub fn to_string(&self) -> String {
        serde_yaml_bw::to_string(&self.content).unwrap_or_default()
    }

    pub fn to_string_with_proxies(&self) -> String {
        // 创建一个新的映射
        let mut root_map = serde_yaml_bw::Mapping::new();
        // 将当前内容（代理序列）作为 "proxies" 键的值插入
        root_map.insert(
            serde_yaml_bw::Value::from("proxies"),
            self.content.clone()
        );
        // 序列化整个映射
        serde_yaml_bw::to_string(&root_map).unwrap_or_default()
    }

}


#[derive(Debug, Clone)]
pub struct SubscriptionInfo {
    pub upload: u64,
    pub download: u64,
    pub total: u64,
    pub expire: u64,
}

impl Default for SubscriptionInfo {
    fn default() -> Self {
        Self {
            upload: 0,
            download: 0,
            total: 0,
            expire: 0,
        }
    }
}

impl SubscriptionInfo {
    pub fn from_hashmap(map: HashMap<String, String>) -> Self {
        let mut sub_info = Self::default();
        
        if let Some(upload_str) = map.get("upload") {
            if let Ok(upload) = upload_str.parse() {
                sub_info.upload = upload;
            }
        }
        
        if let Some(download_str) = map.get("download") {
            if let Ok(download) = download_str.parse() {
                sub_info.download = download;
            }
        }
        
        if let Some(total_str) = map.get("total") {
            if let Ok(total) = total_str.parse() {
                sub_info.total = total;
            }
        }

        if let Some(expire_str) = map.get("expire") {
            if let Ok(expire) = expire_str.parse() {
                sub_info.expire = expire;
            }
        }
        
        sub_info
    }

    pub fn get_from_header(header:HeaderMap) -> Self {
        let info_split= match header.get("subscription-userinfo") {
        Some(v) => {
            // println!("Subscription Info: {}", v.to_str().unwrap_or("Invalid UTF-8"));
            Some(v.to_str().unwrap_or("").split(';').collect::<Vec<&str>>())
        },
            None => {
                println!("No subscription-userinfo header found");
                None
            }
        };

        let mut data_map = HashMap::new();
        if let Some(info_split) = info_split {
            for info in info_split {
                let parts: Vec<&str> = info.split('=').collect();
                if parts.len() == 2 {
                    data_map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
        }

        SubscriptionInfo::from_hashmap(data_map)
    }

    pub fn sum_info(mut info1:Self, mut info2:SubscriptionInfo) -> Self {
        info1.upload += info2.upload;
        info1.download += info2.download;
        info1.total += info2.total;
        if info1.expire < info2.expire {
            info1.expire = info2.expire;
        } 
        info1
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml_bw::Value;

    #[test]
    fn test_proxy_list_ops() {
        let yaml_data = r#"
proxies:
  - name: Node A
    server: a.example.com
    port: 1000
  - name: Node B
    server: b.example.com
    port: 2000
"#;
        let list = SubscriptionProxies::new(yaml_data.to_string());

        // ✅ 1. 查找 “Node B”
        let idx = list.find("name", "Node B", 0).unwrap();
        assert_eq!(idx, 1);


        // ✅ 2. 更新 Node B 的 server
        let updated = list.clone().update(idx, "server", "new-b.example.com");

        let yaml_out = updated.to_string();
        assert!(yaml_out.contains("new-b.example.com"));

        // ✅ 3. 插入新节点到开头
        let mut new_map = Mapping::new();
        new_map.insert(Value::from("name"), Value::from("Node 0"));
        new_map.insert(Value::from("server"), Value::from("zero.example.com"));
        new_map.insert(Value::from("port"), Value::from(999));

        let inserted = updated.clone().add(Some(0), new_map);
        let yaml_out = inserted.to_string();
        assert!(yaml_out.contains("Node 0"));

        // ✅ 4. 删除 Node A (原 index 1，现在是 2)
        let deleted = inserted.delete(2);
        let yaml_out = deleted.to_string();
        assert!(!yaml_out.contains("a.example.com"));

        println!("✅ 最终 YAML:\n{}", deleted.to_string());
    }
}