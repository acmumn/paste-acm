//! An implementation of [Twitter's Snowflake
//! algorithm](https://developer.twitter.com/en/docs/basics/twitter-ids).

use std::cell::RefCell;

use base64::{DecodeError, URL_SAFE};

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
pub fn snowflake_b64() -> (u64, String) {
    use base64::encode_config;
    use byteorder::{BigEndian, ByteOrder};

    let n = snowflake();

    let mut buf = [0u8; 8];
    BigEndian::write_u64(&mut buf, n);
    let s = encode_config(&buf, URL_SAFE);

    (n, s)
}

/// Decodes a snowflake from a string, which is interpreterd as base64.
pub fn decode_snowflake(s: &str) -> Result<u64, DecodeError> {
    use base64::decode_config;
    use byteorder::{BigEndian, ByteOrder};

    let bs = decode_config(s, URL_SAFE)?;
    if bs.len() == 8 {
        Ok(BigEndian::read_u64(&bs))
    } else {
        Err(DecodeError::InvalidLength)
    }
}
