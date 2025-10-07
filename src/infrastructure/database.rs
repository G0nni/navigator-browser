use crate::domain::{
    Bookmark, BookmarkRepository, HistoryEntry, HistoryRepository, Tab, TabId, TabRepository,
    ValidatedUrl,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

/// SQLite-based implementation of repositories
pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(database_path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_path)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .context("Failed to connect to database")?;

        // Run migrations
        Self::create_tables(&pool).await?;

        Ok(Self { pool })
    }

    async fn create_tables(pool: &SqlitePool) -> Result<()> {
        // Create tabs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tabs (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                url TEXT,
                is_private BOOLEAN NOT NULL,
                created_at TEXT NOT NULL,
                last_accessed TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create bookmarks table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                folder TEXT,
                created_at TEXT NOT NULL,
                tags TEXT
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create history table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                visited_at TEXT NOT NULL,
                visit_count INTEGER NOT NULL DEFAULT 1
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create indices for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_history_visited_at ON history(visited_at DESC)")
            .execute(pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookmarks_folder ON bookmarks(folder)")
            .execute(pool)
            .await?;

        Ok(())
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// Implement TabRepository
#[async_trait]
impl TabRepository for SqliteDatabase {
    async fn save(&self, tab: &Tab) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO tabs (id, title, url, is_private, created_at, last_accessed)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(tab.id.to_string())
        .bind(&tab.title)
        .bind(tab.url.as_ref().map(|u| u.as_str()))
        .bind(tab.is_private)
        .bind(tab.created_at.to_rfc3339())
        .bind(tab.last_accessed.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: TabId) -> Result<Option<Tab>> {
        let result = sqlx::query_as::<_, (String, String, Option<String>, bool, String, String)>(
            "SELECT id, title, url, is_private, created_at, last_accessed FROM tabs WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(_id_str, title, url, is_private, created_at, last_accessed)| {
            Tab {
                id: TabId::new(), // Parse from string in production
                title,
                url: url.and_then(|u| ValidatedUrl::parse(&u).ok()),
                is_loading: false,
                is_private,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                last_accessed: chrono::DateTime::parse_from_rfc3339(&last_accessed)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                favicon_url: None,
            }
        }))
    }

    async fn find_all(&self) -> Result<Vec<Tab>> {
        let results = sqlx::query_as::<_, (String, String, Option<String>, bool, String, String)>(
            "SELECT id, title, url, is_private, created_at, last_accessed FROM tabs
             ORDER BY last_accessed DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(_id_str, title, url, is_private, created_at, last_accessed)| Tab {
                id: TabId::new(),
                title,
                url: url.and_then(|u| ValidatedUrl::parse(&u).ok()),
                is_loading: false,
                is_private,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                last_accessed: chrono::DateTime::parse_from_rfc3339(&last_accessed)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                favicon_url: None,
            })
            .collect())
    }

    async fn delete(&self, id: TabId) -> Result<()> {
        sqlx::query("DELETE FROM tabs WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn save_session(&self, tabs: Vec<Tab>) -> Result<()> {
        for tab in tabs {
            TabRepository::save(self, &tab).await?;
        }
        Ok(())
    }

    async fn restore_session(&self) -> Result<Vec<Tab>> {
        TabRepository::find_all(self).await
    }
}

// Implement BookmarkRepository
#[async_trait]
impl BookmarkRepository for SqliteDatabase {
    async fn save(&self, bookmark: &Bookmark) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO bookmarks (title, url, folder, created_at, tags)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&bookmark.title)
        .bind(bookmark.url.as_str())
        .bind(&bookmark.folder)
        .bind(bookmark.created_at.to_rfc3339())
        .bind(serde_json::to_string(&bookmark.tags)?)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Bookmark>> {
        let result = sqlx::query_as::<_, (i64, String, String, Option<String>, String, String)>(
            "SELECT id, title, url, folder, created_at, tags FROM bookmarks WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.and_then(|(id, title, url, folder, created_at, tags)| {
            ValidatedUrl::parse(&url).ok().map(|url| Bookmark {
                id,
                title,
                url,
                folder,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                tags: serde_json::from_str(&tags).unwrap_or_default(),
            })
        }))
    }

    async fn find_all(&self) -> Result<Vec<Bookmark>> {
        let results = sqlx::query_as::<_, (i64, String, String, Option<String>, String, String)>(
            "SELECT id, title, url, folder, created_at, tags FROM bookmarks
             ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .filter_map(|(id, title, url, folder, created_at, tags)| {
                ValidatedUrl::parse(&url).ok().map(|url| Bookmark {
                    id,
                    title,
                    url,
                    folder,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    tags: serde_json::from_str(&tags).unwrap_or_default(),
                })
            })
            .collect())
    }

    async fn find_by_folder(&self, folder: &str) -> Result<Vec<Bookmark>> {
        let results = sqlx::query_as::<_, (i64, String, String, Option<String>, String, String)>(
            "SELECT id, title, url, folder, created_at, tags FROM bookmarks
             WHERE folder = ? ORDER BY created_at DESC",
        )
        .bind(folder)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .filter_map(|(id, title, url, folder, created_at, tags)| {
                ValidatedUrl::parse(&url).ok().map(|url| Bookmark {
                    id,
                    title,
                    url,
                    folder,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    tags: serde_json::from_str(&tags).unwrap_or_default(),
                })
            })
            .collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Bookmark>> {
        let search_pattern = format!("%{}%", query);
        let results = sqlx::query_as::<_, (i64, String, String, Option<String>, String, String)>(
            "SELECT id, title, url, folder, created_at, tags FROM bookmarks
             WHERE title LIKE ? OR url LIKE ? ORDER BY created_at DESC",
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .filter_map(|(id, title, url, folder, created_at, tags)| {
                ValidatedUrl::parse(&url).ok().map(|url| Bookmark {
                    id,
                    title,
                    url,
                    folder,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    tags: serde_json::from_str(&tags).unwrap_or_default(),
                })
            })
            .collect())
    }

    async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM bookmarks WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update(&self, bookmark: &Bookmark) -> Result<()> {
        sqlx::query(
            "UPDATE bookmarks SET title = ?, url = ?, folder = ?, tags = ? WHERE id = ?",
        )
        .bind(&bookmark.title)
        .bind(bookmark.url.as_str())
        .bind(&bookmark.folder)
        .bind(serde_json::to_string(&bookmark.tags)?)
        .bind(bookmark.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

// Implement HistoryRepository
#[async_trait]
impl HistoryRepository for SqliteDatabase {
    async fn add(&self, entry: &HistoryEntry) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO history (url, title, visited_at, visit_count)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(url) DO UPDATE SET
                title = excluded.title,
                visited_at = excluded.visited_at,
                visit_count = visit_count + 1",
        )
        .bind(entry.url.as_str())
        .bind(&entry.title)
        .bind(entry.visited_at.to_rfc3339())
        .bind(entry.visit_count)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    async fn find_by_url(&self, url: &ValidatedUrl) -> Result<Option<HistoryEntry>> {
        let result = sqlx::query_as::<_, (i64, String, String, String, i32)>(
            "SELECT id, url, title, visited_at, visit_count FROM history WHERE url = ?",
        )
        .bind(url.as_str())
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.and_then(|(id, url, title, visited_at, visit_count)| {
            ValidatedUrl::parse(&url).ok().map(|url| HistoryEntry {
                id,
                url,
                title,
                visited_at: chrono::DateTime::parse_from_rfc3339(&visited_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                visit_count,
            })
        }))
    }

    async fn search(&self, query: &str, limit: i32) -> Result<Vec<HistoryEntry>> {
        let search_pattern = format!("%{}%", query);
        let results = sqlx::query_as::<_, (i64, String, String, String, i32)>(
            "SELECT id, url, title, visited_at, visit_count FROM history
             WHERE title LIKE ? OR url LIKE ?
             ORDER BY visited_at DESC LIMIT ?",
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .filter_map(|(id, url, title, visited_at, visit_count)| {
                ValidatedUrl::parse(&url).ok().map(|url| HistoryEntry {
                    id,
                    url,
                    title,
                    visited_at: chrono::DateTime::parse_from_rfc3339(&visited_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    visit_count,
                })
            })
            .collect())
    }

    async fn get_recent(&self, limit: i32) -> Result<Vec<HistoryEntry>> {
        let results = sqlx::query_as::<_, (i64, String, String, String, i32)>(
            "SELECT id, url, title, visited_at, visit_count FROM history
             ORDER BY visited_at DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .filter_map(|(id, url, title, visited_at, visit_count)| {
                ValidatedUrl::parse(&url).ok().map(|url| HistoryEntry {
                    id,
                    url,
                    title,
                    visited_at: chrono::DateTime::parse_from_rfc3339(&visited_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    visit_count,
                })
            })
            .collect())
    }

    async fn delete_by_url(&self, url: &ValidatedUrl) -> Result<()> {
        sqlx::query("DELETE FROM history WHERE url = ?")
            .bind(url.as_str())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn clear_all(&self) -> Result<()> {
        sqlx::query("DELETE FROM history")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn increment_visit_count(&self, url: &ValidatedUrl) -> Result<()> {
        sqlx::query(
            "UPDATE history SET visit_count = visit_count + 1, visited_at = ? WHERE url = ?",
        )
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(url.as_str())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
