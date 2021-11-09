use anyhow::Result;
use std::sync::atomic::{AtomicU32, Ordering::SeqCst};

wai_bindgen_rust::export!("./tests/runtime/numbers/imports.wai");

static SCALAR: AtomicU32 = AtomicU32::new(0);

pub struct Imports;

impl imports::Imports for Imports {
    fn roundtrip_u8(val: u8) -> u8 {
        val
    }

    fn roundtrip_s8(val: i8) -> i8 {
        val
    }

    fn roundtrip_u16(val: u16) -> u16 {
        val
    }

    fn roundtrip_s16(val: i16) -> i16 {
        val
    }

    fn roundtrip_u32(val: u32) -> u32 {
        val
    }

    fn roundtrip_s32(val: i32) -> i32 {
        val
    }

    fn roundtrip_u64(val: u64) -> u64 {
        val
    }

    fn roundtrip_s64(val: i64) -> i64 {
        val
    }

    fn roundtrip_f32(val: f32) -> f32 {
        val
    }

    fn roundtrip_f64(val: f64) -> f64 {
        val
    }

    fn roundtrip_char(val: char) -> char {
        val
    }

    fn set_scalar(val: u32) {
        SCALAR.store(val, SeqCst)
    }

    fn get_scalar() -> u32 {
        SCALAR.load(SeqCst)
    }
}

wai_bindgen_rust::import!("./tests/runtime/numbers/exports.wai");

fn run() -> Result<()> {
    exports::test_number_imports();
    assert_eq!(exports::roundtrip_u8(1), 1);
    assert_eq!(
        exports::roundtrip_u8(u8::min_value()),
        u8::min_value()
    );
    assert_eq!(
        exports::roundtrip_u8(u8::max_value()),
        u8::max_value()
    );

    assert_eq!(exports::roundtrip_s8(1), 1);
    assert_eq!(
        exports::roundtrip_s8(i8::min_value()),
        i8::min_value()
    );
    assert_eq!(
        exports::roundtrip_s8(i8::max_value()),
        i8::max_value()
    );

    assert_eq!(exports::roundtrip_u16(1), 1);
    assert_eq!(
        exports::roundtrip_u16(u16::min_value()),
        u16::min_value()
    );
    assert_eq!(
        exports::roundtrip_u16(u16::max_value()),
        u16::max_value()
    );

    assert_eq!(exports::roundtrip_s16(1), 1);
    assert_eq!(
        exports::roundtrip_s16(i16::min_value()),
        i16::min_value()
    );
    assert_eq!(
        exports::roundtrip_s16(i16::max_value()),
        i16::max_value()
    );

    assert_eq!(exports::roundtrip_u32(1), 1);
    assert_eq!(
        exports::roundtrip_u32(u32::min_value()),
        u32::min_value()
    );
    assert_eq!(
        exports::roundtrip_u32(u32::max_value()),
        u32::max_value()
    );

    assert_eq!(exports::roundtrip_s32(1), 1);
    assert_eq!(
        exports::roundtrip_s32(i32::min_value()),
        i32::min_value()
    );
    assert_eq!(
        exports::roundtrip_s32(i32::max_value()),
        i32::max_value()
    );

    assert_eq!(exports::roundtrip_u64(1), 1);
    assert_eq!(
        exports::roundtrip_u64(u64::min_value()),
        u64::min_value()
    );
    assert_eq!(
        exports::roundtrip_u64(u64::max_value()),
        u64::max_value()
    );

    assert_eq!(exports::roundtrip_s64(1), 1);
    assert_eq!(
        exports::roundtrip_s64(i64::min_value()),
        i64::min_value()
    );
    assert_eq!(
        exports::roundtrip_s64(i64::max_value()),
        i64::max_value()
    );

    assert_eq!(exports::roundtrip_f32(1.0), 1.0);
    assert_eq!(
        exports::roundtrip_f32(f32::INFINITY),
        f32::INFINITY
    );
    assert_eq!(
        exports::roundtrip_f32(f32::NEG_INFINITY),
        f32::NEG_INFINITY
    );
    assert!(exports::roundtrip_f32(f32::NAN).is_nan());

    assert_eq!(exports::roundtrip_f64(1.0), 1.0);
    assert_eq!(
        exports::roundtrip_f64(f64::INFINITY),
        f64::INFINITY
    );
    assert_eq!(
        exports::roundtrip_f64(f64::NEG_INFINITY),
        f64::NEG_INFINITY
    );
    assert!(exports::roundtrip_f64(f64::NAN).is_nan());

    assert_eq!(exports::roundtrip_char('a'), 'a');
    assert_eq!(exports::roundtrip_char(' '), ' ');
    assert_eq!(exports::roundtrip_char('🚩'), '🚩');

    exports::set_scalar(2);
    assert_eq!(exports::get_scalar(), 2);
    exports::set_scalar(4);
    assert_eq!(exports::get_scalar(), 4);

    Ok(())
}
