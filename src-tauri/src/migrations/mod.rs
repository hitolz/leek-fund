use crate::errors::AppResult;

pub fn load_init_sql() -> AppResult<String> {
    Ok(include_str!("001_init.sql").to_string())
}
