use libfj::robocraft::{AuthenticatedTokenProvider, AccountInfo};

pub fn parse_then_request_username(data: &str) -> Result<String, String> {
    let decoded_str = String::from_utf8(
        base64::decode(data).map_err(|_| "Invalid base64 encoding".to_string())?
    ).map_err(|_| "Invalid UTF-8 encoding".to_string())?;
    let decoded = decoded_str.split(":::").collect::<Vec<&str>>();
    if decoded.len() != 2 {
        return Err("Invalid data".to_string());
    }
    let info_maybe = request_account_info_username(decoded[0], decoded[1]);
    if let Ok(info) = info_maybe {
        return serde_json::to_string_pretty(&info).map_err(|_| "JSON encoding error".to_string());
    }
    Err("Invalid username or password".to_string())
}

pub fn request_account_info_username(username: &str, password: &str) -> Result<AccountInfo, ()> {
    let authenticator = AuthenticatedTokenProvider::with_username(username, password).map_err(|_|())?;
    authenticator.get_account_info().map_err(|_|())
}
