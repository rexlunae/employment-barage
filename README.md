# Employment Barage

A cross-platform job search and application automation tool built with Dioxus (Rust).

## Features

- **Resume Upload & Parsing** - PDF/DOCX parsing with automatic data extraction
- **Profile Management** - Comprehensive user profiles with experience, education, skills
- **Resume Generation** - Multiple templates (Professional, Modern, Creative, Simple, Academic)
- **Resume Analysis** - AI-powered suggestions and ATS compatibility scoring
- **Job Search** - Multi-platform job searching from various sources
- **Application Automation** - AI-generated cover letters

## Job Sources

Employment Barage fetches jobs from multiple sources:

### Free Sources (No API Key Required)

| Source | Description | Geographic Focus |
|--------|-------------|------------------|
| **Remotive** | Remote job listings | Worldwide |
| **HN Who's Hiring** | Monthly Hacker News hiring threads | Tech/Worldwide |
| **Arbeitnow** | Job board focused on Europe | Germany/EU |

### API Key Required

| Source | API Info | Status |
|--------|----------|--------|
| **LinkedIn** | Official API (requires business account) | Not implemented |
| **Indeed** | Publisher API (requires approval) | Not implemented |
| **Glassdoor** | Partner API (requires approval) | Not implemented |

## Job Source API Documentation

### Remotive
- **URL**: https://remotive.com/api/remote-jobs
- **Docs**: https://remotive.com/api-documentation
- **Rate Limit**: ~4 requests/day recommended
- **Key Required**: No

### Hacker News Who's Hiring
- **URL**: https://hacker-news.firebaseio.com/v0/
- **Docs**: https://github.com/HackerNews/API
- **Rate Limit**: None specified
- **Key Required**: No
- **Notes**: Scrapes monthly "Ask HN: Who is hiring?" threads

### Arbeitnow
- **URL**: https://www.arbeitnow.com/api/job-board-api
- **Rate Limit**: Not specified
- **Key Required**: No
- **Notes**: Primarily German/EU jobs, some require German language

## Project Structure

```
employment-barage/
├─ api/           # Shared backend logic (server functions, job sources, database)
│  └─ src/
│     ├─ db/           # SQLite database layer
│     ├─ models/       # Data models (Job, Profile, Resume, etc.)
│     └─ services/     # Business logic
│        ├─ job_sources/   # Job API clients (Remotive, HN, Arbeitnow)
│        ├─ job_service.rs # Job search and application services
│        └─ ai_service.rs  # Cover letter generation (Ollama/OpenAI/Claude)
├─ ui/            # Shared UI components
├─ web/           # Web frontend
├─ desktop/       # Desktop app (Tauri)
├─ mobile/        # Mobile app
└─ Cargo.toml     # Workspace config
```

## Development

### Prerequisites

- Rust (latest stable)
- Dioxus CLI: `cargo install dioxus-cli`

### Running

```bash
# Web development server
cd web && dx serve

# Desktop application
cd desktop && dx serve

# Mobile application
cd mobile && dx serve
```

### Building

```bash
cargo build               # Build all workspace members
cargo build -p api        # Build specific crate
```

### Testing

```bash
cargo test                # Run all tests
cargo test -p api         # Test specific crate
```

## Using Job Sources Programmatically

```rust
use api::services::job_sources::JobAggregator;

// Create an aggregator that fetches from all free sources
let aggregator = JobAggregator::new();

// Fetch jobs with optional filters
let jobs = aggregator.fetch_all(
    Some("rust developer"),  // keywords
    Some("remote"),          // location
    Some(50),                // limit per source
).await?;

// Jobs are automatically stored in SQLite when using the server function
let result = fetch_external_jobs(
    Some("python".to_string()),
    Some("remote".to_string()),
    Some(100),
).await?;

println!("Fetched {} jobs, saved {}", result.fetched, result.saved);
```

## Database

Jobs are stored in SQLite at:
- Linux: `~/.local/share/employment-barage/data.db`
- macOS: `~/Library/Application Support/employment-barage/data.db`
- Windows: `%APPDATA%\employment-barage\data.db`

## AI Cover Letter Generation

Supports multiple AI backends for cover letter generation:

- **Ollama** (default) - Local, free, no API key needed
- **OpenAI** - Requires `OPENAI_API_KEY`
- **Claude** - Requires `ANTHROPIC_API_KEY`

## License

MIT
