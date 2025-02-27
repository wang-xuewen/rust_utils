// use std::time::Duration;

// fn main() {
//     // 初始化Map（每60秒全局清理一次）
//     let map = ExpirableMap::new(Duration::from_secs(60));

//     // 插入永不过期的键
//     map.set("config", "value123", None);

//     // 插入10秒后过期的键
//     map.set("session_token", "abc123", Some(Duration::from_secs(10)));

//     // 多线程并发写入
//     let map_arc = Arc::new(map);
//     let handles: Vec<_> = (0..4).map(|i| {
//         let map_clone = map_arc.clone();
//         std::thread::spawn(move || {
//             map_clone.set(i, i * 2, Some(Duration::from_secs(i)));
//         })
//     }).collect();

//     // 等待线程完成
//     for h in handles {
//         h.join().unwrap();
//     }

//     // 读取数据
//     if let Some(val) = map_arc.get(&"config") {
//         println!("Config value: {}", val);
//     }
// }
