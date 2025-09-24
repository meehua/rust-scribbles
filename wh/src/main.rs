use std::{sync::atomic::AtomicUsize, thread, time::Instant};
use rand::Rng;

fn main() {
    let start = Instant::now();
    let section_count=rand::rng().random_range(10..=20); // 使用随机数生成库房区域数量用于模拟使用
    let mut sections=Vec::new();    // 用于存储区域的盘点数据
    let mut actual = [0;5]; // 用于存储实际盘点数据
    for _ in 0..section_count {
        let mut section=Section([0;5]);
        for (i,p) in section.0.iter_mut().enumerate(){  // 遍历区域内的货架
            *p=rand::rng().random_range(0..=1000000); // 使用随机数生成库房货架的盘点数据用于模拟使用
            actual[i]+=*p; // 累加实际盘点数据
        }
        sections.push(section); // 将区域数据添加到区域列表中
    }
    println!("Actual: {:?}", actual); // 输出实际盘点数据，用于对照

    let counted:[AtomicUsize;5] = Default::default(); // 用于存储盘点结果

    thread::scope(|s|{
        for sec in sections.iter() { // 为每个区域（库房）创建一个线程
            s.spawn(||{
                for (i,c) in sec.0.iter().enumerate() { 
                    for _ in 0..*c { // 模拟盘点货架上的每一件商品
                        counted[i].fetch_add(1, std::sync::atomic::Ordering::Relaxed); // 使用原子操作累加盘点结果
                    }
                }
            });
        }
    });

    println!("Counted: {counted:?}"); // 输出盘点结果

    for i in 0..5 {
        assert_eq!(actual[i], counted[i].load(std::sync::atomic::Ordering::Relaxed)); // 验证盘点结果是否与实际数据一致
    }

    let elapsed = start.elapsed();
    println!("Elapsed time:{}", elapsed.as_micros());
}

struct Section ([usize;5]); // 元组结构体，定义库房区域结构体，包含5个货架的盘点数据
