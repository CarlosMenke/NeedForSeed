pub mod requests;

fn get_api_url(path: String) -> String {
    return format!("https://mcmenke.de:8084/{}", path);
    //return format!("https://127.0.0.1:8084/{}", path);
}
