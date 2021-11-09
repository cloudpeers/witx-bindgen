use anyhow::Result;

wai_bindgen_rust::export!("./tests/runtime/flavorful/imports.wai");

pub struct Imports;

impl imports::Imports for Imports {
    fn list_in_record1(ty: imports::ListInRecord1) {
        assert_eq!(ty.a, "list_in_record1");
    }

    fn list_in_record2() -> imports::ListInRecord2 {
        imports::ListInRecord2 {
            a: "list_in_record2".to_string(),
        }
    }

    fn list_in_record3(a: imports::ListInRecord3) -> imports::ListInRecord3 {
        assert_eq!(a.a, "list_in_record3 input");
        imports::ListInRecord3 {
            a: "list_in_record3 output".to_string(),
        }
    }

    fn list_in_record4(a: imports::ListInAlias) -> imports::ListInAlias {
        assert_eq!(a.a, "input4");
        imports::ListInRecord4 {
            a: "result4".to_string(),
        }
    }

    fn list_in_variant1(
        a: imports::ListInVariant11,
        b: imports::ListInVariant12,
        c: imports::ListInVariant13,
    ) {
        assert_eq!(a.unwrap(), "foo");
        assert_eq!(b.unwrap_err(), "bar");
        match c {
            imports::ListInVariant13::V0(s) => assert_eq!(s, "baz"),
            imports::ListInVariant13::V1(_) => panic!(),
        }
    }

    fn list_in_variant2() -> Option<String> {
        Some("list_in_variant2".to_string())
    }

    fn list_in_variant3(a: imports::ListInVariant3) -> Option<String> {
        assert_eq!(a.unwrap(), "input3");
        Some("output3".to_string())
    }

    fn errno_result() -> Result<(), imports::MyErrno> {
        imports::MyErrno::A.to_string();
        format!("{:?}", imports::MyErrno::A);
        fn assert_error<T: std::error::Error>() {}
        assert_error::<imports::MyErrno>();
        Err(imports::MyErrno::B)
    }

    fn list_typedefs(
        a: imports::ListTypedef,
        b: imports::ListTypedef3,
    ) -> (imports::ListTypedef2, imports::ListTypedef3) {
        assert_eq!(a, "typedef1");
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], "typedef2");
        (b"typedef3".to_vec(), vec!["typedef4".to_string()])
    }

    fn list_of_variants(
        bools: Vec<bool>,
        results: Vec<Result<(), ()>>,
        enums: Vec<imports::MyErrno>,
    ) -> (Vec<bool>, Vec<Result<(), ()>>, Vec<imports::MyErrno>) {
        assert_eq!(bools, [true, false]);
        assert_eq!(results, [Ok(()), Err(())]);
        assert_eq!(enums, [imports::MyErrno::Success, imports::MyErrno::A]);
        (
            vec![false, true],
            vec![Err(()), Ok(())],
            vec![imports::MyErrno::A, imports::MyErrno::B],
        )
    }
}

wai_bindgen_rust::import!("./tests/runtime/flavorful/exports.wai");

fn run() -> Result<()> {
    use exports::*;

    exports::test_flavorful_imports();

    exports::list_in_record1(
        ListInRecord1 {
            a: "list_in_record1",
        },
    );
    assert_eq!(exports::list_in_record2().a, "list_in_record2");

    assert_eq!(
        exports
            ::list_in_record3(
                ListInRecord3Param {
                    a: "list_in_record3 input"
                }
            )
            .a,
        "list_in_record3 output"
    );

    assert_eq!(
        exports
            ::list_in_record4(ListInAliasParam { a: "input4" })
            .a,
        "result4"
    );

    exports::list_in_variant1(
        Some("foo"),
        Err("bar"),
        ListInVariant13::V0("baz"),
    );
    assert_eq!(
        exports::list_in_variant2(),
        Some("list_in_variant2".to_string())
    );
    assert_eq!(
        exports::list_in_variant3(Some("input3")),
        Some("output3".to_string())
    );

    assert!(exports::errno_result().is_err());
    MyErrno::A.to_string();
    format!("{:?}", MyErrno::A);
    fn assert_error<T: std::error::Error>() {}
    assert_error::<MyErrno>();

    let (a, b) = exports::list_typedefs("typedef1", &["typedef2"]);
    assert_eq!(a, b"typedef3");
    assert_eq!(b.len(), 1);
    assert_eq!(b[0], "typedef4");
    Ok(())
}
