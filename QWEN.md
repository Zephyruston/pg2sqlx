# pg2sqlx - PostgreSQL to Go Type Mapper

## Project Overview

pg2sqlx is a Rust-based tool designed to solve custom type mapping issues when generating PostgreSQL database models using the goctl tool for Go developers. The tool automatically identifies ENUM and VECTOR types in PostgreSQL schema files and generates the corresponding type mapping configurations, enabling goctl to correctly generate model code that includes these custom types.

### Key Technologies
- **Rust**: Main programming language for the tool
- **Go**: Target language for generated models
- **PostgreSQL**: Database system with custom ENUM and VECTOR types
- **goctl**: Go-zero framework's model generation tool
- **YAML**: Configuration format for goctl

### Architecture
The tool consists of three main components:
1. **Schema Parser** (`src/schema_parser.rs`): Parses PostgreSQL schema files to identify custom ENUM and VECTOR types
2. **YAML Updater** (`src/yaml_updater.rs`): Updates the goctl.yaml configuration file with type mappings
3. **Main Module** (`src/main.rs`): Command-line interface and orchestration logic

## Building and Running

### Prerequisites
- Rust and Cargo installed on the system
- Go development environment
- goctl tool (version 1.6.5 or higher)

### Build Process
```bash
# Build the project in release mode
cargo build --release

# Install the tool globally
cargo install --path .
```

### Usage
```bash
# Basic usage - updates goctl.yaml in place
pg2sqlx --schema-file=001_init_schema.sql --config=goctl.yaml

# Specify output file to preserve original
pg2sqlx --schema-file=001_init_schema.sql --config=goctl.yaml --output=goctl_updated.yaml
```

### Complete Workflow
1. Build the project: `cargo build --release`
2. Run the tool to update configuration: `./target/release/pg2sqlx --schema-file=001_init_schema.sql --config=goctl.yaml`
3. Generate models: `cd script && ./genAll.sh`

## Development Conventions

### Code Structure
- **src/main.rs**: Entry point and CLI argument parsing
- **src/schema_parser.rs**: PostgreSQL schema parsing logic
- **src/yaml_updater.rs**: goctl.yaml file manipulation
- **script/**: Helper scripts for model generation
- **specs/**: Project specifications and documentation

### Type Mapping Conventions
- **ENUM types**: Mapped to Go `string` type with `sql.NullString` for null handling
- **VECTOR types**: Mapped to Go `string` type with `sql.NullString` for null handling and includes the pgvector package

### Dependencies
- **serde**: Serialization/deserialization framework
- **serde_yaml**: YAML parsing and serialization
- **clap**: Command-line argument parsing

## Project Context

This tool was created to address a specific pain point in the go-zero framework's model generation process. When using goctl to generate PostgreSQL models, custom types like ENUM and VECTOR are not properly handled by default, requiring manual configuration in the goctl.yaml file. This tool automates that process by parsing the database schema and automatically generating the necessary type mappings.

The tool works by:
1. Parsing PostgreSQL schema files to identify custom ENUM and VECTOR types
2. Reading the existing goctl.yaml configuration
3. Adding the identified types to the configuration's type mapping section
4. Saving the updated configuration
5. Allowing goctl to generate properly typed models

## Common Development Tasks

### Adding New Type Support
1. Modify the schema parser to recognize the new type
2. Update the YAML updater to add appropriate mappings
3. Update documentation and examples

### Testing
- Ensure schema parsing works with various ENUM formats (single-line and multi-line)
- Verify YAML file updates preserve existing mappings
- Confirm generated configurations work with goctl

### Troubleshooting
Common issues:
1. "unsupported database type" errors - ensure pg2sqlx has run and updated the configuration
2. Type mapping not working - verify goctl version >= 1.6.5 and experimental features are enabled
3. Performance issues - schema parsing should be fast (typically <5ms)