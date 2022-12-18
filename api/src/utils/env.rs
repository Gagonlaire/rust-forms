use std::str::FromStr;

pub fn get_env_var_with_default<T: FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|p| p.parse::<T>().ok())
        .unwrap_or(default) as T
}

pub fn get_env_var(key: &str, fail_message: &str) -> String {
    std::env::var(key).expect(fail_message)
}
