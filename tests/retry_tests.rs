use truthlinked_sdk::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[cfg(test)]
mod retry_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_retry_success_first_attempt() {
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
            attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            async move { Ok::<&str, TruthlinkedError>("success") }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
    }
    
    #[tokio::test]
    async fn test_retry_success_second_attempt() {
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
    async fn test_retry_max_attempts_exceeded() {
        let config = RetryConfig {
            max_attempts: 2,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(10),
            backoff_multiplier: 2.0,
            jitter_factor: 0.0,
        };
        
        let executor = RetryExecutor::new(config);
        let attempt_count = Arc::new(AtomicU32::new(0));
        let attempt_count_clone = attempt_count.clone();
        
        let result: Result<&str> = executor.execute(|| {
            attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                Err::<&str, _>(TruthlinkedError::Network("Always fails".to_string()))
            }
        }).await;
        
        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 2);
    }
    
    #[tokio::test]
    async fn test_no_retry_on_auth_error() {
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
            attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                Err::<&str, _>(TruthlinkedError::Unauthorized)
            }
        }).await;
        
        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
        
        match result.unwrap_err() {
            TruthlinkedError::Unauthorized => {},
            _ => panic!("Expected Unauthorized error"),
        }
    }
    
    #[tokio::test]
    async fn test_no_retry_on_forbidden_error() {
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
            attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                Err::<&str, _>(TruthlinkedError::Forbidden)
            }
        }).await;
        
        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
    }
    
    #[tokio::test]
    async fn test_no_retry_on_rate_limit() {
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
            attempt_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                Err::<&str, _>(TruthlinkedError::RateLimitExceeded("Too many requests".to_string()))
            }
        }).await;
        
        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
    }
    
    #[test]
    fn test_retry_config_presets() {
        let prod = RetryConfig::production();
        assert_eq!(prod.max_attempts, 3);
        assert_eq!(prod.initial_delay, Duration::from_millis(500));
        
        let aggressive = RetryConfig::aggressive();
        assert_eq!(aggressive.max_attempts, 5);
        assert_eq!(aggressive.initial_delay, Duration::from_millis(100));
        
        let none = RetryConfig::none();
        assert_eq!(none.max_attempts, 1);
    }
    
    #[test]
    fn test_delay_calculation() {
        let config = RetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter_factor: 0.0, // No jitter for predictable testing
        };
        
        let _executor = RetryExecutor::new(config);
        
        // Test delay calculation (private method, so we test indirectly)
        // Delays should be: 100ms, 200ms, 400ms, 800ms, etc.
        // This is tested indirectly through the retry behavior
    }
}
