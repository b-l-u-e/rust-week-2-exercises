use rust_week_2_exercises::*;

fn main() {
    println!("🦀 Rust Week 2 Exercises Demo\n");

    // Test hex decoding and endianness (matching unit test)
    println!("=== Hex Operations & Endianness ===");
    let hex = "6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd73800100000";
    let le_bytes = decode_hex(hex).unwrap();
    let be_bytes = to_big_endian(&le_bytes);

    println!("Original hex: {}", hex);
    println!("Decoded to {} bytes", le_bytes.len());
    println!("Little endian (first 8 bytes): {:02x?}", &le_bytes[..8]);
    println!("Big endian (first 8 bytes): {:02x?}", &be_bytes[..8]);
    println!(
        "Endian conversion works: {}",
        be_bytes == le_bytes.iter().rev().cloned().collect::<Vec<_>>()
    );

    // Test basic hex conversion
    println!("\n--- Basic Hex Conversion ---");
    let bytes = vec![0x01, 0x02, 0xff];
    let hex_result = bytes_to_hex(&bytes);
    let bytes_back = hex_to_bytes(&hex_result).unwrap();

    println!("Bytes: {:02x?}", bytes);
    println!("To hex: {}", hex_result);
    println!("Back to bytes: {:02x?}", bytes_back);
    println!("Round trip successful: {}", bytes == bytes_back);

    // Test u32 endian swap
    println!("\n--- U32 Endian Swap ---");
    let num = 0x12345678u32;
    let le_bytes = swap_endian_u32(num);
    println!(
        "u32 0x{:08x} -> little-endian bytes: {:02x?}",
        num, le_bytes
    );
    println!("Expected: [78, 56, 34, 12], Got: {:02x?}", le_bytes);

    println!("\n=== Satoshi Parsing ===");

    // Test valid satoshi parsing
    match parse_satoshis("1000") {
        Ok(sats) => println!("✓ Parsed '1000' -> {} satoshis", sats),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Test invalid satoshi parsing
    match parse_satoshis("abc") {
        Ok(sats) => println!("✓ Parsed 'abc' -> {} satoshis", sats),
        Err(e) => println!("✓ Expected error for 'abc': {}", e),
    }

    // Test edge cases
    match parse_satoshis("21000000000000000") {
        // 21M BTC in sats
        Ok(sats) => println!("✓ Max supply: {} satoshis", sats),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n=== Script Classification ===");

    // Test P2PKH script (OP_DUP OP_HASH160 OP_PUSHDATA)
    let p2pkh_script = vec![0x76, 0xa9, 0x14];
    match classify_script(&p2pkh_script) {
        ScriptType::P2PKH => println!("✓ Detected P2PKH script pattern"),
        ScriptType::P2WPKH => println!("✗ Misclassified as P2WPKH"),
        ScriptType::Unknown => println!("✗ Failed to detect P2PKH"),
    }

    // Test P2WPKH script (OP_0 OP_PUSHDATA)
    let p2wpkh_script = vec![0x00, 0x14, 0xff];
    match classify_script(&p2wpkh_script) {
        ScriptType::P2PKH => println!("✗ Misclassified as P2PKH"),
        ScriptType::P2WPKH => println!("✓ Detected P2WPKH script pattern"),
        ScriptType::Unknown => println!("✗ Failed to detect P2WPKH"),
    }

    // Test unknown script
    let unknown_script = vec![0xab, 0xcd];
    match classify_script(&unknown_script) {
        ScriptType::P2PKH => println!("✗ Misclassified as P2PKH"),
        ScriptType::P2WPKH => println!("✗ Misclassified as P2WPKH"),
        ScriptType::Unknown => println!("✓ Correctly identified unknown script"),
    }

    println!("\n=== Outpoint Operations ===");

    // Test outpoint destructuring (matching unit test)
    let op = Outpoint("abcd1234".to_string(), 1);
    let Outpoint(txid, vout) = op;
    println!("Created outpoint: txid={}, vout={}", txid, vout);

    // Test with realistic transaction ID
    let real_txid = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let outpoint = Outpoint(real_txid.to_string(), 0);
    println!("Real outpoint: {}:{}", outpoint.0, outpoint.1);

    println!("\n=== Script Data Extraction ===");

    // Test script slice (matching unit test)
    let mut script = vec![0x00, 0x14];
    script.extend(vec![0u8; 20]); // Add 20 zero bytes
    let data = read_pushdata(&script);
    println!(
        "Script length: {}, pushdata length: {}",
        script.len(),
        data.len()
    );
    println!(
        "Pushdata (first 8 bytes): {:02x?}",
        &data[..8.min(data.len())]
    );

    // Test with different script
    let script2 = vec![0x76, 0xa9, 0xde, 0xad, 0xbe, 0xef];
    let data2 = read_pushdata(&script2);
    println!("Script2 pushdata: {:02x?}", data2);

    println!("\n=== Wallet Operations ===");

    // Test wallet balance trait
    let wallet = TestWallet { confirmed: 1500 };
    println!("Wallet confirmed balance: {} satoshis", wallet.balance());

    // Test larger wallet
    let big_wallet = TestWallet {
        confirmed: 100000000,
    }; // 1 BTC
    println!(
        "Big wallet balance: {} sats ({:.8} BTC)",
        big_wallet.balance(),
        big_wallet.balance() as f64 / 100_000_000.0
    );

    // Test fee application
    let mut balance = 10000;
    println!("Original balance: {} sats", balance);
    apply_fee(&mut balance, 250);
    println!("After 250 sat fee: {} sats", balance);

    // Test fee larger than balance (should not underflow)
    let mut small_balance = 100;
    println!("Small balance: {} sats", small_balance);
    apply_fee(&mut small_balance, 250);
    println!("After 250 sat fee (protected): {} sats", small_balance);

    println!("\n=== Transaction ID Operations ===");

    // Test move_txid (matching unit test)
    let original = "deadbeef".to_string();
    let result = move_txid(original.clone());
    println!("Original: {}", original);
    println!("Formatted: {}", result);

    // Test with real txid
    let real_txid = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string();
    println!("Real txid: {}", move_txid(real_txid));

    println!("\n=== Opcode Parsing ===");

    // Test valid opcodes
    match Opcode::from_byte(0xac) {
        Ok(opcode) => println!("✓ 0xac -> {:?}", opcode),
        Err(e) => println!("✗ Error: {}", e),
    }

    match Opcode::from_byte(0x76) {
        Ok(opcode) => println!("✓ 0x76 -> {:?}", opcode),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Test invalid opcode (matching unit test)
    match Opcode::from_byte(0x00) {
        Ok(opcode) => println!("✗ Unexpected success: {:?}", opcode),
        Err(e) => println!("✓ Expected error for 0x00: {}", e),
    }

    // Test range of opcodes
    let test_opcodes = vec![0x76, 0xac, 0x51, 0x52, 0x00, 0xff];
    for byte in test_opcodes {
        match Opcode::from_byte(byte) {
            Ok(opcode) => println!("  0x{:02x} -> {:?}", byte, opcode),
            Err(e) => println!("  0x{:02x} -> {}", byte, e),
        }
    }

    println!("\n=== UTXO Operations ===");

    // Test UTXO ownership and cloning (matching unit test)
    let utxo = UTXO {
        txid: vec![0xaa, 0xbb],
        vout: 0,
        value: 1000,
    };

    println!("Original UTXO: {:?}", utxo);
    let consumed = consume_utxo(utxo.clone());
    println!("Consumed UTXO: {:?}", consumed);
    println!("UTXOs equal: {}", consumed == utxo);

    // Test with realistic UTXO
    let real_utxo = UTXO {
        txid: hex_to_bytes("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
            .unwrap(),
        vout: 0,
        value: 50000000, // 0.5 BTC
    };

    println!("\nRealistic UTXO:");
    println!("  TXID: {}", bytes_to_hex(&real_utxo.txid));
    println!("  VOUT: {}", real_utxo.vout);
    println!(
        "  Value: {} sats ({:.8} BTC)",
        real_utxo.value,
        real_utxo.value as f64 / 100_000_000.0
    );

    // Test multiple UTXOs for a wallet
    let utxos = vec![
        UTXO {
            txid: vec![0x01, 0x02],
            vout: 0,
            value: 10000000,
        },
        UTXO {
            txid: vec![0x03, 0x04],
            vout: 1,
            value: 25000000,
        },
        UTXO {
            txid: vec![0x05, 0x06],
            vout: 0,
            value: 15000000,
        },
    ];

    let total_value: u64 = utxos.iter().map(|u| u.value).sum();
    println!("\nWallet UTXOs:");
    for (i, utxo) in utxos.iter().enumerate() {
        println!("  UTXO {}: {} sats (vout {})", i + 1, utxo.value, utxo.vout);
    }
    println!(
        "Total wallet value: {} sats ({:.8} BTC)",
        total_value,
        total_value as f64 / 100_000_000.0
    );

    println!("\n✨ All demos completed successfully!");
    println!("💡 Run 'cargo test' to verify all unit tests pass!");
}
