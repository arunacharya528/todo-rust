use chrono::{DateTime, Utc};

pub fn get_date_time_from_string(
    row: Result<String, rusqlite::Error>,
) -> Result<DateTime<Utc>, rusqlite::Error> {
    row.and_then(|last_modified| {
        DateTime::parse_from_rfc3339(&last_modified)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    4,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })
    })
}
