//! 计算线程池 - 声学仿真等 CPU 密集型计算的异步执行层
//!
//! 使用 tokio 的阻塞线程池处理 CPU 密集型计算任务，
//! 避免阻塞异步运行时的事件循环。

use std::sync::Arc;
use tokio::task::JoinHandle;

/// 计算线程池配置
#[derive(Debug, Clone)]
pub struct ComputePoolConfig {
    pub max_threads: usize,
    pub thread_name: String,
    pub stack_size: Option<usize>,
}

impl Default for ComputePoolConfig {
    fn default() -> Self {
        let cpu_cores = num_cpus::get();
        Self {
            max_threads: cpu_cores.max(2).min(8),
            thread_name: "compute-pool".to_string(),
            stack_size: Some(2 * 1024 * 1024),
        }
    }
}

/// 计算线程池句柄
#[derive(Clone)]
pub struct ComputePool {
    config: Arc<ComputePoolConfig>,
}

impl ComputePool {
    pub fn new() -> Self {
        Self::with_config(ComputePoolConfig::default())
    }

    pub fn with_config(config: ComputePoolConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn config(&self) -> &ComputePoolConfig {
        &self.config
    }

    /// 在计算线程池中执行一个 CPU 密集型任务
    ///
    /// # 参数
    /// - `f`: 要执行的闭包，必须是 `'static` 因为任务可能在线程池中挂起
    ///
    /// # 返回
    /// - `JoinHandle<T>`: 可 await 的任务句柄
    pub fn spawn<F, T>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        tokio::task::spawn_blocking(f)
    }

    /// 执行合金分析计算
    pub async fn run_alloy_analysis<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(f)
            .await
            .map_err(|e| format!("alloy analysis task failed: {}", e))
    }

    /// 执行工艺对比计算
    pub async fn run_process_compare<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(f)
            .await
            .map_err(|e| format!("process comparison task failed: {}", e))
    }

    /// 执行钟楼声学计算（最重的计算任务）
    pub async fn run_tower_acoustics<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(f)
            .await
            .map_err(|e| format!("tower acoustics task failed: {}", e))
    }

    /// 执行虚拟敲钟计算
    pub async fn run_vr_strike<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(f)
            .await
            .map_err(|e| format!("vr strike task failed: {}", e))
    }

    /// 通用声学仿真计算
    pub async fn run_acoustic_sim<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(f)
            .await
            .map_err(|e| format!("acoustic simulation task failed: {}", e))
    }
}

impl Default for ComputePool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compute_pool_basic() {
        let pool = ComputePool::new();
        let result = pool.spawn(|| 42).await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_compute_pool_run_alloy() {
        let pool = ComputePool::new();
        let result = pool.run_alloy_analysis(|| "test_result".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_result");
    }

    #[tokio::test]
    async fn test_compute_pool_config() {
        let config = ComputePoolConfig::default();
        assert!(config.max_threads >= 2);
        assert!(config.max_threads <= 8);
        assert_eq!(config.thread_name, "compute-pool");
    }

    #[tokio::test]
    async fn test_compute_pool_clone() {
        let pool = ComputePool::new();
        let pool2 = pool.clone();
        let r1 = pool.spawn(|| 1).await.unwrap();
        let r2 = pool2.spawn(|| 2).await.unwrap();
        assert_eq!(r1 + r2, 3);
    }
}
