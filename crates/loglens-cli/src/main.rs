use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::{Parser as ClapParser, Subcommand};
use loglens_core::{ParserRegistry, QueryFilter, Severity, StreamingLogReader};
use loglens_storage::{Database, EventRepository, GroupRepository, SourceRepository, UserRepository, WorkspaceRepository};
use uuid::Uuid;

#[derive(ClapParser)]
#[command(name = "loglens", author, version, about = "Local-first log exploration CLI")]
struct Cli {
    #[arg(short, long, default_value = "sqlite:///data/loglens.db")]
    db_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspect a log file without saving to database
    Inspect {
        file: PathBuf,
    },
    /// Import a log file into the database
    Import {
        file: PathBuf,
        #[arg(short, long)]
        workspace: Option<Uuid>,
    },
    /// Search imported log events
    Search {
        query: String,
        #[arg(short, long)]
        level: Option<String>,
        #[arg(short, long, default_value_t = 50)]
        limit: usize,
    },
    /// List aggregated error groups
    Groups {
        #[arg(short, long)]
        level: Option<String>,
    },
    /// Run diagnostic check on environment and database
    Doctor,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { file } => {
            println!("🔍 Inspecting log file: {}", file.display());
            let content = std::fs::read_to_string(&file)
                .with_context(|| format!("Failed to read file {}", file.display()))?;

            let registry = ParserRegistry::new();
            let (parser, confidence) = registry.detect_best_parser(&content);
            println!("Detected parser: {} (confidence: {:.2})", parser.name(), confidence);
            println!("Line count: {}", content.lines().count());
            println!("First 3 lines:");
            for (idx, line) in content.lines().take(3).enumerate() {
                println!("  [{}] {}", idx + 1, line);
            }
        }
        Commands::Import { file, workspace } => {
            let db = Database::new_sqlite(&cli.db_url).await.context("Failed to open database")?;
            let ws_id = workspace.unwrap_or_else(Uuid::new_v4);
            let src_id = Uuid::new_v4();

            let reader = StreamingLogReader::new(Default::default());
            let (source, events) = reader.process_file(&file, ws_id, src_id, None).await
                .map_err(|e| anyhow::anyhow!("Import failed: {}", e))?;

            let src_repo = SourceRepository::new(db.pool());
            src_repo.create_source(&source).await.context("Failed to save source")?;

            let event_repo = EventRepository::new(db.pool());
            event_repo.insert_events_batch(&events).await.context("Failed to save events")?;

            println!("✅ Successfully imported {} events from {}", events.len(), file.display());
        }
        Commands::Search { query, level, limit } => {
            let db = Database::new_sqlite(&cli.db_url).await.context("Failed to open database")?;
            let event_repo = EventRepository::new(db.pool());

            let mut filter = QueryFilter {
                search_query: Some(query.clone()),
                limit,
                ..Default::default()
            };

            if let Some(lvl) = level {
                filter.severities.push(Severity::from_str_loose(&lvl));
            }

            let events = event_repo.query_events(Uuid::nil(), &filter).await.context("Failed to query events")?;
            println!("Found {} matching events:", events.len());
            for ev in events {
                println!("[{}] [{}] {}", ev.ingested_at.format("%H:%M:%S"), ev.severity, ev.message);
            }
        }
        Commands::Groups { level: _ } => {
            let db = Database::new_sqlite(&cli.db_url).await.context("Failed to open database")?;
            let group_repo = GroupRepository::new(db.pool());
            let groups = group_repo.list_groups(Uuid::nil()).await.context("Failed to list groups")?;

            println!("Found {} error groups:", groups.len());
            for g in groups {
                println!("Count: {} | Sev: {} | Sample: {}", g.occurrence_count, g.severity, g.sample_message);
            }
        }
        Commands::Doctor => {
            println!("🩺 Running LogLens Diagnostics...");
            println!("System OS: {}", std::env::consts::OS);
            println!("Arch: {}", std::env::consts::ARCH);

            match Database::new_sqlite(&cli.db_url).await {
                Ok(_) => println!("✅ Database connection OK"),
                Err(e) => println!("❌ Database connection failed: {}", e),
            }
        }
    }

    Ok(())
}
