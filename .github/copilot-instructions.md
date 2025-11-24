# Copilot Coding Agent Instructions for rust-aoc

## Repository Overview

This repository contains Rust solutions for Advent of Code (AoC) problems organized by year. It's a small to medium-sized Rust workspace project using the 2024 edition with an inventory-based plugin system for solution registration.

### Key Statistics
- **Language**: Rust (edition 2024)
- **Project Type**: Cargo workspace with multiple year packages
- **Size**: ~50 source files across multiple years (2022, 2023, 2024)
- **Build Time**: ~7.5 seconds from clean, ~0.2 seconds incremental
- **Release Build**: ~16 seconds

## Build and Validation Instructions

### Prerequisites
- Rust toolchain version 1.91.1+ (rustc 1.91.1 / cargo 1.91.1 or newer)
- No additional system dependencies required

### Essential Commands

**ALWAYS run commands in this specific order to avoid issues:**

#### 1. Building
```bash
# Standard development build (recommended for most work)
cargo build
# Expected time: ~7.5s from clean, ~0.2s incremental
# Output: Binaries in target/debug/

# Release build (optimized)
cargo build --release
# Expected time: ~16 seconds from clean
# Output: Binaries in target/release/
```

#### 2. Running Solutions
```bash
# Run all days for a specific year
cargo run -p runner -- 2024

# Run specific day for a year
cargo run -p runner -- 2024 --day 1

# Run for other years (2022, 2023)
cargo run -p runner -- 2022 --day 5
```

#### 3. Testing
```bash
cargo test
# Expected: No tests currently defined (0 tests)
# Test runner executes successfully even with no tests
```

#### 4. Linting and Formatting

**CRITICAL: Always run formatting and linting before committing code changes:**

```bash
# Check formatting (DO NOT automatically fix yet - review first)
cargo fmt --check
# Returns exit code 1 if formatting issues exist
# Shows diffs of what would change

# Apply formatting fixes
cargo fmt

# Run clippy linter
cargo clippy
# Shows warnings about code quality issues like:
# - Redundant field names (solve: solve should be just solve)
# - Needless return statements
# - Code style improvements
```

#### 5. Code Quality Checks
```bash
# Check for errors without building binaries (faster)
cargo check
# Expected time: ~0.2s after initial build

# Clean build artifacts
cargo clean
# Removes target/ directory (356.9MB when present)
```

### Common Build Issues and Workarounds

1. **File lock warnings**: You may see "Blocking waiting for file lock on package cache" messages when running multiple cargo commands simultaneously. This is normal - cargo will wait and then proceed.

2. **Edition 2024**: This project uses Rust edition 2024. Ensure your toolchain is up to date if you encounter edition-related errors.

3. **No task command**: While a `Taskfile.yml` exists in the repository root, the `task` command is NOT installed and should NOT be used. Use cargo commands directly instead.

## Project Architecture and Layout

### Directory Structure
```
rust-aoc/
├── Cargo.toml                 # Workspace root manifest
├── Cargo.lock                 # Dependency lock file
├── Taskfile.yml               # Task definitions (task command not available)
├── .gitignore                 # Ignores target/ directory
├── common/                    # Shared library crate
│   ├── Cargo.toml            # Package: common v0.1.0, edition 2024
│   └── src/
│       └── lib.rs            # Defines AocEntry trait
├── runner/                    # Main executable crate
│   ├── Cargo.toml            # Package: runner v0.1.0, depends on all year packages
│   └── src/
│       └── main.rs           # CLI entry point using structopt
└── years/                     # Year-specific solution packages
    ├── aoc2022/
    │   ├── Cargo.toml        # Package: aoc2022 v0.1.0
    │   ├── inputs/           # Puzzle inputs (day01/, day02/, etc.)
    │   │   └── dayXX/
    │   │       ├── full.txt
    │   │       └── example.txt
    │   └── src/
    │       ├── lib.rs        # Exports AocEntry2022 struct, uses inventory
    │       ├── day01.rs      # Individual day solutions
    │       ├── day02.rs
    │       └── ...
    ├── aoc2023/              # Same structure as aoc2022
    └── aoc2024/              # Same structure as aoc2022
```

### Architectural Components

#### 1. Common Library (`common/src/lib.rs`)
Defines the core trait that all solutions implement:
```rust
pub trait AocEntry {
    fn year(&self) -> u32;
    fn day(&self) -> u32;
    fn solve(&self) -> fn() -> String;
}
```

#### 2. Year Packages (`years/aocYYYY/src/lib.rs`)
Each year package:
- Defines a year-specific struct (e.g., `AocEntry2024`)
- Uses `inventory::collect!` macro to register solutions
- Exports `get_entries()` function to retrieve all solutions
- Each day module uses `inventory::submit!` to register itself

Example pattern in `years/aoc2024/src/day01.rs`:
```rust
pub fn solve() -> String {
    // Solution logic here
    "Not implemented".to_string()
}

inventory::submit! {
    crate::AocEntry2024 {
        year: 2024,
        day: 1,
        solve,  // Note: clippy warns about redundant field names
    }
}
```

#### 3. Runner (`runner/src/main.rs`)
- CLI using `structopt` for argument parsing
- Accepts year (required) and --day (optional)
- Collects entries from all year packages
- Filters by year and day, then executes solutions
- Outputs: "Day N: <result>"

### Adding a New Day Solution

**Follow this exact pattern:**

1. Create `years/aocYYYY/src/dayXX.rs` with:
   ```rust
   pub fn solve() -> String {
       // Your solution here
       "result".to_string()
   }
   
   inventory::submit! {
       crate::AocEntryYYYY {
           year: YYYY,
           day: XX,
           solve,
       }
   }
   ```

2. Add to `years/aocYYYY/src/lib.rs`:
   ```rust
   pub mod dayXX;
   ```

3. Create input files (if needed):
   - `years/aocYYYY/inputs/dayXX/full.txt`
   - `years/aocYYYY/inputs/dayXX/example.txt`

4. Read inputs using:
   ```rust
   let project_root = env!("CARGO_MANIFEST_DIR");
   let input_path = Path::new(project_root).join("inputs/dayXX/full.txt");
   let contents = fs::read_to_string(input_path).expect("Error reading file");
   ```

### Dependencies
- **anyhow**: Error handling (used in runner)
- **structopt**: CLI argument parsing (used in runner)
- **inventory**: Runtime plugin registration system
- **itertools**: Iterator utilities (aoc2024)
- **regex**: Regular expressions (aoc2024)
- **lazy_static**: Static initialization (aoc2022)

### Key Configuration Files
- **Cargo.toml (root)**: Workspace configuration, lists all member packages
- **No CI/CD**: No GitHub Actions workflows defined
- **No rustfmt.toml**: Uses default Rust formatting rules
- **No clippy.toml**: Uses default clippy lints
- **.gitignore**: Contains only `target` (build directory)

## Validation Steps

Before finalizing any code changes:

1. **Format**: `cargo fmt` - Apply standard Rust formatting
2. **Check**: `cargo check` - Fast error checking (~0.2s)
3. **Build**: `cargo build` - Full build verification (~0.2s incremental)
4. **Lint**: `cargo clippy` - Check for common mistakes and style issues
5. **Test**: `cargo test` - Run all tests (currently none defined)
6. **Run**: Test your specific solution with `cargo run -p runner -- YEAR --day DAY`

## Important Notes for Coding Agents

1. **Trust these instructions**: Only search for additional information if something here is incomplete or contradictory. This document has been validated through actual command execution.

2. **No task command**: Despite `Taskfile.yml` existing, do NOT attempt to use the `task` command as it's not installed. Use cargo directly.

3. **Formatting is required**: The codebase has existing formatting inconsistencies. When modifying code, ensure your changes are formatted with `cargo fmt`.

4. **Common clippy warnings**: The codebase currently has several clippy warnings about redundant field names and needless returns. Match the existing style unless explicitly fixing these issues.

5. **Inventory system**: Solutions are automatically discovered via the `inventory` crate. No manual registration in runner needed - just add the module to the year's lib.rs.

6. **No tests**: This repository currently has no test infrastructure. Focus on building and running solutions to verify correctness.

7. **Input file locations**: Each day's inputs are in `years/aocYYYY/inputs/dayXX/`. Use `env!("CARGO_MANIFEST_DIR")` to build paths relative to the year package.

8. **Build artifacts**: The `target/` directory can grow to ~350MB. It's gitignored and can be cleaned with `cargo clean` if needed.

## Files in Repository Root
- `Cargo.toml` - Workspace manifest
- `Cargo.lock` - Dependency versions
- `Taskfile.yml` - Task definitions (not usable without task CLI)
- `.gitignore` - Git ignore rules (target directory)
- `common/` - Shared trait definitions
- `runner/` - Main executable
- `years/` - Year-specific solution packages
