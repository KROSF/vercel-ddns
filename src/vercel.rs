use eyre::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum VercelError {
    #[error("Unauthorized credentials. Check your Vercel token.")]
    Unauthorized,
    #[error("The domain {domain} could not be found on your account.")]
    MissingDomain { domain: String },
    #[error("Vercel Error {code}: {message}.")]
    Unknown { code: String, message: String },
}
#[derive(Debug, Deserialize)]
struct Root {
    pub pagination: Pagination,
    pub records: Vec<Record>,
}

#[derive(Debug, Deserialize)]
struct Pagination {
    pub count: i64,
    pub next: i64,
    pub prev: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub created: Option<i64>,
    pub created_at: Option<i64>,
    pub creator: Option<String>,
    pub id: Option<String>,
    pub name: String,
    pub slug: Option<String>,
    pub ttl: Option<i64>,
    #[serde(rename = "type")]
    pub dns_type: String,
    pub updated: Option<i64>,
    pub updated_at: Option<i64>,
    pub value: String,
}

impl Record {
    pub fn new(name: String, value: String, dns_type: String, ttl: i64) -> Self {
        Record {
            name,
            value,
            dns_type,
            ttl: Some(ttl),
            id: None,
            slug: None,
            creator: None,
            created: None,
            updated: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct VercelErrorResponse {
    pub error: VercelDNSError,
}

#[derive(Debug, Deserialize)]
struct VercelDNSError {
    pub code: String,
    pub message: String,
}

#[allow(dead_code)]
pub fn get_dns_records(domain: &str, token: &str) -> Result<Vec<Record>> {
    let uri = format!("https://api.vercel.com/v4/domains/{}/records", domain);
    let res = ureq::get(&uri)
        .set("Authorization", &format!("Bearer {}", token))
        .call();

    match res.status() {
        200 => Ok(serde_json::from_str::<Root>(&res.into_string()?)?.records),
        403 => Err(VercelError::Unauthorized {}.into()),
        404 => Err(VercelError::MissingDomain {
            domain: domain.to_string(),
        }
        .into()),
        _ => {
            let error = serde_json::from_str::<VercelErrorResponse>(&res.into_string()?)?.error;
            Err(VercelError::Unknown {
                code: error.code,
                message: error.message,
            }
            .into())
        }
    }
}

pub fn add_dns_record(domain: &str, token: &str, record: Record) -> Result<()> {
    let uri = format!("https://api.vercel.com/v2/domains/{}/records", domain);
    let mut req = ureq::post(&uri)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", token))
        .build();
    let res = req.send_string(&serde_json::to_string(&record)?);

    match res.status() {
        200 => Ok(()),
        403 => Err(VercelError::Unauthorized {}.into()),
        404 => Err(VercelError::MissingDomain {
            domain: domain.to_string(),
        }
        .into()),
        _ => {
            let error = serde_json::from_str::<VercelErrorResponse>(&res.into_string()?)?.error;
            Err(VercelError::Unknown {
                code: error.code,
                message: error.message,
            }
            .into())
        }
    }
}

#[allow(dead_code)]
pub fn delete_dns_record(domain: &str, token: &str, record: Record) -> Result<()> {
    let uri = format!(
        "https://api.vercel.com/v2/domains/{}/records/{}",
        domain,
        record.id.expect("Record did not have an ID.")
    );

    let res = ureq::get(&uri)
        .set("Authorization", &format!("Bearer {}", token))
        .call();

    match res.status() {
        200 => Ok(()),
        403 => Err(VercelError::Unauthorized {}.into()),
        404 => Err(VercelError::MissingDomain {
            domain: domain.to_string(),
        }
        .into()),
        _ => {
            let error = serde_json::from_str::<VercelErrorResponse>(&res.into_string()?)?.error;
            Err(VercelError::Unknown {
                code: error.code,
                message: error.message,
            }
            .into())
        }
    }
}
