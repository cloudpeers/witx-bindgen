use anyhow::Result;

wai_bindgen_rust::export!("./tests/runtime/records/imports.wai");

pub struct Imports;

impl imports::Imports for Imports {
    fn multiple_results() -> (u8, u16) {
        (4, 5)
    }

    fn swap_tuple(a: (u8, u32)) -> (u32, u8) {
        (a.1, a.0)
    }

    fn roundtrip_flags1(a: imports::F1) -> imports::F1 {
        drop(a.to_string());
        drop(format!("{:?}", a));
        drop(a & u8::MAX);
        a
    }

    fn roundtrip_flags2(a: imports::F2) -> imports::F2 {
        a
    }

    fn roundtrip_flags3(
        a: imports::Flag8,
        b: imports::Flag16,
        c: imports::Flag32,
        d: imports::Flag64,
    ) -> (imports::Flag8, imports::Flag16, imports::Flag32, imports::Flag64) {
        (a, b, c, d)
    }

    fn roundtrip_record1(a: imports::R1) -> imports::R1 {
        drop(format!("{:?}", a));
        a
    }

    fn tuple0(_: ()) {}

    fn tuple1(a: (u8,)) -> (u8,) {
        (a.0,)
    }
}

wai_bindgen_rust::import!("./tests/runtime/records/exports.wai");

fn run() -> Result<()> {
    use exports::*;

    exports::test_record_imports();
    assert_eq!(exports::multiple_results(), (100, 200));
    assert_eq!(exports::swap_tuple((1u8, 2u32)), (2u32, 1u8));
    assert_eq!(exports::roundtrip_flags1(F1_A), F1_A);
    assert_eq!(
        exports::roundtrip_flags1(0),
        0
    );
    assert_eq!(exports::roundtrip_flags1(F1_B), F1_B);
    assert_eq!(
        exports::roundtrip_flags1(F1_A | F1_B),
        F1_A | F1_B
    );

    assert_eq!(exports::roundtrip_flags2(F2_C), F2_C);
    assert_eq!(
        exports::roundtrip_flags2(0),
        0
    );
    assert_eq!(exports::roundtrip_flags2(F2_D), F2_D);
    assert_eq!(
        exports::roundtrip_flags2(F2_C | F2_E),
        F2_C | F2_E
    );

    let r = exports::roundtrip_record1(
        R1 {
            a: 8,
            b: 0,
        },
    );
    assert_eq!(r.a, 8);
    assert_eq!(r.b, 0);

    let r = exports::roundtrip_record1(
        R1 {
            a: 0,
            b: F1_A | F1_B,
        },
    );
    assert_eq!(r.a, 0);
    assert_eq!(r.b, F1_A | F1_B);

    assert_eq!(exports::tuple0(()), ());
    assert_eq!(exports::tuple1((1,)), (1,));
    Ok(())
}
