use serde::{Deserialize, de};

pub enum CompressionLevel {
    Fast,
    Best
}

impl<'de> Deserialize<'de> for CompressionLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CompressionLevelVisitor;

        impl<'de> de::Visitor<'de> for CompressionLevelVisitor {
            type Value = CompressionLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`variant1-rename`, `variant2-rename`, or some other string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "fast" => Ok(CompressionLevel::Fast),
                    "best" => Ok(CompressionLevel::Best),
                    _ => panic!("unknown value \"{}\" for compression level", v),
                }
            }
        }

        deserializer.deserialize_identifier(CompressionLevelVisitor)
    }
}
