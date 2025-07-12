//! Tests for the Wallets struct.
//! 
//! These tests cover all methods of the Wallets struct including:
//! - Constructor methods (new, default)
//! - Wallet creation and management
//! - Address retrieval
//! - File persistence
//! - Performance characteristics
//! - Error handling
//! 
//! Note: These tests now use isolated file paths and should be safe to run concurrently.

#[cfg(test)]
mod tests {
    use rust_blockchain::wallet::Wallets;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Test helper to create a temporary directory for test isolation
    fn setup_test_env() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let wallet_file_path = temp_dir.path().join("wallet.dat");
        (temp_dir, wallet_file_path)
    }

    // Test helper to cleanup (TempDir handles cleanup automatically)
    fn cleanup_test_env(_temp_dir: &TempDir) {
        // TempDir automatically cleans up when dropped
    }

    // =============================================================================
    // BASIC WALLETS TESTS (NO FILE PERSISTENCE)
    // =============================================================================

    #[test]
    fn test_wallets_new_empty() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let wallets = Wallets::new_with_file_path(wallet_file_path);
        assert_eq!(wallets.get_addresses().len(), 0);
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_wallets_default_trait() {
        let (temp_dir, _wallet_file_path) = setup_test_env();
        
        let wallets = Wallets::default();
        assert_eq!(wallets.get_addresses().len(), 0);
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_create_single_wallet() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let address = wallets.create_wallet();
        
        // Address should be valid
        assert!(!address.is_empty());
        assert!(address.len() > 25); // Base58 addresses are typically longer
        
        // Should have exactly one wallet
        assert_eq!(wallets.get_addresses().len(), 1);
        assert!(wallets.get_addresses().contains(&address));
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_create_multiple_wallets() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        
        // Create 5 wallets
        let mut addresses = Vec::new();
        for _ in 0..5 {
            let address = wallets.create_wallet();
            addresses.push(address);
        }
        
        // All addresses should be unique
        for i in 0..addresses.len() {
            for j in i + 1..addresses.len() {
                assert_ne!(addresses[i], addresses[j]);
            }
        }
        
        // Should have exactly 5 wallets
        assert_eq!(wallets.get_addresses().len(), 5);
        
        // All created addresses should be in the list
        for address in &addresses {
            assert!(wallets.get_addresses().contains(address));
        }
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_get_addresses_empty() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let wallets = Wallets::new_with_file_path(wallet_file_path);
        let addresses = wallets.get_addresses();
        
        assert_eq!(addresses.len(), 0);
        assert!(addresses.is_empty());
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_get_addresses_with_wallets() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let addr1 = wallets.create_wallet();
        let addr2 = wallets.create_wallet();
        
        let addresses = wallets.get_addresses();
        assert_eq!(addresses.len(), 2);
        assert!(addresses.contains(&addr1));
        assert!(addresses.contains(&addr2));
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_get_wallet_existing() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let address = wallets.create_wallet();
        
        let wallet = wallets.get_wallet(&address);
        assert!(wallet.is_some());
        
        let retrieved_wallet = wallet.unwrap();
        assert_eq!(retrieved_wallet.get_address(), address);
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_get_wallet_nonexistent() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let wallets = Wallets::new_with_file_path(wallet_file_path);
        
        assert!(wallets.get_wallet("nonexistent").is_none());
        assert!(wallets.get_wallet("").is_none());
        assert!(wallets.get_wallet("invalid_address_123").is_none());
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_get_wallet_multiple() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let addr1 = wallets.create_wallet();
        let addr2 = wallets.create_wallet();
        let addr3 = wallets.create_wallet();
        
        // Should be able to retrieve all wallets
        let wallet1 = wallets.get_wallet(&addr1);
        let wallet2 = wallets.get_wallet(&addr2);
        let wallet3 = wallets.get_wallet(&addr3);
        
        assert!(wallet1.is_some());
        assert!(wallet2.is_some());
        assert!(wallet3.is_some());
        
        assert_eq!(wallet1.unwrap().get_address(), addr1);
        assert_eq!(wallet2.unwrap().get_address(), addr2);
        assert_eq!(wallet3.unwrap().get_address(), addr3);
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_wallet_address_case_sensitivity() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let address = wallets.create_wallet();
        
        // Should be case sensitive
        assert!(wallets.get_wallet(&address).is_some());
        assert!(wallets.get_wallet(&address.to_uppercase()).is_none());
        assert!(wallets.get_wallet(&address.to_lowercase()).is_none());
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_wallets_full_lifecycle() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        
        // Start empty
        assert_eq!(wallets.get_addresses().len(), 0);
        
        // Create wallets
        let addr1 = wallets.create_wallet();
        let addr2 = wallets.create_wallet();
        
        // Verify state
        assert_eq!(wallets.get_addresses().len(), 2);
        assert!(wallets.get_wallet(&addr1).is_some());
        assert!(wallets.get_wallet(&addr2).is_some());
        assert_ne!(addr1, addr2);
        
        // Verify addresses list
        let addresses = wallets.get_addresses();
        assert!(addresses.contains(&addr1));
        assert!(addresses.contains(&addr2));
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_wallet_uniqueness() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let mut addresses = Vec::new();
        
        // Create 20 wallets
        for _ in 0..20 {
            let address = wallets.create_wallet();
            assert!(!addresses.contains(&address), "Duplicate address: {address}");
            addresses.push(address);
        }
        
        assert_eq!(addresses.len(), 20);
        assert_eq!(wallets.get_addresses().len(), 20);
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_wallet_properties() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let address = wallets.create_wallet();
        
        let wallet = wallets.get_wallet(&address).unwrap();
        
        // Verify wallet properties
        assert!(!wallet.get_public_key().is_empty());
        assert!(!wallet.get_pkcs8().is_empty());
        assert_eq!(wallet.get_address(), address);
        
        // Verify key sizes are reasonable
        assert!(wallet.get_public_key().len() >= 33); // Compressed ECDSA key
        assert!(wallet.get_pkcs8().len() >= 32); // PKCS8 format
        
        cleanup_test_env(&temp_dir);
    }

    // =============================================================================
    // PERFORMANCE TESTS
    // =============================================================================

    #[test]
    fn test_wallet_creation_performance() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let start_time = std::time::Instant::now();
        
        // Create 25 wallets
        for _ in 0..25 {
            wallets.create_wallet();
        }
        
        let duration = start_time.elapsed();
        assert!(duration.as_secs() < 3, "Wallet creation took too long: {duration:?}");
        assert_eq!(wallets.get_addresses().len(), 25);
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_wallet_retrieval_performance() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let mut wallets = Wallets::new_with_file_path(wallet_file_path);
        let mut addresses = Vec::new();
        
        // Create 50 wallets
        for _ in 0..50 {
            addresses.push(wallets.create_wallet());
        }
        
        // Test retrieval performance
        let start_time = std::time::Instant::now();
        for address in &addresses {
            assert!(wallets.get_wallet(address).is_some());
        }
        let duration = start_time.elapsed();
        
        assert!(duration.as_millis() < 100, "Wallet retrieval took too long: {duration:?}");
        
        cleanup_test_env(&temp_dir);
    }

    // =============================================================================
    // BASIC FILE PERSISTENCE TESTS
    // =============================================================================

    #[test]
    fn test_basic_file_persistence() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let created_address = {
            let mut wallets = Wallets::new_with_file_path(wallet_file_path.clone());
            wallets.create_wallet()
        };
        
        // Create new instance - should load from file
        let wallets = Wallets::new_with_file_path(wallet_file_path);
        assert_eq!(wallets.get_addresses().len(), 1);
        assert!(wallets.get_addresses().contains(&created_address));
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_persistence_with_multiple_wallets() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let (addr1, addr2) = {
            let mut wallets = Wallets::new_with_file_path(wallet_file_path.clone());
            (wallets.create_wallet(), wallets.create_wallet())
        };
        
        // Create new instance - should load both wallets
        let wallets = Wallets::new_with_file_path(wallet_file_path);
        assert_eq!(wallets.get_addresses().len(), 2);
        assert!(wallets.get_addresses().contains(&addr1));
        assert!(wallets.get_addresses().contains(&addr2));
        
        cleanup_test_env(&temp_dir);
    }

    #[test]
    fn test_incremental_persistence() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        // Create first wallet
        let addr1 = {
            let mut wallets = Wallets::new_with_file_path(wallet_file_path.clone());
            wallets.create_wallet()
        };
        
        // Load and add second wallet
        let addr2 = {
            let mut wallets = Wallets::new_with_file_path(wallet_file_path.clone());
            assert_eq!(wallets.get_addresses().len(), 1);
            wallets.create_wallet()
        };
        
        // Load and verify both exist
        let wallets = Wallets::new_with_file_path(wallet_file_path);
        assert_eq!(wallets.get_addresses().len(), 2);
        assert!(wallets.get_addresses().contains(&addr1));
        assert!(wallets.get_addresses().contains(&addr2));
        
        cleanup_test_env(&temp_dir);
    }

    // =============================================================================
    // ERROR HANDLING TESTS
    // =============================================================================

    #[test]
    fn test_error_handling() {
        let (temp_dir, wallet_file_path) = setup_test_env();
        
        let wallets = Wallets::new_with_file_path(wallet_file_path);
        
        // Test various invalid inputs
        assert!(wallets.get_wallet("").is_none());
        assert!(wallets.get_wallet("invalid").is_none());
        assert!(wallets.get_wallet("1").is_none());
        assert!(wallets.get_wallet("a".repeat(1000).as_str()).is_none());
        
        cleanup_test_env(&temp_dir);
    }
} 
