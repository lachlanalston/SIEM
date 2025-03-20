use chrono::Utc;
use std::env;

#[derive(Debug, Clone, Copy)]
enum Facility {
    Kernel = 0,
    User = 1,
    Mail = 2,
    Daemon = 3,
    Auth = 4,
    Syslog = 5,
    LPR = 6,
    News = 7,
    UUCP = 8,
    Clock = 9,
    AuthPriv = 10,
    FTP = 11,
    NTP = 12,
    LogAudit = 13,
    LogAlert = 14,
    Cron = 15,
    Local0 = 16,
    Local1 = 17,
    Local2 = 18,
    Local3 = 19,
    Local4 = 20,
    Local5 = 21,
    Local6 = 22,
    Local7 = 23,
}

impl Facility {
    fn from_str(facility: &str) -> Option<Self> {
        match facility.to_lowercase().as_str() {
            "kernel" => Some(Facility::Kernel),
            "user" => Some(Facility::User),
            "mail" => Some(Facility::Mail),
            "daemon" => Some(Facility::Daemon),
            "auth" => Some(Facility::Auth),
            "syslog" => Some(Facility::Syslog),
            "lpr" => Some(Facility::LPR),
            "news" => Some(Facility::News),
            "uucp" => Some(Facility::UUCP),
            "clock" => Some(Facility::Clock),
            "authpriv" => Some(Facility::AuthPriv),
            "ftp" => Some(Facility::FTP),
            "ntp" => Some(Facility::NTP),
            "logaudit" => Some(Facility::LogAudit),
            "logalert" => Some(Facility::LogAlert),
            "cron" => Some(Facility::Cron),
            "local0" => Some(Facility::Local0),
            "local1" => Some(Facility::Local1),
            "local2" => Some(Facility::Local2),
            "local3" => Some(Facility::Local3),
            "local4" => Some(Facility::Local4),
            "local5" => Some(Facility::Local5),
            "local6" => Some(Facility::Local6),
            "local7" => Some(Facility::Local7),
            _ => None,
        }
    }

    fn to_pri(self, severity: Severity) -> u8 {
        (self as u8) * 8 + (severity as u8)
    }
}

#[derive(Debug, Clone, Copy)]
enum Severity {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debug = 7,
}

impl Severity {
    fn from_str(severity: &str) -> Option<Self> {
        match severity.to_lowercase().as_str() {
            "emergency" => Some(Severity::Emergency),
            "alert" => Some(Severity::Alert),
            "critical" => Some(Severity::Critical),
            "error" => Some(Severity::Error),
            "warning" => Some(Severity::Warning),
            "notice" => Some(Severity::Notice),
            "informational" => Some(Severity::Informational),
            "debug" => Some(Severity::Debug),
            _ => None,
        }
    }
}

/// Get the current timestamp in RFC 3339 format
fn get_timestamp() -> String {
    Utc::now().to_rfc3339()
}

/// Get the hostname or default to "localhost"
fn get_hostname() -> String {
    env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string())
}

/// Log a syslog message with RFC 5424 formatting, parsing facility and severity from strings
fn log_message(facility_str: &str, severity_str: &str, app_name: &str, message: &str) {
    let facility = Facility::from_str(facility_str);
    let severity = Severity::from_str(severity_str);

    if let (Some(facility), Some(severity)) = (facility, severity) {
        let pri = facility.to_pri(severity);
        let timestamp = get_timestamp();
        let hostname = get_hostname();
        let process_id = std::process::id();

        let log_entry = format!(
            "<{}> {} {} {}[{}]: {}",
            pri, timestamp, hostname, app_name, process_id, message
        );

        println!("{}", log_entry);
    } else {
        eprintln!("Invalid facility ({}) or severity ({}).", facility_str, severity_str);
    }
}

fn main() {
    log_message("user", "warning", "MyApp", "This is a warning message.");
    log_message("daemon", "error", "MyApp", "This is an error message.");
    log_message("local0", "informational", "MyApp", "Informational log.");
}
