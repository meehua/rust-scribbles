use std::{error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start =Instant::now();
    let path="./Books";
    let mut map= std::collections::HashMap::new();
    fs::read_dir(path)?
        .filter_map(|entry| {  // 自动抛弃none的元素，保留some的元素并对每个进行使用闭包单独处理
            let entry= entry.ok()?;
            let path= entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str())==Some("txt") { //只处理文件且是txt的文件
                Some(path) // 返回some
            } else {
                None
            }
        })
        .for_each(|p|{ // 对每个文件进行处理
            if let Ok(text)=fs::read_to_string(p){ // 读取文件内容
                text.split_whitespace() // 按空白符分割
                .for_each(|w|{ // 对每个单词进行处理
                    let word = w
                        .trim_matches(|c: char| c.is_ascii_punctuation()) // 对每个字符做判断，去掉标点符号
                        .to_lowercase(); // 转为小写
                    if !word.is_empty()  { // 如果单词不为空
                        *map.entry(word).or_insert(0) += 1; // 计数加一
                    }
                });
            }
        });

    println!("Map count:{}", map.len()); // 打印不同单词的数量，即map的长度

    let mut vec:Vec<_>= map.iter().collect(); // 转为可变数组
    vec.sort_by(|a,b| b.1.cmp(a.1)); // 按值排序，降序

    for i in 0..10 {
        println!("{}: {}", vec.get(i).unwrap().0, vec.get(i).unwrap().1);
    }

    let elapsed = start.elapsed();
    println!("Time elapsed: {}", elapsed.as_millis());

    Ok(())


}

