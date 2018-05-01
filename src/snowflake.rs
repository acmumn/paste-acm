//! An implementation of [Twitter's Snowflake
//! algorithm](https://developer.twitter.com/en/docs/basics/twitter-ids).

use std::cell::RefCell;

thread_local! {
    static INCREMENT: RefCell<u64> = RefCell::new(0);
    static THREAD_ID: u64 = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::thread;

        let mut hasher = DefaultHasher::new();
        thread::current().id().hash(&mut hasher);
        hasher.finish()
    };
}

/// Generates a snowflake.
pub fn snowflake() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let n = INCREMENT.with(|i| {
        let mut i = i.borrow_mut();
        let n = *i;
        *i = (n + 1) & 0xfff;
        n
    });

    let g = THREAD_ID.with(|&x| x) & 0x3ff;

    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before the Unix epoch?");
    let t = (t.as_secs() * 1000) + ((t.subsec_nanos() as u64) / 1_000_000);
    let t = t % 0x3ffffffffff;

    (t << 22) | (g << 12) | n
}

/// Generates a snowflake as a base64 string.
pub fn snowflake_b64() -> String {
    use base64::{encode_config, URL_SAFE};
    use byteorder::{ByteOrder, LittleEndian};

    let mut buf = [0u8; 8];
    LittleEndian::write_u64(&mut buf, snowflake());
    encode_config(&buf, URL_SAFE)
}
