extern crate firebase;

use firebase::Firebase;

fn main() {
    let david = Firebase::new("https://shining-torch-7752.firebaseio.com/users/david.json")
        .ok()
        .unwrap();

    let res = david
        .set(serde_json::json!({"firstName":"Dave"}))
        .ok()
        .unwrap();

    println!("Response body: {:?}", res.body);
    println!("Response code: {:?}", res.code);
    println!("Response success: {:?}", res.is_success());

    let res = david
        .update(serde_json::json!({"firstName":"David"}))
        .ok()
        .unwrap();

    println!("Response body: {:?}", res.body);
    println!("Response code: {:?}", res.code);
    println!("Response success: {:?}", res.is_success());
}
