pub fn toml_deserialize<T>(txt: &str) -> Result<T, toml::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    toml::from_str::<T>(txt)
}

pub fn toml_serialize<T>(object: &T) -> Result<String, toml::ser::Error>
where
    T: serde::Serialize,
{
    toml::to_string(object)
}
