use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use rusqlite::{Connection, params};
use serde::Serialize;

use crate::models::{Event, Filing, MissingItem, Observation, SourceRun};

pub struct Database {
    conn: Connection,
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub collection_batches: i64,
    pub source_runs: i64,
    pub observations: i64,
    pub filings: i64,
    pub events: i64,
    pub missing_items: i64,
}

#[derive(Debug, Serialize)]
pub struct RecentSourceRun {
    pub batch_id: Option<String>,
    pub profile: Option<String>,
    pub selector: Option<String>,
    pub source: String,
    pub status: String,
    pub http_status: Option<u16>,
    pub error: Option<String>,
    pub url: String,
    pub fetched_at: String,
}

#[derive(Debug, Serialize)]
pub struct RecentObservation {
    pub metric_code: String,
    pub metric_name: String,
    pub priority: String,
    pub category: String,
    pub value_num: Option<f64>,
    pub value_text: Option<String>,
    pub unit: String,
    pub observed_at: String,
    pub source: String,
    pub source_url: String,
    pub created_at: String,
    pub batch_id: Option<String>,
    pub profile: Option<String>,
    pub selector: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RecentFiling {
    pub company: String,
    pub form: String,
    pub filing_date: String,
    pub report_date: Option<String>,
    pub primary_doc: Option<String>,
    pub description: Option<String>,
    pub source_url: String,
}

#[derive(Debug, Serialize)]
pub struct RecentEvent {
    pub source: String,
    pub event_type: String,
    pub title: String,
    pub event_date: Option<String>,
    pub url: String,
    pub summary: Option<String>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS source_runs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                batch_id TEXT,
                profile TEXT,
                selector TEXT,
                source TEXT NOT NULL,
                url TEXT NOT NULL,
                fetched_at TEXT NOT NULL,
                status TEXT NOT NULL,
                http_status INTEGER,
                error TEXT,
                raw_excerpt TEXT
            );

            CREATE TABLE IF NOT EXISTS collection_batches (
                batch_id TEXT PRIMARY KEY,
                command TEXT NOT NULL,
                profile TEXT,
                selector TEXT,
                started_at TEXT NOT NULL,
                finished_at TEXT,
                ok_sources INTEGER,
                warn_sources INTEGER
            );

            CREATE TABLE IF NOT EXISTS observations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id INTEGER,
                metric_code TEXT NOT NULL,
                metric_name TEXT NOT NULL,
                priority TEXT NOT NULL,
                category TEXT NOT NULL,
                value_num REAL,
                value_text TEXT,
                unit TEXT NOT NULL,
                observed_at TEXT NOT NULL,
                source TEXT NOT NULL,
                source_url TEXT NOT NULL,
                attributes_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY(run_id) REFERENCES source_runs(id)
            );

            CREATE INDEX IF NOT EXISTS idx_observations_metric_time
                ON observations(metric_code, observed_at);

            CREATE TABLE IF NOT EXISTS missing_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                collector TEXT NOT NULL,
                metric_code TEXT NOT NULL,
                metric_name TEXT NOT NULL,
                priority TEXT NOT NULL,
                reason TEXT NOT NULL,
                source_hint TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(collector, metric_code)
            );

            CREATE TABLE IF NOT EXISTS filings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                company TEXT NOT NULL,
                cik TEXT NOT NULL,
                accession_no TEXT NOT NULL,
                form TEXT NOT NULL,
                filing_date TEXT NOT NULL,
                report_date TEXT,
                primary_doc TEXT,
                description TEXT,
                source_url TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(company, accession_no)
            );

            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source TEXT NOT NULL,
                event_type TEXT NOT NULL,
                title TEXT NOT NULL,
                event_date TEXT,
                url TEXT NOT NULL,
                summary TEXT,
                created_at TEXT NOT NULL,
                UNIQUE(source, title, event_date)
            );
            "#,
        )?;
        self.ensure_source_run_metadata_columns()?;
        Ok(())
    }

    fn ensure_source_run_metadata_columns(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(source_runs)")?;
        let columns = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        for (column, definition) in [
            ("batch_id", "TEXT"),
            ("profile", "TEXT"),
            ("selector", "TEXT"),
        ] {
            if !columns.iter().any(|existing| existing == column) {
                match self.conn.execute(
                    &format!("ALTER TABLE source_runs ADD COLUMN {column} {definition}"),
                    [],
                ) {
                    Ok(_) => {}
                    Err(error) if error.to_string().contains("duplicate column name") => {}
                    Err(error) => return Err(error.into()),
                }
            }
        }

        Ok(())
    }

    pub fn start_collection_batch(
        &self,
        batch_id: &str,
        command: &str,
        profile: Option<&str>,
        selector: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO collection_batches
                (batch_id, command, profile, selector, started_at)
            VALUES
                (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(batch_id) DO UPDATE SET
                command = excluded.command,
                profile = excluded.profile,
                selector = excluded.selector,
                started_at = excluded.started_at,
                finished_at = NULL,
                ok_sources = NULL,
                warn_sources = NULL
            "#,
            params![
                batch_id,
                command,
                profile,
                selector,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn finish_collection_batch(
        &self,
        batch_id: &str,
        ok_sources: usize,
        warn_sources: usize,
    ) -> Result<()> {
        self.conn.execute(
            r#"
            UPDATE collection_batches
            SET finished_at = ?2,
                ok_sources = ?3,
                warn_sources = ?4
            WHERE batch_id = ?1
            "#,
            params![
                batch_id,
                Utc::now().to_rfc3339(),
                ok_sources as i64,
                warn_sources as i64
            ],
        )?;
        Ok(())
    }

    pub fn insert_source_run(&self, run: &SourceRun) -> Result<i64> {
        self.conn.execute(
            r#"
            INSERT INTO source_runs
                (batch_id, profile, selector, source, url, fetched_at, status,
                 http_status, error, raw_excerpt)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
                run.batch_id,
                run.profile,
                run.selector,
                run.source,
                run.url,
                Utc::now().to_rfc3339(),
                run.status,
                run.http_status,
                run.error,
                run.raw_excerpt
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn insert_observation(&self, run_id: i64, obs: &Observation) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO observations
                (run_id, metric_code, metric_name, priority, category, value_num,
                 value_text, unit, observed_at, source, source_url, attributes_json, created_at)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            "#,
            params![
                run_id,
                obs.metric_code,
                obs.metric_name,
                obs.priority,
                obs.category,
                obs.value_num,
                obs.value_text,
                obs.unit,
                obs.observed_at,
                obs.source,
                obs.source_url,
                obs.attributes.to_string(),
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn upsert_missing_item(&self, item: &MissingItem) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO missing_items
                (collector, metric_code, metric_name, priority, reason, source_hint, created_at)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(collector, metric_code) DO UPDATE SET
                metric_name = excluded.metric_name,
                priority = excluded.priority,
                reason = excluded.reason,
                source_hint = excluded.source_hint,
                created_at = excluded.created_at
            "#,
            params![
                item.collector,
                item.metric_code,
                item.metric_name,
                item.priority,
                item.reason,
                item.source_hint,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn delete_missing_metric(&self, metric_code: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM missing_items WHERE metric_code = ?1",
            params![metric_code],
        )?;
        Ok(())
    }

    pub fn upsert_filing(&self, filing: &Filing) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO filings
                (company, cik, accession_no, form, filing_date, report_date,
                 primary_doc, description, source_url, created_at)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            ON CONFLICT(company, accession_no) DO UPDATE SET
                form = excluded.form,
                filing_date = excluded.filing_date,
                report_date = excluded.report_date,
                primary_doc = excluded.primary_doc,
                description = excluded.description,
                source_url = excluded.source_url,
                created_at = excluded.created_at
            "#,
            params![
                filing.company,
                filing.cik,
                filing.accession_no,
                filing.form,
                filing.filing_date,
                filing.report_date,
                filing.primary_doc,
                filing.description,
                filing.source_url,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn upsert_event(&self, event: &Event) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO events
                (source, event_type, title, event_date, url, summary, created_at)
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(source, title, event_date) DO UPDATE SET
                event_type = excluded.event_type,
                url = excluded.url,
                summary = excluded.summary,
                created_at = excluded.created_at
            "#,
            params![
                event.source,
                event.event_type,
                event.title,
                event.event_date,
                event.url,
                event.summary,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn summary(&self) -> Result<Summary> {
        Ok(Summary {
            collection_batches: self.count("collection_batches")?,
            source_runs: self.count("source_runs")?,
            observations: self.count("observations")?,
            filings: self.count("filings")?,
            events: self.count("events")?,
            missing_items: self.count("missing_items")?,
        })
    }

    pub fn missing_items(&self) -> Result<Vec<MissingItem>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT collector, metric_code, metric_name, priority, reason, source_hint
            FROM missing_items
            ORDER BY priority, metric_code
            "#,
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(MissingItem {
                collector: row.get(0)?,
                metric_code: row.get(1)?,
                metric_name: row.get(2)?,
                priority: row.get(3)?,
                reason: row.get(4)?,
                source_hint: row.get(5)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    pub fn recent_source_runs(&self, limit: usize) -> Result<Vec<RecentSourceRun>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT batch_id, profile, selector, source, status, http_status,
                   error, url, fetched_at
            FROM source_runs
            ORDER BY id DESC
            LIMIT ?1
            "#,
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(RecentSourceRun {
                batch_id: row.get(0)?,
                profile: row.get(1)?,
                selector: row.get(2)?,
                source: row.get(3)?,
                status: row.get(4)?,
                http_status: row.get(5)?,
                error: row.get(6)?,
                url: row.get(7)?,
                fetched_at: row.get(8)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    pub fn latest_observations(&self) -> Result<Vec<RecentObservation>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT o.metric_code, o.metric_name, o.priority, o.category, o.value_num,
                   o.value_text, o.unit, o.observed_at, o.source, o.source_url,
                   o.created_at, r.batch_id, r.profile, r.selector
            FROM observations o
            LEFT JOIN source_runs r ON r.id = o.run_id
            JOIN (
                SELECT metric_code, MAX(id) AS max_id
                FROM observations
                GROUP BY metric_code
            ) latest ON latest.max_id = o.id
            ORDER BY o.id DESC
            "#,
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(RecentObservation {
                metric_code: row.get(0)?,
                metric_name: row.get(1)?,
                priority: row.get(2)?,
                category: row.get(3)?,
                value_num: row.get(4)?,
                value_text: row.get(5)?,
                unit: row.get(6)?,
                observed_at: row.get(7)?,
                source: row.get(8)?,
                source_url: row.get(9)?,
                created_at: row.get(10)?,
                batch_id: row.get(11)?,
                profile: row.get(12)?,
                selector: row.get(13)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    pub fn recent_observations(&self, limit: usize) -> Result<Vec<RecentObservation>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT o.metric_code, o.metric_name, o.priority, o.category, o.value_num,
                   o.value_text, o.unit, o.observed_at, o.source, o.source_url,
                   o.created_at, r.batch_id, r.profile, r.selector
            FROM observations o
            LEFT JOIN source_runs r ON r.id = o.run_id
            ORDER BY o.id DESC
            LIMIT ?1
            "#,
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(RecentObservation {
                metric_code: row.get(0)?,
                metric_name: row.get(1)?,
                priority: row.get(2)?,
                category: row.get(3)?,
                value_num: row.get(4)?,
                value_text: row.get(5)?,
                unit: row.get(6)?,
                observed_at: row.get(7)?,
                source: row.get(8)?,
                source_url: row.get(9)?,
                created_at: row.get(10)?,
                batch_id: row.get(11)?,
                profile: row.get(12)?,
                selector: row.get(13)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    pub fn recent_filings(&self, limit: usize) -> Result<Vec<RecentFiling>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT company, form, filing_date, report_date, primary_doc,
                   description, source_url
            FROM filings
            ORDER BY filing_date DESC, id DESC
            LIMIT ?1
            "#,
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(RecentFiling {
                company: row.get(0)?,
                form: row.get(1)?,
                filing_date: row.get(2)?,
                report_date: row.get(3)?,
                primary_doc: row.get(4)?,
                description: row.get(5)?,
                source_url: row.get(6)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    pub fn recent_events(&self, limit: usize) -> Result<Vec<RecentEvent>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT source, event_type, title, event_date, url, summary
            FROM events
            ORDER BY COALESCE(event_date, '') DESC, id DESC
            LIMIT ?1
            "#,
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(RecentEvent {
                source: row.get(0)?,
                event_type: row.get(1)?,
                title: row.get(2)?,
                event_date: row.get(3)?,
                url: row.get(4)?,
                summary: row.get(5)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    fn count(&self, table: &str) -> Result<i64> {
        let sql = format!("SELECT COUNT(*) FROM {table}");
        self.conn
            .query_row(&sql, [], |row| row.get(0))
            .map_err(Into::into)
    }
}
