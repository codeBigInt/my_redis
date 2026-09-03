use crate::database::Database;

pub async fn command_parser(cmd: &str, db: &Database) -> Result<String, String> {
    let command_parts: Vec<&str> = cmd.split_whitespace().collect();

    match command_parts.as_slice() {
        ["SET", key, value, "EXP", ttl] => {
            let ttl = ttl.parse::<u64>().expect("Invalid ttl time provided");
            db.set(key.to_string(), value.to_string(), Some(ttl)).await;

            Ok("+OK\r\n".to_string())
        }

        ["SET", key, value] => {
            db.set(key.to_string(), value.to_string(), None).await;
            Ok("+OK\r\n".to_string())
        }

        ["GET", key] => {
            if let Some(value) = db.get(key).await {
                Ok(format!("${}\r\n{}\r\n", value.len(), value))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }

        ["DEL", key] => {
            if db.delete(key).await {
                Ok("${}-1\r\n".to_string())
            } else {
                Ok("$-0\r\n".to_string())
            }
        }

        _ => Ok("-ERR\r\nUnknown command, please try again!!".to_string()),
    }
}
