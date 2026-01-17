use crate::error::{Result, TruthlinkedError};
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration with exponential backoff
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier (typically 2.0)
    pub backoff_multiplier: f64,
    /// Random jitter factor (0.0 to 1.0)
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
}

impl RetryConfig {
    /// Create retry config for production use
    pub fn production() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
    
    /// Create retry config for aggressive retries
    pub fn aggressive() -> Self {
        Self {
            max_attempts: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 1.5,
            jitter_factor: 0.2,
        }
    }
    
    /// Create retry config with no retries
    pub fn none() -> Self {
        Self {
            max_attempts: 1,
            initial_delay: Duration::from_secs(0),
            max_delay: Duration::from_secs(0),
            backoff_multiplier: 1.0,
            jitter_factor: 0.0,
        }
    }
}

/// Retry executor with exponential backoff and jitter
pub struct RetryExecutor {
    config: RetryConfig,
}

impl RetryExecutor {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }
    
    /// Execute operation with retries
    pub async fn execute<F, Fut, T>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;
        
        for attempt in 0..self.config.max_attempts {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Don't retry certain errors
                    if !self.should_retry(&e) {
                        return Err(e);
                    }
                    
                    last_error = Some(e);
                    
                    // Don't sleep after the last attempt
                    if attempt + 1 < self.config.max_attempts {
                        let delay = self.calculate_delay(attempt);
                        sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or(TruthlinkedError::Network("Max retries exceeded".to_string())))
    }
    
    /// Determine if error should be retried
    fn should_retry(&self, error: &TruthlinkedError) -> bool {
        match error {
            // Retry network errors
            TruthlinkedError::Network(_) => true,
            // Retry server errors
            TruthlinkedError::ServerError => true,
            // Don't retry auth errors
            TruthlinkedError::Unauthorized => false,
            TruthlinkedError::Forbidden => false,
            // Don't retry client errors
            TruthlinkedError::InvalidRequest(_) => false,
            // Don't retry rate limits (handle separately)
            TruthlinkedError::RateLimitExceeded(_) => false,
            // Don't retry other errors
            _ => false,
        }
    }
    
    /// Calculate delay with exponential backoff and jitter
    fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.config.initial_delay.as_millis() as f64;
        let exponential_delay = base_delay * self.config.backoff_multiplier.powi(attempt as i32);
        let capped_delay = exponential_delay.min(self.config.max_delay.as_millis() as f64);
        
        // Add jitter to prevent thundering herd
        let jitter = if self.config.jitter_factor > 0.0 {
            use rand::Rng;
            let jitter_amount = capped_delay * self.config.jitter_factor;
            
            rand::thread_rng().gen_range(-jitter_amount..=jitter_amount)
        } else {
            0.0
        };
        
        let final_delay = (capped_delay + jitter).max(0.0) as u64;
        Duration::from_millis(final_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_retry_success_on_second_attempt() {
        let config = RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(10),
            backoff_multiplier: 2.0,
            jitter_factor: 0.0,
        };
        
        let executor = RetryExecutor::new(config);
        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();
        
        let result: Result<&str> = executor.execute(|| {
            let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                if count == 0 {
                    Err(TruthlinkedError::Network("Connection failed".to_string()))
                } else {
                    Ok("success")
                }
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count.load(Ordering::SeqCst), 2);
    }
    
    #[tokio::test]
    async fn test_no_retry_on_auth_error() {
        let config = RetryConfig::none();
        let executor = RetryExecutor::new(config);
        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();
        
        let result: Result<&str> = executor.execute(|| {
            attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                Err(TruthlinkedError::Unauthorized)
            }
        }).await;
        
        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
    }
}
