fn main() -> sled::Result<()> {
    let tree = sled::open("/tmp/welcome-to-sled")?;

    // insert and get, similar to std's BTreeMap
    let old_value = tree.insert("key_1", "value1")?;
    let _val2key2 = tree.insert("key_2", "value2")?;
    let _key3val3 = tree.insert("key_3", "value3")?;
    let _key4val4 = tree.insert("key_4", "value4")?;
    
    if let Some(val) = old_value {
        print!("Oldest Value: {:?}\n", std::str::from_utf8(&val));
    }
    assert_eq!(tree.get(&"key_1").expect("get Tree"), Some(sled::IVec::from("value1")),);

    // range queries
    for kv_result in tree.range("key_1".."key_9") {
        let (key, val) = kv_result.unwrap();
        let key_str = std::str::from_utf8(&key);
        let val_str = std::str::from_utf8(&val);

        print!("Got result {:?} : {:?}\n", key_str, val_str);
    }

    // deletion
    let old_value = tree.remove(&"key_1")?;
    print!("Pre delete Old value: {:?}\n", std::str::from_utf8(&old_value.unwrap()));

    // atomic compare and swap
    let _ = tree.insert("key_1", "current_value")?;
    let _ = tree.compare_and_swap("key_1", Some("current_value"), Some("new_value"))?;

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)
    tree.flush()?;

    Ok(())
}
