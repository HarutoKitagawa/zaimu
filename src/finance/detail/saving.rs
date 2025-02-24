use rust_decimal::Decimal;
use chrono::{Local, Datelike};

use crate::util::get_next_ym;

pub type SavingKey = (i32, u32);

#[derive(Debug, Clone)]
pub struct Saving {
    pub key: SavingKey,
    pub amount: Decimal,
}

impl Saving {
    pub fn new(key: SavingKey, amount: Decimal) -> Self {
        Self { key, amount }
    }
}

pub trait SavingRepo {
    fn get(&self, key: &SavingKey) -> Result<Option<Saving>, anyhow::Error>;
    fn store(&self, key: &SavingKey, saving: Saving) -> Result<(), anyhow::Error>;
    fn update(&self, key: &SavingKey, saving: Saving) -> Result<(), anyhow::Error>;
}

pub fn update_saving(
    key: SavingKey,
    amount: Decimal,
    saving_repo: &impl SavingRepo,
) -> Result<(), anyhow::Error> {
    let mut current_ym = key;
    let today = Local::now();
    while current_ym.0 < today.year() || (current_ym.0 == today.year() && current_ym.1 <= today.month()) {
        match saving_repo.get(&current_ym)? {
            Some(saving) => {
                let updated_saving: Saving = Saving::new(current_ym, saving.amount + amount);
                saving_repo.update(&current_ym, updated_saving)?
            },
            None => {
                match saving_repo.get(&(current_ym.0, if current_ym.1 == 1 { 12 } else { current_ym.1 - 1 }))? {
                    Some(last_month_saving) => {
                        let updated_saving = Saving::new(current_ym, last_month_saving.amount + amount);
                        saving_repo.store(&current_ym, updated_saving)?
                    },
                    None => {
                        let updated_saving = Saving::new(current_ym, amount);
                        saving_repo.store(&current_ym, updated_saving)?
                    },
                }
            },
        };
        current_ym = get_next_ym(current_ym);
    }
    Ok(())
}