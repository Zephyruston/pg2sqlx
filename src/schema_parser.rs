//! PostgreSQL schema parser module
//! This module is responsible for parsing PostgreSQL schema files and extracting type information

use std::fs;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct EnumType {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct VectorType {
    pub name: String,
}

pub struct SchemaParser {
    pub enum_types: Vec<EnumType>,
    pub vector_types: Vec<VectorType>,
    verbose: bool,
}

impl SchemaParser {
    pub fn with_verbose(verbose: bool) -> Self {
        SchemaParser {
            enum_types: Vec::new(),
            vector_types: Vec::new(),
            verbose,
        }
    }

    /// Parse a PostgreSQL schema file and extract custom types
    pub fn parse_schema_file(&mut self, file_path: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.verbose = verbose;
        let mut file = fs::File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        self.parse_schema_contents(&contents);
        Ok(())
    }

    /// Parse schema contents and extract custom types
    fn parse_schema_contents(&mut self, contents: &str) {
        let lines: Vec<&str> = contents.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            
            // Debug output for CREATE TYPE lines
            if self.verbose && line.to_uppercase().starts_with("CREATE TYPE") {
                println!("Found CREATE TYPE line: {}", line);
            }

            // Check for ENUM type definitions
            if line.to_uppercase().starts_with("CREATE TYPE") && line.to_uppercase().contains("ENUM") {
                if self.verbose {
                    println!("Processing ENUM type: {}", line);
                }
                if let Some(enum_type) = self.parse_enum_type(&lines, &mut i) {
                    if self.verbose {
                        println!("Successfully parsed ENUM type: {}", enum_type.name);
                    }
                    self.enum_types.push(enum_type);
                } else if self.verbose {
                    println!("Failed to parse ENUM type: {}", line);
                }
                // Skip to the end of the ENUM definition to avoid reprocessing lines
                while i < lines.len() && !lines[i].trim().ends_with(");") {
                    i += 1;
                }
                i += 1; // Move past the closing line
                continue;
            }

            // Check for VECTOR type usage (in column definitions)
            if line.to_uppercase().contains("VECTOR(") {
                // Add VECTOR type if it's used in the schema
                if !self.vector_types.iter().any(|v| v.name == "vector") {
                    self.vector_types.push(VectorType {
                        name: "vector".to_string(),
                    });
                }
            }

            i += 1;
        }
    }

    /// Parse an ENUM type definition
    fn parse_enum_type(&self, lines: &[&str], index: &mut usize) -> Option<EnumType> {
        let start_line = lines[*index].trim();
        if self.verbose {
            println!("Parsing ENUM from line: {}", start_line);
        }
        
        // Extract type name (between "CREATE TYPE" and "AS ENUM")
        let type_name = if let Some(start_pos) = start_line.find("CREATE TYPE") {
            let start_pos = start_pos + 12; // "CREATE TYPE".len()
            if let Some(end_pos) = start_line.find("AS ENUM") {
                start_line[start_pos..end_pos].trim().to_string()
            } else {
                if self.verbose {
                    println!("Could not find AS ENUM in line: {}", start_line);
                }
                return None;
            }
        } else {
            if self.verbose {
                println!("Could not find CREATE TYPE in line: {}", start_line);
            }
            return None;
        };

        if self.verbose {
            println!("Extracted type name: {}", type_name);
        }
        
        // Clean up the type name (remove schema prefix if exists)
        let type_name = type_name.split('.').next_back().unwrap_or(&type_name).to_string();
        if self.verbose {
            println!("Cleaned type name: {}", type_name);
        }

        let mut values = Vec::new();
        
        // Check if this is a single-line ENUM definition
        if start_line.contains(");") {
            // Single-line ENUM
            if self.verbose {
                println!("Processing single-line ENUM");
            }
            // Extract values between parentheses
            let line_without_comments = if let Some(pos) = start_line.find("--") {
                &start_line[..pos]
            } else {
                start_line
            };
            
            // Find the part between parentheses
            if let Some(start_pos) = line_without_comments.find('(')
                && let Some(end_pos) = line_without_comments.find(')') {
                    let values_part = &line_without_comments[start_pos+1..end_pos];
                    if self.verbose {
                        println!("Values part: {}", values_part);
                    }
                    
                    // Split by comma and extract values
                    let parts: Vec<&str> = values_part.split(',').collect();
                    for part in parts {
                        let part = part.trim();
                        if part.starts_with('\'') && part.ends_with('\'') && part.len() > 2 {
                            let value = part[1..part.len()-1].to_string();
                            values.push(value.clone());
                            if self.verbose {
                                println!("Found enum value: {}", value);
                            }
                        }
                    }
                }
        } else {
            // Multi-line ENUM
            if self.verbose {
                println!("Processing multi-line ENUM");
            }
            let mut in_enum_definition = false;

            // Parse enum values
            while *index < lines.len() {
                let line = lines[*index].trim();
                if self.verbose {
                    println!("Processing line in enum definition: {}", line);
                }
                
                // Check if we're entering the enum definition
                if line.to_uppercase().contains("AS ENUM") {
                    in_enum_definition = true;
                    if self.verbose {
                        println!("Entered enum definition");
                    }
                }
                
                // If we're in the enum definition, extract values
                if in_enum_definition {
                    // Extract values between parentheses
                    let line_without_comments = if let Some(pos) = line.find("--") {
                        &line[..pos]
                    } else {
                        line
                    };
                    
                    if self.verbose {
                        println!("Line without comments: {}", line_without_comments);
                    }
                    
                    // Look for values in the format 'value_name'
                    let parts: Vec<&str> = line_without_comments.split(',').collect();
                    for part in parts {
                        let part = part.trim();
                        if part.starts_with('\'') && part.ends_with('\'') && part.len() > 2 {
                            let value = part[1..part.len()-1].to_string();
                            values.push(value.clone());
                            if self.verbose {
                                println!("Found enum value: {}", value);
                            }
                        }
                    }
                    
                    // Check if this is the end of the enum definition
                    if line_without_comments.contains(");") {
                        if self.verbose {
                            println!("Reached end of enum definition");
                        }
                        break;
                    }
                }
                
                *index += 1;
            }
        }

        if !values.is_empty() {
            Some(EnumType {
                name: type_name,
                values,
            })
        } else {
            if self.verbose {
                println!("No values found for enum type: {}", type_name);
            }
            None
        }
    }
}