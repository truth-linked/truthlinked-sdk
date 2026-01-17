use truthlinked_sdk::*;
use mockito::Server;
use serde_json::json;

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_health_check_success() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                "status": "healthy",
                "version": "1.0.0"
            }).to_string())
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.status, "healthy");
        assert_eq!(health.version, "1.0.0");
        
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_health_check_server_error() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(500)
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::ServerError => {},
            e => panic!("Expected ServerError, got {:?}", e),
        }
        
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_health_check_unauthorized() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(401)
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::Unauthorized => {},
            e => panic!("Expected Unauthorized, got {:?}", e),
        }
        
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_health_check_forbidden() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(403)
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::Forbidden => {},
            e => panic!("Expected Forbidden, got {:?}", e),
        }
        
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_health_check_rate_limited() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(429)
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::RateLimitExceeded(_) => {},
            e => panic!("Expected RateLimitExceeded, got {:?}", e),
        }
        
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_health_check_invalid_json() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("invalid json")
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            TruthlinkedError::SerializationError => {},
            e => panic!("Expected SerializationError, got {:?}", e),
        }
        
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_health_check_with_retry() {
        let mut server = Server::new_async().await;
        
        // First request fails, second succeeds
        let mock_fail = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(500)
            .expect(1)
            .create_async()
            .await;
        
        let mock_success = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                "status": "healthy",
                "version": "1.0.0"
            }).to_string())
            .expect(1)
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .retries(2)
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.status, "healthy");
        
        mock_fail.assert_async().await;
        mock_success.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_request_signing_headers() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Regex(r"^\d+$".to_string()))
            .match_header("X-Signature", mockito::Matcher::Regex(r"^[A-Za-z0-9+/]+=*$".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                "status": "healthy",
                "version": "1.0.0"
            }).to_string())
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_ok());
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_custom_user_agent() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .match_header("User-Agent", "CustomApp/2.0")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                "status": "healthy",
                "version": "1.0.0"
            }).to_string())
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .user_agent("CustomApp/2.0")
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_ok());
        mock.assert_async().await;
    }
    
    #[tokio::test]
    async fn test_custom_headers() {
        let mut server = Server::new_async().await;
        
        let mock = server.mock("GET", "/health")
            .match_header("X-Timestamp", mockito::Matcher::Any)
            .match_header("X-Signature", mockito::Matcher::Any)
            .match_header("X-Custom-Header", "custom-value")
            .match_header("X-Request-ID", "12345")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                "status": "healthy",
                "version": "1.0.0"
            }).to_string())
            .create_async()
            .await;
        
        let client = ClientBuilder::testing(&server.url(), "test_key")
            .header("X-Custom-Header", "custom-value").unwrap()
            .header("X-Request-ID", "12345").unwrap()
            .build()
            .unwrap();
        
        let result = client.health().await;
        
        assert!(result.is_ok());
        mock.assert_async().await;
    }
}
