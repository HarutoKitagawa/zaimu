use super::job;
use super::income;
mod db_dummy;
use db_dummy::DummyPartTimeJobRepo;

pub fn get_part_time_job_repo() -> impl job::PartTimeJobRepo {
    DummyPartTimeJobRepo::new()
}