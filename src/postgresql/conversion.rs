use postgres::{Row, types::Type};
use serde_json::{Value, Map, Number};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::IntoJson;

impl IntoJson for Vec<Row> {
    fn into_json(self) -> crate::Result<Value> {
        let mut result = Vec::new();

        for row in self.into_iter() {
            let mut object = Map::new();

            for (idx, column) in row.columns().iter().enumerate() {
                let column_name: String = column.name().into();

                let value = match *column.type_() {
                    Type::BOOL => match row.try_get(idx)? {
                        Some(val) => Value::Bool(val),
                        None => Value::Null,
                    },
                    Type::INT2 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i16 = val;
                            Value::Number(Number::from(val))
                        }
                        None => Value::Null,
                    },
                    Type::INT4 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i32 = val;
                            Value::Number(Number::from(val))
                        }
                        None => Value::Null,
                    },
                    Type::INT8 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i64 = val;
                            Value::Number(Number::from(val))
                        }
                        None => Value::Null,
                    },
                    Type::NUMERIC => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Decimal = val;
                            let val: f64 = val.to_string().parse().unwrap();
                            Value::Number(Number::from_f64(val).unwrap())
                        }
                        None => Value::Null,
                    },
                    Type::FLOAT4 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: f32 = val;
                            let val = f64::from(val);
                            Value::Number(Number::from_f64(val).unwrap())
                        }
                        None => Value::Null,
                    },
                    Type::FLOAT8 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: f64 = val;
                            Value::Number(Number::from_f64(val).unwrap())
                        }
                        None => Value::Null,
                    },
                    Type::TIMESTAMP => match row.try_get(idx)? {
                        Some(val) => {
                            let ts: NaiveDateTime = val;
                            let dt = DateTime::<Utc>::from_utc(ts, Utc);
                            Value::String(dt.to_rfc3339())
                        }
                        None => Value::Null,
                    },
                    Type::UUID => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Uuid = val;
                            Value::String(val.to_hyphenated().to_string())
                        }
                        None => Value::Null,
                    },
                    Type::INT2_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<i16> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| Value::Number(Number::from(x)))
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::INT4_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<i32> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| Value::Number(Number::from(x)))
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::INT8_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<i64> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| Value::Number(Number::from(x)))
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::FLOAT4_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<f32> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| {
                                        Value::Number(
                                            Number::from_f64(f64::from(x)).unwrap(),
                                        )
                                    })
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::FLOAT8_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<f64> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| Value::Number(Number::from_f64(x).unwrap()))
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::BOOL_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<bool> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| Value::Bool(x))
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::TIMESTAMP_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<NaiveDateTime> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| {
                                        let dt = DateTime::<Utc>::from_utc(x, Utc);
                                        Value::String(dt.to_rfc3339())
                                    })
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::NUMERIC_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<Decimal> = val;
                            Value::Array(
                                val.into_iter()
                                    .map(|x| {
                                        let val: f64 = x.to_string().parse().unwrap();
                                        Value::Number(
                                            Number::from_f64(val).unwrap(),
                                        )
                                    })
                                    .collect(),
                            )
                        }
                        None => Value::Null,
                    },
                    Type::TEXT_ARRAY | Type::NAME_ARRAY | Type::VARCHAR_ARRAY => {
                        match row.try_get(idx)? {
                            Some(val) => {
                                let val: Vec<String> = val;
                                Value::Array(
                                    val.into_iter()
                                        .map(|x| Value::String(x))
                                        .collect(),
                                )
                            }
                            None => Value::Null,
                        }
                    }
                    Type::OID => match row.try_get(idx)? {
                        Some(val) => {
                            let val: u32 = val;
                            Value::Number(Number::from(val))
                        }
                        None => Value::Null,
                    },
                    Type::CHAR => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i8 = val;
                            Value::String(val.to_string())
                        }
                        None => Value::Null,
                    },
                    _ => match row.try_get(idx)? {
                        Some(val) => {
                            let val: String = val;
                            Value::String(val)
                        }
                        None => Value::Null,
                    },
                };

                object.insert(column_name, value);
            }

            result.push(Value::Object(object));
        }

        Ok(Value::Array(result))
    }
}
