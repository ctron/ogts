use chrono::NaiveDate;
use ogts_common::model::Child;
use ogts_common::ListOptions;

pub struct ChildService;

impl ChildService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(&self, child: Child) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn list(&self, opts: ListOptions) -> anyhow::Result<Vec<Child>> {
        Ok(vec![Child {
            name: "Heinz-Herbert".to_string(),
            family_name: "Mustermeier".to_string(),
            birthday: NaiveDate::from_ymd_opt(2016, 1, 1).unwrap_or(NaiveDate::MIN),
        }])
    }
}
