# DeserializeOrdered

A custom derive proc macro for Rust that allows structs to be deserialized from JSON arrays with explicit field ordering.

## Overview

`DeserializeOrdered` provides flexible deserialization from JSON arrays with explicit control over which array index maps to which struct field. This allows for:

- Deserializing a struct from a JSON array instead of the typical JSON object
- Explicitly controlling which array index maps to which struct field
- Out-of-order field mapping
- Handling sparse arrays (skipping irrelevant elements)
- Detailed error messages when deserialization fails

## Usage

Add the `DeserializeOrdered` trait to your struct and use the `#[order(n)]` attribute on each field to specify which JSON array index to use:

```rust
use json_array_derive::DeserializeOrdered;
use serde::Serialize;

// Basic usage - fields mapped to sequential array indices
#[derive(Debug, DeserializeOrdered, Serialize)]
struct Person {
    #[order(0)]
    id: i32,
    
    #[order(1)]
    name: String,
    
    #[order(2)]
    height: f64,
}

// Out-of-order mapping
#[derive(Debug, DeserializeOrdered, Serialize)]
struct OutOfOrderPerson {
    #[order(2)]
    id: i32,
    
    #[order(0)]
    name: String,
    
    #[order(1)]
    height: f64,
}

// Sparse mapping (skipping array elements)
#[derive(Debug, DeserializeOrdered, Serialize)]
struct SparsePerson {
    #[order(0)]
    id: i32,
    
    #[order(4)]
    name: String,
    
    #[order(1)]
    height: f64,
}
```

## Examples

```rust
// Deserialize from a JSON array with in-order indices
let json_array = r#"[1, "Jane Doe", 1.75]"#;
let person: Person = serde_json::from_str(json_array).unwrap();
println!("{:?}", person);
// Output: Person { id: 1, name: "Jane Doe", height: 1.75 }

// Deserialize with out-of-order indices
let json_array2 = r#"["Alice Smith", 1.65, 42]"#;
let out_of_order_person: OutOfOrderPerson = serde_json::from_str(json_array2).unwrap();
println!("{:?}", out_of_order_person);
// Output: OutOfOrderPerson { id: 42, name: "Alice Smith", height: 1.65 }

// Deserialize with sparse indices (skipping elements)
let json_array3 = r#"[99, 1.72, "ignore me", "also ignore", "Bob Johnson", "more to ignore"]"#;
let sparse_person: SparsePerson = serde_json::from_str(json_array3).unwrap();
println!("{:?}", sparse_person);
// Output: SparsePerson { id: 99, name: "Bob Johnson", height: 1.72 }
```

## Requirements

Every field in the struct must have an `#[order(n)]` attribute specifying which array index to use for deserialization.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
json-array-derive = { git = "https://github.com/manbango/ordered_derive" }
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
