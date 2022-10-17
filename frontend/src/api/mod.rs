pub mod requests;

fn get_api_url(path: String) -> String {
    return format!("http://127.0.0.1:8084/{}", path);
}
