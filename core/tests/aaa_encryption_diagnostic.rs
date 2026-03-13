//! Diagnostic test to determine if Linux encryption failure is caused by
//! enable_autoload_extension(false) or a deeper bundled build issue.
//!
//! Run with: cargo test --test encryption_linux_debug -- --nocapture

use duckdb::Connection;
use tempfile::TempDir;

/// Test 1: Try encryption with autoloading DISABLED (current behavior)
#[test]
fn test_encrypt_autoload_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_no_autoload.duckdb");

    let config = duckdb::Config::default()
        .enable_autoload_extension(false)
        .unwrap();
    let conn = Connection::open_in_memory_with_flags(config).unwrap();

    let result = conn.execute(
        &format!(
            "ATTACH '{}' AS enc (ENCRYPTION_KEY '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef')",
            db_path.display()
        ),
        [],
    );

    match &result {
        Ok(_) => println!("AUTOLOAD=false: encryption ATTACH succeeded"),
        Err(e) => println!("AUTOLOAD=false: encryption ATTACH failed: {e}"),
    }

    // Don't assert — we want to see the output on all platforms
    let autoload_disabled_works = result.is_ok();

    // Test 2: Try with autoloading ENABLED
    let temp_dir2 = TempDir::new().unwrap();
    let db_path2 = temp_dir2.path().join("test_with_autoload.duckdb");

    let config2 = duckdb::Config::default()
        .enable_autoload_extension(true)
        .unwrap();
    let conn2 = Connection::open_in_memory_with_flags(config2).unwrap();

    let result2 = conn2.execute(
        &format!(
            "ATTACH '{}' AS enc (ENCRYPTION_KEY '0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef')",
            db_path2.display()
        ),
        [],
    );

    match &result2 {
        Ok(_) => println!("AUTOLOAD=true: encryption ATTACH succeeded"),
        Err(e) => println!("AUTOLOAD=true: encryption ATTACH failed: {e}"),
    }

    let autoload_enabled_works = result2.is_ok();

    // Summary
    println!("\n=== ENCRYPTION DIAGNOSTIC SUMMARY ===");
    println!("Platform: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);
    println!("Autoload disabled: {}", if autoload_disabled_works { "WORKS" } else { "BROKEN" });
    println!("Autoload enabled:  {}", if autoload_enabled_works { "WORKS" } else { "BROKEN" });

    if !autoload_disabled_works && autoload_enabled_works {
        println!("DIAGNOSIS: enable_autoload_extension(false) is the cause");
    } else if !autoload_disabled_works && !autoload_enabled_works {
        println!("DIAGNOSIS: Bundled DuckDB build lacks encryption support entirely");
    } else {
        println!("DIAGNOSIS: Encryption works (both modes)");
    }

    // Assert at least one works, otherwise encryption is fundamentally broken
    assert!(
        autoload_disabled_works || autoload_enabled_works,
        "Encryption is completely broken on this platform"
    );
}
