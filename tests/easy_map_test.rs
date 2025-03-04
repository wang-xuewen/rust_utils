// 引用 lib.rs 中导出的 easy_map 模块
use rust_utils::easy_map::EasyMap;
use std::time::Duration;

#[test]
fn test_set_and_get() {
    let map = EasyMap::new(Duration::from_secs(60));

    // 设置一个值，永不过期
    map.set("key1".to_string(), "value1".to_string(), None);

    // 获取值
    assert_eq!(map.get(&"key1".to_string()), Some("value1".to_string()));
}

#[test]
fn test_expired_entry() {
    let map = EasyMap::new(Duration::from_secs(60));

    // 设置一个 1 秒后过期的值
    map.set(
        "key2".to_string(),
        "value2".to_string(),
        Some(Duration::from_secs(1)),
    );

    // 立即获取值，应该存在
    assert_eq!(map.get(&"key2".to_string()), Some("value2".to_string()));

    // 等待 2 秒后再次获取值，应该过期
    std::thread::sleep(Duration::from_secs(2));

    assert_eq!(map.get(&"key2".to_string()), None);
}

#[test]
fn test_non_existent_key() {
    let map: EasyMap<String, String> = EasyMap::new(Duration::from_secs(60));

    // map.set("key2".to_string(), "value2".to_string(), None);

    // 获取不存在的键
    assert_eq!(map.get(&"non_existent".to_string()), None);
}

#[test]
fn test_background_cleanup() {
    let map = EasyMap::new(Duration::from_secs(1));

    // 设置一个不过期的值
    map.set(
        "key2".to_string(),
        "value2".to_string(),
        Some(Duration::from_secs(1)),
    );

    // 立即获取值，应该存在
    assert_eq!(map.get(&"key2".to_string()), Some("value2".to_string()));

    // 等待 2 秒后再次获取值，应该过期
    std::thread::sleep(Duration::from_secs(3));

    assert_eq!(map.get(&"key2".to_string()), None);
}
