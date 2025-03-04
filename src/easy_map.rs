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
    /// cleanup_interval：启动清理过期key的间隔，如果小于1分钟，则会默认设置为1分钟
    /// 这个值应当适当设置大一些，用于一些很久没使用的但过期了的key的清楚，释放内存
    /// eg. let map = EasyMap::new(Duration::from_secs(60));
    pub fn new(cleanup_interval: Duration) -> Self {
        // let cleanup_interval = if cleanup_interval < Duration::from_secs(60) {
        //     Duration::from_secs(60) // 默认值 1 分钟
        // } else {
        //     cleanup_interval
        // };

        // 最小值 1 分钟
        let cleanup_interval = cleanup_interval.max(Duration::from_secs(60));

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
                // 使用下划线前缀表示该变量在闭包逻辑中未被使用（避免编译器警告
                // 返回true的条目保留，否则被移除
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
                // 这里必须先drop之后（释放读锁）再remove，
                // 否则会一直阻塞。
                drop(entry);
                self.data.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        })
    }

    /// 判断key是否存在
    pub fn is_exist(&self, key: &K) -> bool
    where
        V: Clone,
    {
        return self.data.contains_key(key);
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
