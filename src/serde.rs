use crate::components::{Speech, Timespan};
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;
use web_time::Duration;

impl Serialize for Timespan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Timespan", 1)?;
        state.serialize_field("elapsed", &self.elapsed().as_secs())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Timespan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Elapsed,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("`elapsed` or `start`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "elapsed" => Ok(Field::Elapsed),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct TimespanVisitor;

        impl<'de> Visitor<'de> for TimespanVisitor {
            type Value = Timespan;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Timespan")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Timespan, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut elapsed: Option<u64> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Elapsed => {
                            if elapsed.is_some() {
                                return Err(de::Error::duplicate_field("elapsed"));
                            }
                            elapsed = Some(map.next_value()?);
                        }
                    }
                }
                let mut timespan = Timespan::new();
                let elapsed = elapsed.ok_or_else(|| de::Error::missing_field("elapsed"))?;
                timespan.elapsed = Duration::new(elapsed, 0);

                Ok(timespan)
            }
        }

        const FIELDS: &[&str] = &["elapsed"];
        deserializer.deserialize_struct("Timespan", FIELDS, TimespanVisitor)
    }
}

impl Serialize for Speech {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Speech", 2)?;
        state.serialize_field("duration", &self.duration)?;
        state.serialize_field("category", &self.category)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Speech {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Duration,
            Category,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str("`duration` or `category`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "duration" => Ok(Field::Duration),
                            "category" => Ok(Field::Category),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SpeechVisitor;

        impl<'de> Visitor<'de> for SpeechVisitor {
            type Value = Speech;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct Speech")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Speech, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut duration = None;
                let mut category = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Duration => {
                            if duration.is_some() {
                                return Err(de::Error::duplicate_field("duration"));
                            }
                            duration = Some(map.next_value()?);
                        }
                        Field::Category => {
                            if category.is_some() {
                                return Err(de::Error::duplicate_field("speech"));
                            }
                            category = Some(map.next_value()?);
                        }
                    }
                }
                let duration = duration.ok_or_else(|| de::Error::missing_field("duration"))?;
                let category = category.ok_or_else(|| de::Error::missing_field("category"))?;
                let mut speech = Speech::new();
                speech.duration = duration;
                speech.category = category;
                Ok(speech)
            }
        }

        const FIELDS: &[&str] = &["duration", "category"];
        deserializer.deserialize_struct("Speech", FIELDS, SpeechVisitor)
    }
}
