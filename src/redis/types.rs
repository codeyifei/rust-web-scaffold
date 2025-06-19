use redis::ToRedisArgs;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisValue<T>(T);

impl<T> RedisValue<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> ToRedisArgs for RedisValue<T>
where
    T: Serialize,
{
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        let json = serde_json::to_string(&self.0).unwrap();
        out.write_arg(json.as_bytes());
    }
}

// impl<T> FromRedisValue for RedisValue<T>
// where
//     T: DeserializeOwned,
// {
//     fn from_redis_value(v: &Value) -> RedisResult<Self> {
//         let value: Result<serde_json::Value, &'static str> = match v {
//             Value::Nil => Ok(serde_json::Value::Null),
//             Value::Int(num) => Ok(serde_json::Value::Number((*num).into())),
//             Value::BulkString(str) => match String::from_utf8_lossy(str).into_owned().parse() {
//                 Ok(str) => Ok(str),
//                 Err(_) => Err("invalid string"),
//             },
//             Value::Array(_array) => Ok(serde_json::Value::Array(vec![
//                 serde_json::Value::String(String::from("test")),
//                 serde_json::Value::String(String::from("123")),
//             ])),
//             Value::SimpleString(str) => Ok(serde_json::Value::String(str.clone())),
//             Value::Okay => Ok(serde_json::Value::String("OK".into())),
//             Value::Map(_map) => Err("这是一个map"),
//             // Value::Attribute { .. } => {}
//             // Value::Set(_) => {}
//             Value::Double(num) => Ok(json!(*num)),
//             Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
//             // Value::VerbatimString { .. } => {}
//             Value::BigNumber(num) => Ok(serde_json::Value::String(num.to_string())),
//             // Value::Push { .. } => {}
//             // Value::ServerError(_) => {}
//             _ => Err("非法的数据类型"),
//         };
//         let value =
//             value.or_else(|msg| Err::<_, RedisError>((ErrorKind::ParseError, msg).into()))?;
//         println!("{}", value);
//         match serde_json::from_value(value) {
//             Ok(value) => Ok(RedisValue(value)),
//             Err(_err) => Err((ErrorKind::ParseError, "反序列化失败").into()),
//         }
//     }
// }

impl<T> Deref for RedisValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
