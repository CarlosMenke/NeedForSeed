pub mod requests;

fn get_api_url(path: String) -> String {
    return format!("https://mcmenke.de:8084/{}", path);
}
