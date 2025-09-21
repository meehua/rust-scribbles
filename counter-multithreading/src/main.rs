use std::{collections::HashMap, error::Error, fs, thread, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start =Instant::now();
    let path="../counter/Books/";
    let mut map= std::collections::HashMap::new();
    let files:Vec<_>=fs::read_dir(path)?
        .filter_map(|entry| {  // 自动抛弃none的元素，保留some的元素并对每个进行使用闭包单独处理
            let entry= entry.ok()?;
            let path= entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str())==Some("txt") { //只处理文件且是txt的文件
                Some(path) // 返回some
            } else {
                None
            }
        }).collect();

    let mut handles = vec![];
    const CHUNK_SIZE:usize= 10; // 每个线程处理10个文件
    let chunks = files.chunks(CHUNK_SIZE); // 将文件分块
    for chunk in chunks {
        let mut local_map= HashMap::new(); // 每个线程有自己的map
        let chunk =chunk.to_vec(); // 克隆一份, 因为chunk是引用，而files活的不够长。
        let handle =thread::spawn( move | |{
            chunk.iter().filter_map(|p| fs::read_to_string(p).ok())
            .for_each(|text|{
                text.split_whitespace()
                .for_each(|w|{
                    let word = w
                        .trim_matches(|c: char| c.is_ascii_punctuation())
                        .to_lowercase();
                    if !word.is_empty()  {
                        *local_map.entry(word).or_insert(0) += 1;
                    }
                });
            });
            local_map
        });
        handles.push(handle);
    }

    for h in handles{
        let local_map = h.join().unwrap();
        for(k,v) in local_map {
            *map.entry(k).or_insert(0) += v;    // 将线程的局部map并到全局map
        }
    }
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

