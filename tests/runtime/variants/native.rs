use anyhow::Result;

wai_bindgen_rust::export!("./tests/runtime/variants/imports.wai");

pub struct Imports;

impl imports::Imports for Imports {
    fn roundtrip_option(a: Option<f32>) -> Option<u8> {
        a.map(|x| x as u8)
    }

    fn roundtrip_result(a: Result<u32, f32>) -> Result<f64, u8> {
        match a {
            Ok(a) => Ok(a.into()),
            Err(b) => Err(b as u8),
        }
    }

    fn roundtrip_enum(a: imports::E1) -> imports::E1 {
        assert_eq!(a, a);
        a
    }

    fn invert_bool(a: bool) -> bool {
        !a
    }

    fn variant_casts(a: imports::Casts) -> imports::Casts {
        a
    }

    fn variant_zeros(a: imports::Zeros) -> imports::Zeros {
        a
    }

    fn variant_typedefs(_: Option<u32>, _: bool, _: Result<u32, ()>) {}

    fn variant_enums(
        a: bool,
        b: Result<(), ()>,
        c: imports::MyErrno,
    ) -> (bool, Result<(), ()>, imports::MyErrno) {
        assert_eq!(a, true);
        assert_eq!(b, Ok(()));
        assert_eq!(c, imports::MyErrno::Success);
        (false, Err(()), imports::MyErrno::A)
    }
}

wai_bindgen_rust::import!("./tests/runtime/variants/exports.wai");

fn run() -> Result<()> {
    use exports::*;

    exports::test_variant_imports();

    assert_eq!(exports::roundtrip_option(Some(1.0)), Some(1));
    assert_eq!(exports::roundtrip_option(None), None);
    assert_eq!(exports::roundtrip_option(Some(2.0)), Some(2));
    assert_eq!(exports::roundtrip_result(Ok(2)), Ok(2.0));
    assert_eq!(exports::roundtrip_result(Ok(4)), Ok(4.0));
    assert_eq!(exports::roundtrip_result(Err(5.3)), Err(5));

    assert_eq!(exports::roundtrip_enum(E1::A), E1::A);
    assert_eq!(exports::roundtrip_enum(E1::B), E1::B);

    assert_eq!(exports::invert_bool(true), false);
    assert_eq!(exports::invert_bool(false), true);

    let (a1, a2, a3, a4, a5, a6) = exports::variant_casts(
        (C1::A(1), C2::A(2), C3::A(3), C4::A(4), C5::A(5), C6::A(6.0)),
    );
    assert!(matches!(a1, C1::A(1)));
    assert!(matches!(a2, C2::A(2)));
    assert!(matches!(a3, C3::A(3)));
    assert!(matches!(a4, C4::A(4)));
    assert!(matches!(a5, C5::A(5)));
    assert!(matches!(a6, C6::A(b) if b == 6.0));

    let (a1, a2, a3, a4, a5, a6) = exports::variant_casts(
        (
            C1::B(1),
            C2::B(2.0),
            C3::B(3.0),
            C4::B(4.0),
            C5::B(5.0),
            C6::B(6.0),
        ),
    );
    assert!(matches!(a1, C1::B(1)));
    assert!(matches!(a2, C2::B(b) if b == 2.0));
    assert!(matches!(a3, C3::B(b) if b == 3.0));
    assert!(matches!(a4, C4::B(b) if b == 4.0));
    assert!(matches!(a5, C5::B(b) if b == 5.0));
    assert!(matches!(a6, C6::B(b) if b == 6.0));

    let (a1, a2, a3, a4) =
        exports::variant_zeros((Z1::A(1), Z2::A(2), Z3::A(3.0), Z4::A(4.0)));
    assert!(matches!(a1, Z1::A(1)));
    assert!(matches!(a2, Z2::A(2)));
    assert!(matches!(a3, Z3::A(b) if b == 3.0));
    assert!(matches!(a4, Z4::A(b) if b == 4.0));

    exports::variant_typedefs(None, false, Err(()));

    Ok(())
}
