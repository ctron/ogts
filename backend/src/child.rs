use ogts_common::{model::Child, ListOptions};
use sqlx::types::chrono::NaiveDate;
use tokio::sync::Mutex;

pub struct ChildService {
    children: Mutex<Vec<Child>>,
}

impl ChildService {
    pub fn new() -> Self {
        let child = Child {
            name: "Heinz-Herbert".to_string(),
            family_name: "Mustermeier".to_string(),
            birthday: NaiveDate::from_ymd_opt(2016, 1, 1).unwrap_or(NaiveDate::MIN),
        };

        ChildService {
            children: Mutex::new(vec![child]),
        }
    }

    pub async fn create(&self, child: Child) -> anyhow::Result<()> {
        self.children.lock().await.push(child);
        Ok(())
    }

    pub async fn list(&self, opts: ListOptions) -> anyhow::Result<Vec<Child>> {
        Ok(self.children.lock().await.clone())
    }
}
