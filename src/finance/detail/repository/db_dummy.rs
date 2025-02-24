use chrono::{DateTime, Local};
use rust_decimal_macros::dec;
use std::vec;
use std::{cell::RefCell, collections::HashMap};

use crate::finance::detail::income::*;
use crate::finance::detail::outcome::*;
use crate::finance::detail::adjustment::*;
use crate::finance::detail::saving::*;

pub struct DummyIncomeRepo;

thread_local! {
    static INCOME_COLLECTION: RefCell<HashMap<u64, Income>> = RefCell::new(HashMap::from_iter(vec![
        (1, Income { id: Some(1), name: "Income 1".to_string(), amount: dec!(100000), date: Local::now() }),
        (2, Income { id: Some(2), name: "Income 2".to_string(), amount: dec!(50000), date: Local::now() }),
    ]));
    static OUTCOME_COLLECTION: RefCell<HashMap<u64, Outcome>> = RefCell::new(HashMap::from_iter(vec![
        (1, Outcome { id: Some(1), name: "Outcome 1".to_string(), amount: dec!(10000), date: Local::now() }),
        (2, Outcome { id: Some(2), name: "Outcome 2".to_string(), amount: dec!(5000), date: Local::now() }),
    ]));
    static ADJUSTMENT_COLLECTION: RefCell<HashMap<AdjustmentKey, Adjustment>> = RefCell::new(HashMap::from_iter(vec![]));
    static SAVING_COLLECTION: RefCell<HashMap<SavingKey, Saving>> = RefCell::new(HashMap::from_iter(vec![
        ((2025, 1), Saving { key: (2025, 1), amount: dec!(100000) }),
        ((2025, 2), Saving { key: (2025, 2), amount: dec!(235000) }),
    ]));
}

impl IncomeRepo for DummyIncomeRepo {
    fn list(&self, start_date: &DateTime<Local>, end_date: &DateTime<Local>) -> Result<Vec<Income>, anyhow::Error> {
        Ok(INCOME_COLLECTION.with(|collection| {
            collection
                .borrow()
                .clone()
                .into_iter()
                .map(|(_, income)| income)
                .filter(|income| income.date >= *start_date && income.date <= *end_date)
                .collect()
        }))
    }
    fn get_by_id(&self, id: u64) -> Result<Option<Income>, anyhow::Error> {
        Ok(INCOME_COLLECTION.with(|collection| collection.borrow().get(&id).cloned()))
    }
    fn store(&self, mut income: Income) -> Result<u64, anyhow::Error> {
        let id = INCOME_COLLECTION.with(|collection| {
            let id = collection.borrow().len() as u64 + 1;
            income.id = Some(id);
            collection.borrow_mut().insert(id, income);
            id
        });
        Ok(id)
    }
    fn update(&self, income: Income) -> Result<(), anyhow::Error> {
        INCOME_COLLECTION.with(|collection| {
            collection.borrow_mut().insert(income.id.unwrap(), income);
        });
        Ok(())
    }
    fn delete_by_id(&self, id: u64) -> Result<(), anyhow::Error> {
        INCOME_COLLECTION.with(|collection| {
            collection.borrow_mut().remove(&id);
        });
        Ok(())
    }
}

impl DummyIncomeRepo {
    pub fn new() -> Self {
        Self
    }
}

pub struct DummyOutcomeRepo;

impl OutcomeRepo for DummyOutcomeRepo {
    fn list(&self, start_date: &DateTime<Local>, end_date: &DateTime<Local>) -> Result<Vec<Outcome>, anyhow::Error> {
        Ok(OUTCOME_COLLECTION
            .with(|collection| collection.borrow().clone())
            .into_iter()
            .map(|(_, outcome)| outcome)
            .filter(|outcome| outcome.date >= *start_date && outcome.date <= *end_date)
            .collect())
    }
    fn get_by_id(&self, id: u64) -> Result<Option<Outcome>, anyhow::Error> {
        Ok(OUTCOME_COLLECTION.with(|collection| {
            collection
                .borrow()
                .iter()
                .find(|(_, outcome)| outcome.id == Some(id))
                .map(|(_, outcome)| outcome.clone())
        }))
    }
    fn store(&self, mut outcome: Outcome) -> Result<u64, anyhow::Error> {
        let id = OUTCOME_COLLECTION.with(|collection| {
            let id = collection.borrow().len() as u64 + 1;
            outcome.id = Some(id);
            collection.borrow_mut().insert(id, outcome);
            id
        });
        Ok(id)
    }
    fn update(&self, outcome: Outcome) -> Result<(), anyhow::Error> {
        OUTCOME_COLLECTION.with(|collection| {
            collection.borrow_mut().insert(outcome.id.unwrap(), outcome);
        });
        Ok(())
    }
    fn delete_by_id(&self, id: u64) -> Result<(), anyhow::Error> {
        OUTCOME_COLLECTION.with(|collection| {
            collection.borrow_mut().remove(&id);
        });
        Ok(())
    }
}

impl DummyOutcomeRepo {
    pub fn new() -> Self {
        Self
    }
}

pub struct DummyAdjustmentRepo;

impl AdjustmentRepo for DummyAdjustmentRepo {
    fn get(&self, key: &AdjustmentKey) -> Result<Option<Adjustment>, anyhow::Error> {
        Ok(ADJUSTMENT_COLLECTION.with(|collection| {
            collection.borrow().get(key).cloned()
        }))
    }
    fn store(&self, key: &AdjustmentKey, adjustment: Adjustment) -> Result<(), anyhow::Error> {
        ADJUSTMENT_COLLECTION.with(|collection| {
            collection.borrow_mut().insert(*key, adjustment);
        });
        Ok(())
    }
    fn delete(&self, key: &AdjustmentKey) -> Result<(), anyhow::Error> {
        ADJUSTMENT_COLLECTION.with(|collection| {
            collection.borrow_mut().remove(key);
        });
        Ok(())
    }
}

impl DummyAdjustmentRepo {
    pub fn new() -> Self {
        Self
    }
}

pub struct DummySavingRepo;

impl SavingRepo for DummySavingRepo {
    fn get(&self, key: &SavingKey) -> Result<Option<Saving>, anyhow::Error> {
        Ok(SAVING_COLLECTION.with(|collection| {
            collection.borrow().get(key).cloned()
        }))
    }
    fn store(&self, key: &SavingKey, saving: Saving) -> Result<(), anyhow::Error> {
        SAVING_COLLECTION.with(|collection| {
            collection.borrow_mut().insert(*key, saving);
        });
        Ok(())
    }
    fn update(&self, key: &SavingKey, saving: Saving) -> Result<(), anyhow::Error> {
        SAVING_COLLECTION.with(|collection| {
            collection.borrow_mut().insert(*key, saving);
        });
        Ok(())
    }
}

impl DummySavingRepo {
    pub fn new() -> Self {
        Self
    }
}