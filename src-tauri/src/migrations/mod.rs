use crate::errors::AppResult;

pub fn load_migration_sqls() -> AppResult<Vec<String>> {
    Ok(vec![
        include_str!("001_init.sql").to_string(),
        include_str!("002_group_fund_positions.sql").to_string(),
    ])
}
