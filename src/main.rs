//! Main module for the pg2sqlx tool
//! This tool helps map PostgreSQL custom types to Go types in goctl.yaml

use clap::Parser;
use std::time::Instant;

mod schema_parser;
mod yaml_updater;

use schema_parser::SchemaParser;
use yaml_updater::YamlUpdater;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the PostgreSQL schema file
    #[arg(short, long)]
    schema_file: String,

    /// Path to the goctl.yaml configuration file
    #[arg(short, long)]
    config: String,

    /// Path to output the updated configuration file (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// Enable verbose output for debugging
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Start timer for performance measurement
    let start_time = Instant::now();

    // Parse the schema file
    let mut parser = SchemaParser::with_verbose(cli.verbose);
    parser.parse_schema_file(&cli.schema_file, cli.verbose)?;
    
    // Print found types for debugging
    if cli.verbose {
        println!("Found {} ENUM types:", parser.enum_types.len());
        for enum_type in &parser.enum_types {
            println!("  - {} with {} values", enum_type.name, enum_type.values.len());
        }
        
        println!("Found {} VECTOR types:", parser.vector_types.len());
        for vector_type in &parser.vector_types {
            println!("  - {}", vector_type.name);
        }
        
        // Show parsing time
        let parse_duration = start_time.elapsed();
        println!("Schema parsing completed in: {:?}", parse_duration);
    }

    // Load the YAML configuration
    let mut updater = YamlUpdater::new(&cli.config)?;
    
    // Add mappings for ENUM types
    for enum_type in &parser.enum_types {
        updater.add_enum_mapping(&enum_type.name);
    }
    
    // Add mapping for VECTOR type if it's used
    if !parser.vector_types.is_empty() {
        updater.add_vector_mapping();
    }
    
    // Sort the types map to ensure consistent ordering
    let sorted_types = updater.get_sorted_types_map();
    
    // Create a new config with sorted types
    let mut sorted_config = updater.config().clone();
    sorted_config.model.types_map.clear();
    for (key, value) in sorted_types {
        sorted_config.model.types_map.insert(key.clone(), value.clone());
    }
    
    // Create a new updater with the sorted config
    let sorted_updater = YamlUpdater::with_config(sorted_config);
    
    // Save the updated configuration
    match cli.output {
        Some(output_path) => {
            sorted_updater.save_to_new_file(&output_path)?;
            if cli.verbose {
                println!("Updated configuration saved to {}", output_path);
            } else {
                println!("Configuration updated successfully");
            }
        }
        None => {
            sorted_updater.save_to_file(&cli.config)?;
            if cli.verbose {
                println!("Configuration updated in place: {}", cli.config);
            } else {
                println!("Configuration updated successfully");
            }
        }
    }
    
    // Show total execution time
    if cli.verbose {
        let total_duration = start_time.elapsed();
        println!("Total execution time: {:?}", total_duration);
    }
    
    Ok(())
}