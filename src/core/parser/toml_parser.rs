pub fn read_from_toml<T>(txt: &str) -> Result<T, toml::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    toml::from_str::<T>(txt)
}
