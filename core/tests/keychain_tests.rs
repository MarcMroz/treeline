//! Keychain service integration tests
//!
//! Tests for KeychainService: store, retrieve, delete, overwrite, availability.
//! These tests interact with the real OS keychain.
//!
//! On platforms where the keychain is unavailable (e.g., headless Linux CI),
//! tests skip gracefully via the `require_keychain!` macro.
//!
//! All mutating tests are serialized via KEYCHAIN_LOCK because they share a
//! single global keychain entry (service="treeline", user="encryption-key").
//!
//! Run with: cargo test --test keychain_tests -- --nocapture

use std::sync::Mutex;

use treeline_core::services::KeychainService;

/// Serialize all keychain tests — they share a single global entry.
static KEYCHAIN_LOCK: Mutex<()> = Mutex::new(());

/// Skip the test if the OS keychain is not available (e.g., headless Linux CI).
macro_rules! require_keychain {
    () => {
        if !KeychainService::is_available() {
            eprintln!("SKIPPED: OS keychain not available on this platform");
            return;
        }
    };
}

/// Clean up any leftover test key before/after each test.
fn cleanup() {
    let _ = KeychainService::delete_key();
}

#[test]
fn test_store_and_retrieve() {
    let _lock = KEYCHAIN_LOCK.lock().unwrap();
    require_keychain!();
    cleanup();

    let key = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
    KeychainService::store_key(key).expect("store_key should succeed");

    let retrieved = KeychainService::get_key()
        .expect("get_key should not error")
        .expect("get_key should return Some");

    assert_eq!(retrieved, key);
    cleanup();
}

#[test]
fn test_delete() {
    let _lock = KEYCHAIN_LOCK.lock().unwrap();
    require_keychain!();
    cleanup();

    let key = "deadbeef01234567deadbeef01234567deadbeef01234567deadbeef01234567";
    KeychainService::store_key(key).expect("store_key should succeed");

    KeychainService::delete_key().expect("delete_key should succeed");

    let retrieved = KeychainService::get_key().expect("get_key should not error");
    assert!(retrieved.is_none(), "key should be gone after delete");
}

#[test]
fn test_get_nonexistent() {
    let _lock = KEYCHAIN_LOCK.lock().unwrap();
    require_keychain!();
    cleanup();

    let retrieved = KeychainService::get_key().expect("get_key should not error");
    assert!(
        retrieved.is_none(),
        "get_key should return None when no key is stored"
    );
}

#[test]
fn test_overwrite() {
    let _lock = KEYCHAIN_LOCK.lock().unwrap();
    require_keychain!();
    cleanup();

    let key1 = "1111111111111111111111111111111111111111111111111111111111111111";
    let key2 = "2222222222222222222222222222222222222222222222222222222222222222";

    KeychainService::store_key(key1).expect("first store should succeed");
    KeychainService::store_key(key2).expect("second store should succeed");

    let retrieved = KeychainService::get_key()
        .expect("get_key should not error")
        .expect("get_key should return Some");

    assert_eq!(retrieved, key2, "should return the latest stored key");
    cleanup();
}

#[test]
fn test_delete_nonexistent() {
    let _lock = KEYCHAIN_LOCK.lock().unwrap();
    require_keychain!();
    cleanup();

    // Deleting when nothing is stored should succeed silently
    KeychainService::delete_key().expect("delete_key on nonexistent should succeed");
}

#[test]
fn test_is_available() {
    // This test is read-only — no lock needed
    let available = KeychainService::is_available();
    eprintln!("KeychainService::is_available() = {}", available);
    // On desktop macOS/Windows, this should be true
    // On headless Linux CI, it may be false
}
