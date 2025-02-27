use dashmap::DashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};
struct Entry<T> {
    value: T,
    expires_at: Option<SystemTime>,
}

pub struct EasyMap<K, V>
where
    K: Eq + std::hash::Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    data: Arc<DashMap<K, Entry<V>>>,
    stop_flag: Arc<AtomicBool>,
}
impl<K, V> EasyMap<K, V>
where
    K: Eq + std::hash::Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    /// 创建新Map并启动后台清理线程
    pub fn new(cleanup_interval: Duration) -> Self {
        let data = Arc::new(DashMap::new());
        let data_clone = data.clone();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_clone = stop_flag.clone();

        // 后台清理线程
        thread::spawn(move || {
            while !stop_clone.load(Ordering::Relaxed) {
                thread::sleep(cleanup_interval);
                let now = SystemTime::now();

                // DashMap 支持多线程并发操作（分段锁机制）‌
                data_clone.retain(|_k, entry: &mut Entry<V>| {
                    // 显式标注 entry 类型
                    entry.expires_at.map_or(true, |t| now < t)
                });
            }
        });

        Self { data, stop_flag }
    }

    /// 插入键值对（支持设置TTL）
    pub fn set(&self, key: K, value: V, ttl: Option<Duration>) {
        let expires_at = ttl.map(|d| SystemTime::now() + d);
        self.data.insert(key, Entry { value, expires_at });
    }

    /// 获取值（惰性删除过期键）
    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.data.get(key).and_then(|entry| {
            if entry.expires_at.map_or(false, |t| SystemTime::now() >= t) {
                self.data.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        })
    }
}

impl<K, V> Drop for EasyMap<K, V>
where
    K: Eq + std::hash::Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}
