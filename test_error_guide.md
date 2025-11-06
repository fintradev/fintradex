# How to Verify Errors Are Being Caught in service.rs

## Quick Test

Add this to the TOP of your `service.rs` file (after the module doc comment):

```rust
// This WILL cause a compilation error:
const TEST_ERROR: i32 = "not a number"; // Type mismatch error
```

## Then run:

```bash
cargo clean
cargo check --release
```

You should see an error message.

## Why Errors Might Not Be Caught

1. **Incremental Compilation**: Cargo caches compiled code. Use `cargo clean` first.

2. **Wrong Target**: Make sure you're building the right crate:
   ```bash
   cargo check --release -p fintradex-node
   ```

3. **Not Actual Compile Errors**:
   - Commenting out code = still valid Rust
   - Unused variables = warnings only (not errors)
   - Logic bugs = runtime errors (not compile errors)

## Types of "Errors" That Won't Prevent Compilation

- ❌ Unused functions (just warnings)
- ❌ Commented-out code
- ❌ Logic mistakes
- ❌ Runtime panics

## Types of "Errors" That WILL Prevent Compilation

- ✅ Syntax errors (`let x = y z;`)
- ✅ Type mismatches (`let x: i32 = "string";`)
- ✅ Undefined variables
- ✅ Missing semicolons
- ✅ `compile_error!()` macro calls

## Best Practice

Always use:
```bash
cargo check --release
```
Instead of:
```bash
cargo build --release
```
Because `check` is faster and catches compilation errors without building the final binary.


