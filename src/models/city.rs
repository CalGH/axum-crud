use serde::{Deserialize, Serialize};
use crate::api::{get_one, post_put_or_delete_one};
use crate::Appstate;
use crate::QueryParams;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, Path, Query},
    http::StatusCode,
    Extension, Json,
};
use crud_macros::{DeleteOne, GetOne, PostOne, PutOne};
use postgres_from_row::FromRow;
use serde::de::{self, Visitor};
use serde_json::{json, Value};
use std::{error::Error, fmt::Display};

pub trait GetAllFields {
    fn get_all_fields(&self) -> Result<Vec<String>, CityError>;
}

#[derive(Debug)]
pub enum CityError {
    BadCountryValue,
}

impl Display for CityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad vector for Country")
    }
}

impl Error for CityError {}

struct CityVisitor {}

impl<'de> Visitor<'de> for CityVisitor {
    type Value = City;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Invalid input")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut city = City {
            ..Default::default()
        };

        while let Some(key) = map.next_key::<String>()? {
            if key.starts_with("country") {
                city.country = map.next_value::<String>()?.as_bytes().to_vec();
            } else {
                match key.as_str() {
                    "noname" => city.noname = map.next_value()?,
                    "population" => city.population = map.next_value()?,
                    "area" => {
                        city.area = {
                            let val = map.next_value::<String>()?;

                            let expected = "valid i32 as string in format 0x[0-9]+";
                            let stringarea: Result<&str, A::Error> =
                                val.strip_prefix("0x").ok_or_else(|| {
                                    de::Error::invalid_value(
                                        de::Unexpected::Other("not string or didnt start with 0x"),
                                        &expected,
                                    )
                                });

                            match stringarea {
                                Ok(_) => {}
                                Err(err) => return Err(err),
                            }

                            let areasigned: Result<i32, A::Error> =
                                <i32>::from_str_radix(stringarea?, 16).map_err(|_err| {
                                    de::Error::invalid_value(
                                        de::Unexpected::Other(
                                            format!(
                                                "value might be lt {} | gt {}",
                                                i32::MIN,
                                                i32::MAX
                                            )
                                            .as_str(),
                                        ),
                                        &expected,
                                    )
                                });


                            match areasigned {
                                Ok(val) => val,
                                Err(err) => return Err(err),
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(city)
    }
}

#[derive(FromRow, Serialize, Debug, Default, GetOne, PutOne, PostOne, DeleteOne)]
pub struct City {
    #[serde(skip_deserializing)]
    pub id: i32,
    #[from_row(rename = "name")]
    pub noname: String,
    #[from_row(from = "String")]
    pub country: Vec<u8>,
    pub population: i32,
    pub area: i32,
}

impl<'de> Deserialize<'de> for City {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(CityVisitor {})
    }
}

impl GetAllFields for City {
    fn get_all_fields(&self) -> Result<Vec<String>, CityError> {
        let country = String::from_utf8(self.country.clone()).map_err(|_| CityError::BadCountryValue)?;
        Ok(vec![
            self.noname.clone(),
            country,
            self.population.to_string(),
            self.area.to_string(),
        ])
    }
}

/*impl From<&Row> for City {
 *    fn from(row: &Row) -> Self {
 *        Self {
 *            name: row.get("name"),
 *            country: row.get("country"),
 *            population: row.get("population"),
 *            area: row.get("area")
 *        }
 *    }
 * }
 */
