use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
	let map: HashMap<String, String> = HashMap::new();
	let map = Arc::new(Mutex::new(map)); // shadow map
	let handle_map = map.clone();
	tokio::spawn(async move {
		handle_map.lock().unwrap().insert("test".into(), "value".into());
	}).await.expect("expected future to complete successfully");
	println!("{}", map.lock().unwrap().get("test").unwrap());
}
