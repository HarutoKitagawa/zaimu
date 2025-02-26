use crate::finance::api::plan::*;
use dioxus::prelude::*;

#[component]
pub fn PartTimeJobIncomes(
    year: Signal<i32>,
    month: Signal<u32>,
    part_time_job_incomes: Signal<Vec<PartTimeJobIncomeSchema>>,
    handle_edit_part_time_job: Callback<(u64, String, String, String, String)>,
) -> Element {
    rsx! {
        div {
            id: "job-list",
            table {
                colgroup {
                    col { style: "width: 25%;" }
                    col { style: "width: 12.5%;" }
                    col { style: "width: 12.5%;" }
                    col { style: "width: 25%;" }
                    col { style: "width: 25%;" }
                }
                thead {
                    tr {
                        th { "名前" }
                        th { "時給" }
                        th { "時間" }
                        th { "振込日" }
                        th { "合計" }
                    }
                }
                tbody {
                    for i in 0..part_time_job_incomes().len() {
                        tr {
                            td { "{part_time_job_incomes()[i].name}" }
                            td {
                                input {
                                    type: "number",
                                    value: part_time_job_incomes()[i].hourly_wage.to_string(),
                                    onchange: move |e| {
                                        let job = &part_time_job_incomes()[i];
                                        handle_edit_part_time_job((
                                            job.id,
                                            job.name.clone(),
                                            e.value(),
                                            job.hour.to_string(),
                                            job.payment_date.clone(),
                                        ))
                                    },
                                }
                            }
                            td {
                                input {
                                    type: "number",
                                    value: part_time_job_incomes()[i].hour.to_string(),
                                    onchange: move |e| {
                                        let job = &part_time_job_incomes()[i];
                                        handle_edit_part_time_job((
                                            job.id,
                                            job.name.clone(),
                                            job.hourly_wage.to_string(),
                                            e.value(),
                                            job.payment_date.clone(),
                                        ))
                                    },
                                }
                            }
                            td { "{part_time_job_incomes()[i].payment_date}" }
                            td {
                                class: "amount",
                                "{part_time_job_incomes()[i].total}"
                            }
                        }
                    }
                }
            }
        }
    }
}