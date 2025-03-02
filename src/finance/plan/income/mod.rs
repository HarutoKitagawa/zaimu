pub mod income;
pub mod job;
pub mod temporary_income;

pub use income::{Income, IncomeRepo, ToIncome};
pub use income::get_incomes;