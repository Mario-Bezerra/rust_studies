use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;


#[test]
fn test_get_crates() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let response = client.get(format!("{}/crates", common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_crate(&client, b_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_crate() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    // Test
    let response = client.post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "foo",
        "name": "Foo crate",
        "version": "0.1",
        "description": "Foo crate description",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"],
    }));

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let response = client.get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "foo",
        "name": "Foo crate",
        "version": "0.1",
        "description": "Foo crate description",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"],
    }));

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "fooz",
        "name": "Fooz crate",
        "version": "0.2",
        "description": "Fooz crate description",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"],
    }));

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate_and_change_rustacean() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let rustacean_two = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean_two["id"],
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "fooz",
        "name": "Fooz crate",
        "version": "0.2",
        "description": "Fooz crate description",
        "rustacean_id": rustacean_two["id"],
        "created_at": a_crate["created_at"]
    }));

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let response = client.delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Cleanup
    common::delete_test_rustacean(&client, rustacean);
}