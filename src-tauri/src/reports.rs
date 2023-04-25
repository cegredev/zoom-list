use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use chrono::{Months, NaiveDateTime};
use rusqlite::{params, Connection};

use crate::{
    clients::{get_client, Client},
    config::Config,
};

const MONTHS_FULL: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

#[derive(Debug)]
struct ReportData {
    start: NaiveDateTime,
    end: NaiveDateTime,
    duration_m: i64,
}

#[derive(Debug)]
struct Report {
    client: Client,
    records: Vec<ReportData>,
}

fn get_records(
    conn: &Connection,
    client_id: i32,
    year: u32,
    month: u32,
) -> Result<Vec<ReportData>> {
    let range_start = NaiveDateTime::parse_from_str(
        format!("{year}-{month}-01 0:0:0").as_str(),
        "%Y-%m-%d %H:%M:%S",
    )
    .context("Could not generate start range date")?;
    let range_end = range_start
        .checked_add_months(Months::new(1))
        .context("Could not add month!")?;

    let mut statement =
        conn.prepare("SELECT start_ms, duration_m FROM records WHERE client_id=?1 AND start_ms BETWEEN ?2 AND ?3")?;

    let data: Vec<ReportData> = statement
        .query_map(
            params![
                client_id,
                range_start.timestamp_millis(),
                range_end.timestamp_millis()
            ],
            |row| {
                let start_ms: i64 = row.get(0)?;
                let duration_m: i64 = row.get(1)?;

                let start = NaiveDateTime::from_timestamp_millis(start_ms).expect("shit");
                let end = NaiveDateTime::from_timestamp_millis(start_ms + duration_m * 60 * 1000)
                    .expect("shit");

                Ok(ReportData {
                    start,
                    end,
                    duration_m,
                })
            },
        )
        .context("Could not create report data from SQL data.")?
        .filter_map(|value| match value {
            Ok(value) => Some(value),
            Err(_) => None,
        })
        .collect();

    Ok(data)
}

fn get_report_data(conn: &Connection, client_id: i32, year: u32, month: u32) -> Result<Report> {
    let client = get_client(&conn, client_id)?;
    let records = get_records(conn, client_id, year, month)?;

    Ok(Report { client, records })
}

fn write_report_to_file(file: &PathBuf, report: Report) -> Result<()> {
    todo!()
}

pub fn generate_report(
    conn: &Connection,
    config: &Config,
    client_id: i32,
    year: u32,
    month: u32,
) -> Result<String> {
    let report = get_report_data(conn, client_id, year, month).unwrap();

    println!("got report {:?}", report);

    let mut path: PathBuf = config.path.as_str().to_owned().into();
    if config.divide_by_year {
        path.push(format!("{year}"));

        if config.divide_by_month {
            path.push(format!("{}", MONTHS_FULL[month as usize - 1]));
        }
    }

    fs::create_dir_all(path.clone())?;

    path.push(format!("{}.pdf", report.client.name));

    println!("path: {:?}", path);

    Ok(path.to_str().unwrap().to_owned())
}