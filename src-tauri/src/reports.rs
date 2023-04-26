use std::{
    fmt::write,
    fs::{self},
    path::PathBuf,
};

use anyhow::{Context, Result};
use chrono::{DateTime, Months, NaiveDateTime};
use headless_chrome::{types::PrintToPdfOptions, Browser};
use rand::{thread_rng, Rng};
use rusqlite::{params, Connection};
use serde::Serialize;
use tauri::PathResolver;

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReportDataPretty {
    start: String,
    end: String,
    duration_minutes: i64,
}

impl ReportData {
    fn pretty(self: &Self) -> ReportDataPretty {
        let fmt = "%d.%m.%Y %H:%M";

        ReportDataPretty {
            start: self.start.format(fmt).to_string(),
            end: self.end.format(fmt).to_string(),
            duration_minutes: self.duration_m,
        }
    }
}

#[derive(Debug)]
struct Report {
    client: Client,
    month_index: usize,
    records: Vec<ReportData>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReportPretty {
    client_name: String,
    month_pretty: String,
    records: Vec<ReportDataPretty>,
}

impl Report {
    fn pretty(self: &Self) -> ReportPretty {
        ReportPretty {
            client_name: self.client.name.clone(),
            month_pretty: MONTHS_FULL[self.month_index].to_string(),
            records: self.records.iter().map(|record| record.pretty()).collect(),
        }
    }
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

    Ok(Report {
        client,
        month_index: month as usize - 1,
        records,
    })
}

fn generate_report_html(path_resolver: &PathResolver, report: Report) -> Result<String> {
    let mut rng = thread_rng();

    let mut out_html_file = std::env::temp_dir();
    out_html_file.push(format!("zoom-list_{}.html", rng.gen_range(0..100000)));

    let template = fs::read_to_string(
        path_resolver
            .resolve_resource("resources/report_template.html")
            .context("Could not get template")?,
    )?;

    let reg = handlebars::Handlebars::new();
    let content = reg.render_template(template.as_str(), &report.pretty())?;

    fs::write(out_html_file.clone(), content)?;

    println!("html: {:?}", out_html_file);

    Ok(out_html_file
        .to_str()
        .context("Could not turn path into string")?
        .to_owned())
}

fn write_report_to_file(
    file: &PathBuf,
    path_resolver: &PathResolver,
    report: Report,
) -> Result<()> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.navigate_to(generate_report_html(path_resolver, report)?.as_str())?;
    tab.wait_until_navigated()?;

    let mut options = PrintToPdfOptions::default();
    options.landscape = Some(false);
    options.print_background = Some(true);

    let bytes = tab.print_to_pdf(Some(options))?;

    fs::write(file, bytes)?;

    Ok(())
}

pub fn generate_report(
    conn: &Connection,
    config: &Config,
    path_resolver: &PathResolver,
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

    write_report_to_file(&path, path_resolver, report)?;

    Ok(path.to_str().unwrap().to_owned())
}
