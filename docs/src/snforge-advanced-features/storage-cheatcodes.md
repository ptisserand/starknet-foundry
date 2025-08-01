# Direct Storage Access

In some instances, it's not possible for contracts to expose API that we'd like to use in order to initialize
the contracts before running some tests. For those cases `snforge` exposes storage-related cheatcodes,
which allow manipulating the storage directly (reading and writing).

In order to obtain the variable address that you'd like to write to, or read from, you need to use either:
- `selector!` macro - if the variable is not a mapping
- `map_entry_address` function in tandem with `selector!` - for key-value pair of a map variable
- `starknet::storage_access::storage_address_from_base`

> 📝 **Note**
>
> For more user friendly access to storage, consider using the [`interact_with_state`](../appendix/cheatcodes/interact_with_state.md) cheatcode.

## Example: Felt-only storage
This example uses only felts for simplicity.

1. Exact storage fields

```rust
{{#include ../../listings/direct_storage_access/tests/felts_only/field.cairo}}
```

2. Map entries

```rust
{{#include ../../listings/direct_storage_access/tests/felts_only/map_entry.cairo}}
```

## Example: Complex structures in storage
This example uses a complex key and value, with default derived serialization methods (via `#[derive(starknet::Store)]`).

We use a contract along with helper structs:

```rust
{{#include ../../listings/direct_storage_access/src/complex_structures.cairo}}
```

And perform a test checking `load` and `store` behavior in context of those structs:

```rust
{{#include ../../listings/direct_storage_access/tests/complex_structures.cairo}}
```

> ⚠️ **Warning**
>
> Complex data can often times be packed in a custom manner (see [this pattern](https://book.cairo-lang.org/ch16-01-optimizing-storage-costs.html)) to optimize costs.
> If that's the case for your contract, make sure to handle deserialization properly - standard methods might not work.
> **Use those cheatcode as a last-resort, for cases that cannot be handled via contract's API!**

## Example: Using enums in storage

Enums use 0-based layout for serialization. For example, `FirstVariantOfSomeEnum(100)` will be serialized as `[0, 100]`. However, their Starknet storage layout is 1-based for most enums, especially for these with derived `Store` trait implementation. Therefore, `FirstVariantOfSomeEnum(100)` will be stored on Starknet as `[1, 100]`. 

Remember that this rule may not hold for enums that with manual `Store` trait implementation. The most notable example is `Option`, e.g. `Option::None` will be stored as `[0]` and `Option::Some(100)` will be stored as `[1, 100]`.

Below is an example of a contract which can store `Option<u256>` values:

```rust
{{#include ../../listings/direct_storage_access/src/using_enums.cairo}}
```

And a test which uses `store` and reads the value:

```rust
{{#include ../../listings/direct_storage_access/tests/using_enums.cairo}}
```

```shell
snforge test test_store_and_read
```

<details>
<summary>Output:</summary>

```shell
Collected 1 test(s) from direct_storage_access package
Running 1 test(s) from tests/
[PASS] direct_storage_access_tests::using_enums::test_store_and_read (gas: ~233)
Running 0 test(s) from src/
Tests: 1 passed, 0 failed, 0 ignored, 4 filtered out
```

</details>

> 📝 **Note**
>
> The `load` cheatcode will return zeros for memory you haven't written into yet (it is a default storage value for Starknet contracts' storage).

## Example with `storage_address_from_base`
This example uses `storage_address_from_base` with entry's of the storage variable.

```rust
{{#include ../../listings/direct_storage_access/tests/using_storage_address_from_base.cairo}}
```