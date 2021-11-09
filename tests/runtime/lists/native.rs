use anyhow::Result;

wai_bindgen_rust::export!("./tests/runtime/lists/imports.wai");

pub struct Imports;

impl imports::Imports for Imports {
    fn list_param(list: Vec<u8>) {
        assert_eq!(list, [1, 2, 3, 4]);
    }

    fn list_param2(ptr: String) {
        assert_eq!(ptr, "foo");
    }

    fn list_param3(ptr: Vec<String>) {
        assert_eq!(ptr.len(), 3);
        assert_eq!(ptr[0], "foo");
        assert_eq!(ptr[1], "bar");
        assert_eq!(ptr[2], "baz");
    }

    fn list_param4(ptr: Vec<Vec<String>>) {
        assert_eq!(ptr.len(), 2);
        assert_eq!(ptr[0][0], "foo");
        assert_eq!(ptr[0][1], "bar");
        assert_eq!(ptr[1][0], "baz");
    }

    fn list_result() -> Vec<u8> {
        vec![1, 2, 3, 4, 5]
    }

    fn list_result2() -> String {
        "hello!".to_string()
    }

    fn list_result3() -> Vec<String> {
        vec!["hello,".to_string(), "world!".to_string()]
    }

    fn string_roundtrip(s: String) -> String {
        s.to_string()
    }

    fn list_minmax8(u: Vec<u8>, s: Vec<i8>) -> (Vec<u8>, Vec<i8>) {
        assert_eq!(u, [u8::MIN, u8::MAX]);
        assert_eq!(s, [i8::MIN, i8::MAX]);
        (u, s)
    }

    fn list_minmax16(u: Vec<u16>, s: Vec<i16>) -> (Vec<u16>, Vec<i16>) {
        assert_eq!(u, [u16::MIN, u16::MAX]);
        assert_eq!(s, [i16::MIN, i16::MAX]);
        (u, s)
    }

    fn list_minmax32(u: Vec<u32>, s: Vec<i32>) -> (Vec<u32>, Vec<i32>) {
        assert_eq!(u, [u32::MIN, u32::MAX]);
        assert_eq!(s, [i32::MIN, i32::MAX]);
        (u, s)
    }

    fn list_minmax64(u: Vec<u64>, s: Vec<i64>) -> (Vec<u64>, Vec<i64>) {
        assert_eq!(u, [u64::MIN, u64::MAX]);
        assert_eq!(s, [i64::MIN, i64::MAX]);
        (u, s)
    }

    fn list_minmax_float(u: Vec<f32>, s: Vec<f64>) -> (Vec<f32>, Vec<f64>) {
        assert_eq!(u, [f32::MIN, f32::MAX, f32::NEG_INFINITY, f32::INFINITY]);
        assert_eq!(s, [f64::MIN, f64::MAX, f64::NEG_INFINITY, f64::INFINITY]);
        (u, s)
    }

    fn unaligned_roundtrip1(
        u16s: Vec<u16>,
        u32s: Vec<u32>,
        u64s: Vec<u64>,
        flag32s: Vec<imports::Flag32>,
        flag64s: Vec<imports::Flag64>,
    ) {
        assert_eq!(u16s, [1]);
        assert_eq!(u32s, [2]);
        assert_eq!(u64s, [3]);
        assert_eq!(flag32s, [imports::FLAG32_B8]);
        assert_eq!(flag64s, [imports::FLAG64_B9]);
    }

    fn unaligned_roundtrip2(
        records: Vec<imports::UnalignedRecord>,
        f32s: Vec<f32>,
        f64s: Vec<f64>,
        strings: Vec<String>,
        lists: Vec<Vec<u8>>,
    ) {
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].a, 10);
        assert_eq!(records[0].b, 11);
        assert_eq!(f32s, [100.0]);
        assert_eq!(f64s, [101.0]);
        assert_eq!(strings, ["foo"]);
        assert_eq!(lists, [&[102][..]]);
    }
}

wai_bindgen_rust::import!("./tests/runtime/lists/exports.wai");

fn run() -> Result<()> {
    let bytes = exports::allocated_bytes();
    exports::test_list_imports();
    exports::list_param(&[1, 2, 3, 4]);
    exports::list_param2("foo");
    exports::list_param3(&["foo", "bar", "baz"]);
    exports::list_param4(&[&["foo", "bar"], &["baz"]]);
    assert_eq!(exports::list_result(), [1, 2, 3, 4, 5]);
    assert_eq!(exports::list_result2(), "hello!");
    assert_eq!(exports::list_result3(), ["hello,", "world!"]);
    assert_eq!(exports::string_roundtrip("x"), "x");
    assert_eq!(exports::string_roundtrip(""), "");
    assert_eq!(
        exports::string_roundtrip("hello ⚑ world"),
        "hello ⚑ world"
    );
    // Ensure that we properly called `free` everywhere in all the glue that we
    // needed to.
    assert_eq!(bytes, exports::allocated_bytes());
    Ok(())
}
