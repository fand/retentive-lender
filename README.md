# retentive-lender

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

## Comparison with https://github.com/jdm/accountable-refcell

While developing this library I found a similary library named [`accountable-refcell`](https://github.com/jdm/accountable-refcell).
It's a thin wrapper of `RefCell` which stores the backtraces of the borrow calls and shows the backtraces on BorrowError / BorrowMutError.

While `accountable-refcell` is more informative than this crate, it only works when RUST_BACKTRACE is defined. This means we have to configure the build to include debug symbols, which is troublesome in some use cases.

Also, `retentive-lender` only provides limited APIs and the users cannot access the `RefCell` directly, so it will never panic.

## LICENSE

MIT
