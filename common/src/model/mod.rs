#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Child {
    pub name: String,
    pub family_name: String,
    pub birthday: chrono::NaiveDate,
}
