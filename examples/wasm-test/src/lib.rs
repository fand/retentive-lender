#![cfg(target_arch = "wasm32")]
extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use retentive_lender::Lender;

#[wasm_bindgen_test]
fn borrow_twice() -> Result<(), String> {
    let d = Lender::new(1);

    let b1 = d.borrow("b1")?;
    let b2 = d.borrow("b2")?;

    assert_eq!(*b1, 1);
    assert_eq!(*b2, 1);

    Ok(())
}

#[wasm_bindgen_test]
fn borrow_once_borrow_mut_once() -> Result<(), String> {
    let d = Lender::new(1);

    let _b1 = d.borrow("b1")?;
    let b2 = d.borrow_mut("b2");

    assert!(b2.is_err());
    if let Err(b) = b2 {
        b.contains("Currently borrowed by: [\"b1\"]");
        dbg!(b);
    }

    Ok(())
}

#[wasm_bindgen_test]
fn borrow_twice_borrow_mut_once() -> Result<(), String> {
    let d = Lender::new(1);

    let _b1 = d.borrow("b1")?;
    let _b2 = d.borrow("b2")?;
    let b3 = d.borrow_mut("b3");

    assert!(b3.is_err());
    if let Err(b) = b3 {
        b.contains("Currently borrowed by: [\"b1\", \"b2\"]");
    }

    Ok(())
}

#[wasm_bindgen_test]
fn borrow_mut_once_borrow_once() -> Result<(), String> {
    let d = Lender::new(1);

    let _b1 = d.borrow_mut("b1")?;
    let b2 = d.borrow("b2");

    assert!(b2.is_err());
    if let Err(b) = b2 {
        b.contains("Currently borrowed by: [\"b1\"]");
    }

    Ok(())
}

#[wasm_bindgen_test]
fn borrow_mut_twice() -> Result<(), String> {
    let d = Lender::new(1);

    let _b1 = d.borrow_mut("b1")?;
    let b2 = d.borrow_mut("b2");

    assert!(b2.is_err());
    if let Err(b) = b2 {
        b.contains("Currently borrowed by: [\"b1\"]");
    }

    Ok(())
}

#[wasm_bindgen_test]
fn clone_and_borrow_mut() -> Result<(), String> {
    let d = Lender::new(1);
    let _b1 = d.borrow_mut("b1")?;

    let _d2 = d.clone();
    let b2 = d.borrow_mut("b2");

    assert!(b2.is_err());
    if let Err(b) = b2 {
        b.contains("Currently borrowed by: [\"b1\"]");
    }

    Ok(())
}
