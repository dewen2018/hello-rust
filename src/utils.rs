use mysql::Value;

// 添加辅助函数：将MySQL的Value类型转换为f64
pub fn value_to_f64(value: Value) -> f64 {
    match value {
        Value::Bytes(ref bytes) => {
            std::str::from_utf8(bytes)
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0)
        }
        Value::Int(i) => i as f64,
        Value::UInt(u) => u as f64,
        Value::Float(f) => f as f64,
        Value::Double(d) => d,
        _ => 0.0,
    }
}

// 添加辅助函数：将MySQL的Date类型转换为String
pub fn date_to_string(date: Value) -> String {
    match date {
        Value::Date(year, month, day, _, _, _, _) => {
            format!("{:04}-{:02}-{:02}", year, month, day)
        }
        _ => {
            if let Value::Bytes(ref bytes) = date {
                String::from_utf8_lossy(bytes).to_string()
            } else {
                format!("{:?}", date) // 将Value转换为调试字符串表示
            }
        }
    }
}