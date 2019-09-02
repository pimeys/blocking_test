use super::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use postgres::{types::Type, NoTls, Row};
use r2d2_postgres::PostgresConnectionManager;
use rust_decimal::Decimal;
use std::sync::Arc;
use tokio_executor::blocking;
use uuid::Uuid;

pub trait IntoJson {
    fn into_json(self) -> crate::Result<serde_json::Value>;
}

pub struct Postgres {
    pool: Arc<r2d2::Pool<PostgresConnectionManager<NoTls>>>,
}

impl AsyncConnector for Postgres {
    fn new() -> Self {
        let manager = PostgresConnectionManager::new(
            "user = postgres host = localhost password = prisma"
                .parse()
                .unwrap(),
            NoTls,
        );

        let pool = Arc::new(r2d2::Pool::builder().build(manager).unwrap());

        Self { pool }
    }

    fn run(&self, query: String) -> FutureObj<'static, crate::Result<serde_json::Value>> {
        let pool = self.pool.clone();

        let fut = blocking::run(move || {
            let mut client = pool.get()?;
            let rows = client.query(query.as_str(), &[])?;

            Ok(rows.into_json()?)
        });

        FutureObj::new(Box::new(fut))
    }
}

impl IntoJson for Vec<Row> {
    fn into_json(self) -> crate::Result<serde_json::Value> {
        let mut result = Vec::new();

        for row in self.into_iter() {
            let mut object = serde_json::Map::new();

            for (idx, column) in row.columns().iter().enumerate() {
                let column_name: String = column.name().into();

                let value = match *column.type_() {
                    Type::BOOL => match row.try_get(idx)? {
                        Some(val) => serde_json::Value::Bool(val),
                        None => serde_json::Value::Null,
                    },
                    Type::INT2 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i16 = val;
                            serde_json::Value::Number(serde_json::Number::from(val))
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::INT4 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i32 = val;
                            serde_json::Value::Number(serde_json::Number::from(val))
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::INT8 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i64 = val;
                            serde_json::Value::Number(serde_json::Number::from(val))
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::NUMERIC => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Decimal = val;
                            let val: f64 = val.to_string().parse().unwrap();
                            serde_json::Value::Number(serde_json::Number::from_f64(val).unwrap())
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::FLOAT4 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: f32 = val;
                            let val = f64::from(val);
                            serde_json::Value::Number(serde_json::Number::from_f64(val).unwrap())
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::FLOAT8 => match row.try_get(idx)? {
                        Some(val) => {
                            let val: f64 = val;
                            serde_json::Value::Number(serde_json::Number::from_f64(val).unwrap())
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::TIMESTAMP => match row.try_get(idx)? {
                        Some(val) => {
                            let ts: NaiveDateTime = val;
                            let dt = DateTime::<Utc>::from_utc(ts, Utc);
                            serde_json::Value::String(dt.to_rfc3339())
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::UUID => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Uuid = val;
                            serde_json::Value::String(val.to_hyphenated().to_string())
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::INT2_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<i16> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| serde_json::Value::Number(serde_json::Number::from(x)))
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::INT4_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<i32> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| serde_json::Value::Number(serde_json::Number::from(x)))
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::INT8_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<i64> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| serde_json::Value::Number(serde_json::Number::from(x)))
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::FLOAT4_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<f32> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| {
                                        serde_json::Value::Number(
                                            serde_json::Number::from_f64(f64::from(x)).unwrap(),
                                        )
                                    })
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::FLOAT8_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<f64> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| serde_json::Value::Number(serde_json::Number::from_f64(x).unwrap()))
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::BOOL_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<bool> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| serde_json::Value::Bool(x))
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::TIMESTAMP_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<NaiveDateTime> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| {
                                        let dt = DateTime::<Utc>::from_utc(x, Utc);
                                        serde_json::Value::String(dt.to_rfc3339())
                                    })
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::NUMERIC_ARRAY => match row.try_get(idx)? {
                        Some(val) => {
                            let val: Vec<Decimal> = val;
                            serde_json::Value::Array(
                                val.into_iter()
                                    .map(|x| {
                                        let val: f64 = x.to_string().parse().unwrap();
                                        serde_json::Value::Number(
                                            serde_json::Number::from_f64(val).unwrap(),
                                        )
                                    })
                                    .collect(),
                            )
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::TEXT_ARRAY | Type::NAME_ARRAY | Type::VARCHAR_ARRAY => {
                        match row.try_get(idx)? {
                            Some(val) => {
                                let val: Vec<String> = val;
                                serde_json::Value::Array(
                                    val.into_iter()
                                        .map(|x| serde_json::Value::String(x))
                                        .collect(),
                                )
                            }
                            None => serde_json::Value::Null,
                        }
                    }
                    Type::OID => match row.try_get(idx)? {
                        Some(val) => {
                            let val: u32 = val;
                            serde_json::Value::Number(serde_json::Number::from(val))
                        }
                        None => serde_json::Value::Null,
                    },
                    Type::CHAR => match row.try_get(idx)? {
                        Some(val) => {
                            let val: i8 = val;
                            serde_json::Value::String(val.to_string())
                        }
                        None => serde_json::Value::Null,
                    },
                    _ => match row.try_get(idx)? {
                        Some(val) => {
                            let val: String = val;
                            serde_json::Value::String(val)
                        }
                        None => serde_json::Value::Null,
                    },
                };

                object.insert(column_name, value);
            }

            result.push(serde_json::Value::Object(object));
        }

        Ok(serde_json::Value::Array(result))
    }
}
