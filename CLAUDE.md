# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a Dioxus fullstack application using a Cargo workspace with multi-platform support. The architecture follows a shared UI approach with platform-specific entry points for web, desktop, and mobile targets.

## Common Commands

### Development
Navigate to the specific platform directory and use `dx serve`:
```bash
cd web && dx serve        # Web development server
cd desktop && dx serve    # Desktop application
cd mobile && dx serve     # Mobile application
```

### Building
```bash
cargo build               # Build all workspace members
cargo build -p web        # Build specific crate
cargo build -p desktop
cargo build -p mobile
cargo build -p ui
cargo build -p api
```

### Testing
```bash
cargo test                # Run all tests
cargo test -p <crate>     # Test specific crate
```

### Linting
```bash
cargo clippy              # Lint all workspace members
cargo clippy -p <crate>   # Lint specific crate
```

## Architecture

### Workspace Structure
- **`ui/`** - Shared UI components (Hero, Navbar, Echo) used across all platforms
- **`api/`** - Shared backend server functions and API logic
- **`web/`** - Web-specific entry point and views
- **`desktop/`** - Desktop-specific entry point and views  
- **`mobile/`** - Mobile-specific entry point and views

### Platform Pattern
Each platform crate follows the same structure:
- `main.rs` - Entry point with platform-specific routing setup
- `views/` - Platform-specific view implementations
- `assets/` - Platform-specific assets and styling

### Shared Components
- UI components in `ui/src/` are re-exported through `lib.rs`
- Server functions in `api/src/lib.rs` use `#[server]` macro for fullstack integration
- Each platform wraps shared components with platform-specific routing

### Key Dependencies
- **Dioxus 0.6.0** - Main framework with router support
- **dioxus-bootstrap** - UI component library
- Platform crates depend on both `ui` and workspace `dioxus`
- dioxus-bootstrap - Provides the main components with bootstrap for styling.

### Clippy Configuration
Custom clippy rules in `clippy.toml` prevent holding Dioxus signals/refs across await points to avoid borrow checker issues.

## Application Features

### Core Functionality
- **Resume Upload & Parsing** - PDF/DOCX parsing with automatic data extraction
- **Profile Management** - Comprehensive user profiles with experience, education, skills
- **Resume Generation** - Multiple templates (Professional, Modern, Creative, Simple, Academic)
- **Resume Analysis** - AI-powered suggestions and ATS compatibility scoring
- **Job Search** - Multi-platform job searching (LinkedIn, Indeed, Glassdoor)
- **Application Automation** - AI-generated cover letters and automated applications

### UI Components
- `Dashboard` - Main application interface with navigation
- `ResumeUpload` - File upload with drag-and-drop support
- `ProfileManager` - Tabbed profile editing interface
- `ResumeBuilder` - Template selection and resume generation
- `JobSearch` - Job search with filtering and quick apply functionality

### Server Functions
Located in `api/src/`:
- `profile_service.rs` - Profile and resume parsing endpoints
- `resume_service.rs` - Resume generation and analysis
- `job_service.rs` - Job search and application automation

## Development Notes

- Each platform starts with identical UI but can diverge independently
- Server functions in `api/` are automatically collected when using `dx serve`
- Assets are platform-specific and loaded using the `asset!` macro
- Routes are defined per-platform using Dioxus router macros
- Bootstrap CSS and Font Awesome icons are loaded via CDN
- File upload uses web-sys APIs for browser compatibility