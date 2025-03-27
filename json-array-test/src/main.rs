use json_array_derive::DeserializeOrdered;
use serde::{Deserialize, Serialize};

// Example struct with fields matching array indices
#[derive(Debug, DeserializeOrdered, Serialize)]
struct Person {
    #[order(0)]
    id: i32,
    
    #[order(1)]
    name: String,
    
    #[order(2)]
    height: f64,
}

// Example with out-of-order indices
#[derive(Debug, DeserializeOrdered, Serialize)]
struct OutOfOrderPerson {
    #[order(2)]
    id: i32,
    
    #[order(0)]
    name: String,
    
    #[order(1)]
    height: f64,
}

// Example with sparse indices (skipping some array elements)
#[derive(Debug, DeserializeOrdered, Serialize)]
struct SparsePerson {
    #[order(0)]
    id: i32,
    
    #[order(4)]
    name: String,
    
    #[order(1)]
    height: f64,
}

// For regular serde serialization/deserialization
#[derive(Debug, Deserialize, Serialize)]
struct PersonStandard {
    id: i32,
    name: String,
    height: f64,
}

fn main() {
    // Test case 1: Deserialize from a JSON object (standard way)
    let json_obj = r#"{"id": 1, "name": "John Doe", "height": 1.85}"#;
    let person_obj: PersonStandard = serde_json::from_str(json_obj).unwrap();
    println!("Deserialized from JSON object: {:?}", person_obj);
    
    // Test case 2: Deserialize from a JSON array with in-order indices
    let json_array = r#"[1, "Jane Doe", 1.75]"#;
    let person: Person = serde_json::from_str(json_array).unwrap();
    println!("Deserialized from JSON array (in-order): {:?}", person);
    
    // Test case 3: Deserialize from a JSON array with out-of-order indices
    let json_array2 = r#"["Alice Smith", 1.65, 42]"#;
    let out_of_order_person: OutOfOrderPerson = serde_json::from_str(json_array2).unwrap();
    println!("Deserialized from JSON array (out-of-order): {:?}", out_of_order_person);
    
    // Test case 4: Deserialize from a JSON array with sparse indices (extraneous elements)
    let json_array3 = r#"[99, 1.72, "ignore me", "also ignore", "Bob Johnson", "more to ignore"]"#;
    let sparse_person: SparsePerson = serde_json::from_str(json_array3).unwrap();
    println!("Deserialized from JSON array (sparse): {:?}", sparse_person);
}
