use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};


#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
                    .get("http://127.0.0.1:8000/rustaceans")
                    .send()
                    .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_post_rustacean() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/rustaceans")
                                    .json(&json!({
                                        "name": "Foo bar",
                                        "email": "foo@bar.com"
                                    }))
                                    .send()
                                    .unwrap();
    
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean : Value = response.json().unwrap();
    assert_eq!(rustacean , json!({
        "id" : rustacean["id"],
        "name" : "Foo bar",
        "email" : "foo@bar.com",
        "created_at" : rustacean["created_at"]
    }));
}