# retentive-lender

This crate provides `retentive_lender::Lender` which wraps `Rc<RefCell<T>>` for the interior mutability pattern. It also keeps the name of the borrower until the value is dropped, and shows the borrower information when it violates the  Rust's borrow rule.

## Install

```
cargo add retentive-lender
```

## Usage

```rust
use retentive_lender::Lender;

{
    let data = Lender::new(1);

    let borrow1 = data.borrow("borrow 1")?;
    let borrow2 = data.borrow_mut("borrow 2");
    assert!(borrow2, Err(r#"Failed to borrow mutable reference. Currently borrowed by: ["borrow 1"]"#.to_string()));
}

{
    let data = Lender::new(1);

    let borrow1 = data.borrow_mut("borrow 1")?;
    let borrow2 = data.borrow("borrow 2");

    assert!(borrow2, Err(r#"Failed to borrow immutable reference. Currently borrowed by: ["borrow 1 (mut)"]"#.to_string()));
}

{
    let data = Lender::new(1);

    let borrow1 = data.borrow("borrow 1")?;
    let borrow2 = data.borrow("borrow 2")?;
    let borrow3 = data.borrow_mut("borrow 3");

    assert!(borrow3, Err(r#"Failed to borrow mutable reference. Currently borrowed by: ["borrow 1", "borrow 2"]"#.to_string()));
}
```

## Comparison with `debug_cell` and `accountable-refcell`

While developing this library I found a similary library named [`debug_cell`](https://github.com/alexcrichton/debug-cell) [`accountable-refcell`](https://github.com/jdm/accountable-refcell).
They are thin wrappers of `RefCell` which store the backtraces of the borrow calls and shows the backtraces on BorrowError / BorrowMutError.

While these libraries are pretty useful, they only work when RUST_BACKTRACE is defined. This means we have to configure the build to include debug symbols, which is troublesome in some use cases, especially on WebAssembly.

Also, `retentive-lender` only provides limited APIs and the users cannot access the `RefCell` directly, so it will never panic.

## LICENSE

MIT
