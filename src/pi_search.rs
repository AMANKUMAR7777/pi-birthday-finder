use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub pattern: String,
    pub position: u64,
    pub before_context: String,
    pub matched: String,
    pub after_context: String,
    pub count: u64,
    pub query_time: Duration,
    pub source: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DateFormat {
    DDMMYYYY, // 25052009
    MMDDYYYY, // 05252009
    YYYYMMDD, // 20090525
    DDMMYY,   // 250509
    MMDDYY,   // 052509
    DDMM,     // 2505
}

impl DateFormat {
    pub fn all() -> &'static [DateFormat] {
        &[
            DateFormat::DDMMYYYY,
            DateFormat::MMDDYYYY,
            DateFormat::YYYYMMDD,
            DateFormat::DDMMYY,
            DateFormat::MMDDYY,
            DateFormat::DDMM,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            DateFormat::DDMMYYYY => "DD/MM/YYYY (e.g. 25052009)",
            DateFormat::MMDDYYYY => "MM/DD/YYYY (e.g. 05252009)",
            DateFormat::YYYYMMDD => "YYYY/MM/DD (e.g. 20090525)",
            DateFormat::DDMMYY => "DD/MM/YY   (e.g. 250509)",
            DateFormat::MMDDYY => "MM/DD/YY   (e.g. 052509)",
            DateFormat::DDMM => "DD/MM      (e.g. 2505)",
        }
    }

    pub fn format(&self, day: u32, month: u32, year: i32) -> String {
        let yy = (year.abs() % 100) as u32;
        match self {
            DateFormat::DDMMYYYY => format!("{:02}{:02}{:04}", day, month, year),
            DateFormat::MMDDYYYY => format!("{:02}{:02}{:04}", month, day, year),
            DateFormat::YYYYMMDD => format!("{:04}{:02}{:02}", year, month, day),
            DateFormat::DDMMYY => format!("{:02}{:02}{:02}", day, month, yy),
            DateFormat::MMDDYY => format!("{:02}{:02}{:02}", month, day, yy),
            DateFormat::DDMM => format!("{:02}{:02}", day, month),
        }
    }
}

#[derive(Deserialize)]
struct AngioResponse {
    r: Option<Vec<AngioResultItem>>,
}

#[derive(Deserialize)]
struct AngioResultItem {
    k: Option<String>,
    p: Option<u64>,
    db: Option<String>,
    da: Option<String>,
    c: Option<u64>,
    status: Option<String>,
}

/// Query the 200-million digit Pi index via backend service
pub fn query_pi_online(query: &str) -> Result<SearchResult, String> {
    let start = Instant::now();
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .map_err(|e| e.to_string())?;

    let body_str = format!("q={}", query);
    let res = client
        .post("https://www.angio.net/newpi/piquery")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body_str)
        .send()
        .map_err(|e| format!("Network request failed: {}", e))?;

    let angio: AngioResponse = res
        .json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(items) = angio.r {
        if let Some(item) = items.into_iter().next() {
            if item.status.as_deref() == Some("notfound") || item.p.is_none() {
                return Err(format!("Sequence \"{}\" not found in first 200 million digits.", query));
            }

            return Ok(SearchResult {
                pattern: item.k.unwrap_or_else(|| query.to_string()),
                position: item.p.unwrap_or(0),
                before_context: item.db.unwrap_or_default(),
                matched: query.to_string(),
                after_context: item.da.unwrap_or_default(),
                count: item.c.unwrap_or(1),
                query_time: start.elapsed(),
                source: "Pi 200M Index Engine".to_string(),
            });
        }
    }

    Err(format!("Pattern \"{}\" not found in search range.", query))
}

/// Fallback embedded first 10,000 digits of Pi for instant offline lookup
pub const FIRST_10000_DIGITS: &str = include_str!("pi_10k.txt");

pub fn query_pi_offline(query: &str) -> Option<SearchResult> {
    let start = Instant::now();
    if let Some(pos) = FIRST_10000_DIGITS.find(query) {
        let start_ctx = pos.saturating_sub(20);
        let end_ctx = (pos + query.len() + 20).min(FIRST_10000_DIGITS.len());

        let before = &FIRST_10000_DIGITS[start_ctx..pos];
        let after = &FIRST_10000_DIGITS[pos + query.len()..end_ctx];

        Some(SearchResult {
            pattern: query.to_string(),
            position: pos as u64 + 1, // 1-indexed after 3.
            before_context: before.to_string(),
            matched: query.to_string(),
            after_context: after.to_string(),
            count: 1,
            query_time: start.elapsed(),
            source: "Local Offline Pi Cache (10k digits)".to_string(),
        })
    } else {
        None
    }
}
