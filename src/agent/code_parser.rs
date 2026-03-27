// src/agent/code_parser.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tree_sitter::{Language, Parser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFile {
    pub path: String,
    pub language: String,
    pub functions: Vec<Function>,
    pub structs: Vec<Struct>,
    pub enums: Vec<Enum>,
    pub imports: Vec<Import>,
    pub docs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub signature: String,
    pub docs: Vec<String>,
    pub start_line: u32,
    pub end_line: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub docs: Vec<String>,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub docs: Vec<String>,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub files: Vec<CodeFile>,
    pub total_files: usize,
    pub total_functions: usize,
    pub total_structs: usize,
    pub total_enums: usize,
}

pub fn query_code_analysis(
    topic: &str,
    area: &str,
    module: Option<&str>,
    item_type: &str,
    pattern: Option<&str>,
) -> Result<serde_json::Value> {
    let topic_path = crate::fs::topic_path(topic, area);

    let mut results = serde_json::json!({
        "topic": topic,
        "area": area,
        "modules": []
    });

    if !topic_path.exists() {
        return Ok(results);
    }

    let modules = if let Some(mod_name) = module {
        vec![mod_name.to_string()]
    } else {
        std::fs::read_dir(&topic_path)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().to_str().map(String::from))
            .filter(|n| n != "spec" && n != "task")
            .collect()
    };

    for mod_name in modules {
        let mod_path = topic_path.join(&mod_name);
        let functions_path = mod_path.join("functions.md");

        let mut module_data = serde_json::json!({
            "name": mod_name,
            "functions": [],
            "structs": [],
            "enums": []
        });

        if functions_path.exists() {
            let content = std::fs::read_to_string(&functions_path)?;

            match item_type {
                "functions" | "all" => {
                    let funcs = extract_functions_from_md(&content);
                    let filtered: Vec<_> = funcs
                        .into_iter()
                        .filter(|f| {
                            pattern.map_or(true, |p| {
                                f.get("name")
                                    .and_then(|n| n.as_str())
                                    .map_or(false, |n| n.to_lowercase().contains(&p.to_lowercase()))
                            })
                        })
                        .collect();
                    module_data["functions"] = serde_json::json!(filtered);
                }
                "structs" => {
                    let structs = extract_structs_from_md(&content);
                    let filtered: Vec<_> = structs
                        .into_iter()
                        .filter(|s| {
                            pattern.map_or(true, |p| {
                                s.get("name")
                                    .and_then(|n| n.as_str())
                                    .map_or(false, |n| n.to_lowercase().contains(&p.to_lowercase()))
                            })
                        })
                        .collect();
                    module_data["structs"] = serde_json::json!(filtered);
                }
                "enums" => {
                    let enums = extract_enums_from_md(&content);
                    let filtered: Vec<_> = enums
                        .into_iter()
                        .filter(|e| {
                            pattern.map_or(true, |p| {
                                e.get("name")
                                    .and_then(|n| n.as_str())
                                    .map_or(false, |n| n.to_lowercase().contains(&p.to_lowercase()))
                            })
                        })
                        .collect();
                    module_data["enums"] = serde_json::json!(filtered);
                }
                _ => {}
            }
        }

        results["modules"].as_array_mut().unwrap().push(module_data);
    }

    Ok(results)
}

fn extract_functions_from_md(content: &str) -> Vec<serde_json::Value> {
    let mut funcs = Vec::new();
    let mut in_func = false;
    let mut current_name = String::new();
    let mut current_sig = String::new();

    for line in content.lines() {
        if line.starts_with("### `") && line.contains("`") {
            in_func = true;
            if let Some(start) = line.find("`") {
                if let Some(end) = line[start + 1..].find("`") {
                    current_name = line[start + 1..start + 1 + end].to_string();
                }
            }
        } else if in_func && line.starts_with("```") {
            current_sig = line.trim_start_matches("```").trim().to_string();
        } else if in_func && line.starts_with("Lines:") {
            funcs.push(serde_json::json!({
                "name": current_name,
                "signature": current_sig
            }));
            in_func = false;
        }
    }
    funcs
}

fn extract_structs_from_md(content: &str) -> Vec<serde_json::Value> {
    let mut structs = Vec::new();

    for line in content.lines() {
        if line.contains("**Structs:**") || line.contains("**Struct:**") {
            continue;
        }
        if line.starts_with("- `") && line.contains("`") {
            if let Some(name) = line.split("`- `").nth(1) {
                if let Some(name) = name.split("`").next() {
                    structs.push(serde_json::json!({ "name": name }));
                }
            }
        }
    }
    structs
}

fn extract_enums_from_md(content: &str) -> Vec<serde_json::Value> {
    let mut enums = Vec::new();

    for line in content.lines() {
        if line.contains("**Enums:**") || line.contains("**Enum:**") {
            continue;
        }
        if line.starts_with("- `") && line.contains("`") {
            if let Some(name) = line.split("`- `").nth(1) {
                if let Some(name) = name.split("`").next() {
                    enums.push(serde_json::json!({ "name": name }));
                }
            }
        }
    }
    enums
}

pub struct CodeParser {
    parser: Parser,
}

impl CodeParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            parser: Parser::new(),
        })
    }

    pub fn set_language(&mut self, language: &str) -> Result<()> {
        let lang: Language = match language {
            "rust" => tree_sitter_rust::LANGUAGE.into(),
            "javascript" | "js" => tree_sitter_javascript::LANGUAGE.into(),
            "typescript" | "ts" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            "python" | "py" => tree_sitter_python::LANGUAGE.into(),
            "go" | "golang" => tree_sitter_go::LANGUAGE.into(),
            "bash" | "sh" => tree_sitter_bash::LANGUAGE.into(),
            _ => return Err(anyhow::anyhow!("Unsupported language: {}", language)),
        };
        self.parser.set_language(&lang)?;
        Ok(())
    }

    pub fn parse_file(&mut self, path: &Path) -> Result<CodeFile> {
        let content = std::fs::read_to_string(path)?;
        let language = Self::language_from_extension(path.extension());

        if language == "unknown" {
            return Ok(CodeFile {
                path: path.to_string_lossy().to_string(),
                language,
                functions: vec![],
                structs: vec![],
                enums: vec![],
                imports: vec![],
                docs: vec![],
            });
        }

        self.set_language(&language)?;

        let tree = self
            .parser
            .parse(&content, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse file: {:?}", path))?;

        let root = tree.root_node();

        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut imports = Vec::new();
        let mut docs = Vec::new();

        self.extract_nodes(
            &root,
            &content,
            &mut functions,
            &mut structs,
            &mut enums,
            &mut imports,
            &mut docs,
        );

        Ok(CodeFile {
            path: path.to_string_lossy().to_string(),
            language,
            functions,
            structs,
            enums,
            imports,
            docs,
        })
    }

    fn extract_nodes(
        &self,
        node: &tree_sitter::Node,
        content: &str,
        functions: &mut Vec<Function>,
        structs: &mut Vec<Struct>,
        enums: &mut Vec<Enum>,
        imports: &mut Vec<Import>,
        docs: &mut Vec<String>,
    ) {
        let kind = node.kind();

        // Extract functions - broader patterns for different languages
        if kind.contains("function")
            || kind == "function_item"
            || kind == "function_declaration"
            || kind == "function_signature"
        {
            let name = self.get_node_text(node, content, "identifier").or_else(|| {
                node.child_by_field_name("name").map(|n| {
                    n.utf8_text(content.as_bytes())
                        .unwrap_or_default()
                        .to_string()
                })
            });

            if let Some(name) = name {
                let signature = node
                    .utf8_text(content.as_bytes())
                    .unwrap_or_default()
                    .to_string();
                let first_line = signature.lines().next().unwrap_or("").to_string();
                let docs = self.extract_docs(node, content);
                functions.push(Function {
                    name,
                    signature: first_line,
                    docs,
                    start_line: node.start_position().row as u32 + 1,
                    end_line: node.end_position().row as u32 + 1,
                });
            }
        }

        // Extract structs - broader patterns
        if kind.contains("struct") || kind == "struct_item" || kind == "struct_declaration" {
            let name = self
                .get_node_text(node, content, "type_identifier")
                .or_else(|| {
                    node.child_by_field_name("name").map(|n| {
                        n.utf8_text(content.as_bytes())
                            .unwrap_or_default()
                            .to_string()
                    })
                });

            if let Some(name) = name {
                let mut fields = Vec::new();

                // Try to get field declarations
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    let child_kind = child.kind();
                    if child_kind.contains("field") || child_kind == "field_declaration" {
                        if let Some(field_name) = child
                            .child_by_field_name("name")
                            .or_else(|| child.child_by_field_name("field_identifier"))
                            .map(|n| {
                                n.utf8_text(content.as_bytes())
                                    .unwrap_or_default()
                                    .to_string()
                            })
                        {
                            let field_type = child
                                .child_by_field_name("type")
                                .map(|n| {
                                    n.utf8_text(content.as_bytes())
                                        .unwrap_or_default()
                                        .to_string()
                                })
                                .unwrap_or_default();
                            fields.push(StructField {
                                name: field_name,
                                type_name: field_type,
                            });
                        }
                    }
                }

                let docs = self.extract_docs(node, content);
                structs.push(Struct { name, docs, fields });
            }
        }

        // Extract enums - broader patterns
        if kind.contains("enum") || kind == "enum_item" || kind == "enum_declaration" {
            let name = self
                .get_node_text(node, content, "type_identifier")
                .or_else(|| {
                    node.child_by_field_name("name").map(|n| {
                        n.utf8_text(content.as_bytes())
                            .unwrap_or_default()
                            .to_string()
                    })
                });

            if let Some(name) = name {
                let mut variants = Vec::new();

                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variant" || child.kind().contains("variant") {
                        if let Some(variant_name) = child
                            .child_by_field_name("name")
                            .or_else(|| child.child_by_field_name("identifier"))
                            .map(|n| {
                                n.utf8_text(content.as_bytes())
                                    .unwrap_or_default()
                                    .to_string()
                            })
                        {
                            variants.push(variant_name);
                        }
                    }
                }

                let docs = self.extract_docs(node, content);
                enums.push(Enum {
                    name,
                    docs,
                    variants,
                });
            }
        }

        // Extract imports - broader patterns
        if kind.contains("import") || kind == "use_declaration" || kind.contains("use_item") {
            if let Ok(imp) = self.extract_import(node, content) {
                if !imp.path.is_empty() {
                    imports.push(imp);
                }
            }
        }

        // Recurse through children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_nodes(&child, content, functions, structs, enums, imports, docs);
        }
    }

    fn get_node_text(
        &self,
        node: &tree_sitter::Node,
        content: &str,
        field: &str,
    ) -> Option<String> {
        node.child_by_field_name(field).map(|n| {
            n.utf8_text(content.as_bytes())
                .unwrap_or_default()
                .to_string()
        })
    }

    fn extract_docs(&self, node: &tree_sitter::Node, content: &str) -> Vec<String> {
        let mut docs = Vec::new();

        // Look for doc comments (usually before the node)
        if let Some(prev_sibling) = node.prev_sibling() {
            let text = prev_sibling
                .utf8_text(content.as_bytes())
                .unwrap_or_default();
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("///")
                    || trimmed.starts_with("//!")
                    || trimmed.starts_with("\"\"\"")
                {
                    let doc = trimmed
                        .trim_start_matches("///")
                        .trim_start_matches("//!")
                        .trim_matches('"')
                        .trim();
                    if !doc.is_empty() {
                        docs.push(doc.to_string());
                    }
                }
            }
        }

        docs
    }

    fn extract_import(&self, node: &tree_sitter::Node, content: &str) -> Result<Import> {
        let path = node
            .utf8_text(content.as_bytes())
            .unwrap_or_default()
            .to_string();
        Ok(Import {
            path,
            items: Vec::new(),
        })
    }

    fn language_from_extension(ext: Option<&std::ffi::OsStr>) -> String {
        match ext.and_then(|e| e.to_str()) {
            Some("rs") => "rust".to_string(),
            Some("js") | Some("jsx") => "javascript".to_string(),
            Some("ts") | Some("tsx") => "typescript".to_string(),
            Some("py") => "python".to_string(),
            Some("go") => "go".to_string(),
            Some("sh") | Some("bash") => "bash".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

pub fn analyze_directory(dir: &Path, languages: Vec<String>) -> Result<CodeAnalysis> {
    let mut parser = CodeParser::new()?;
    let mut files = Vec::new();

    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let lang = match ext {
            "rs" => "rust",
            "js" | "jsx" => "javascript",
            "ts" | "tsx" => "typescript",
            "py" => "python",
            "go" => "go",
            "sh" | "bash" => "bash",
            _ => continue,
        };

        if languages.is_empty() || languages.iter().any(|l| l == lang) {
            if let Ok(code_file) = parser.parse_file(path) {
                if !code_file.functions.is_empty()
                    || !code_file.structs.is_empty()
                    || !code_file.enums.is_empty()
                    || !code_file.imports.is_empty()
                {
                    files.push(code_file);
                }
            }
        }
    }

    let total_functions = files.iter().map(|f| f.functions.len()).sum();
    let total_structs = files.iter().map(|f| f.structs.len()).sum();
    let total_enums = files.iter().map(|f| f.enums.len()).sum();

    Ok(CodeAnalysis {
        total_files: files.len(),
        total_functions,
        total_structs,
        total_enums,
        files,
    })
}

pub fn parse_file_to_json(
    path: &str,
    language: Option<&str>,
    item_type: &str,
    pattern: Option<&str>,
) -> Result<String> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path));
    }

    let lang = language
        .map(String::from)
        .unwrap_or_else(|| detect_language(file_path));

    if lang == "unknown" {
        return Err(anyhow::anyhow!(
            "Could not auto-detect language. Please specify with -l flag."
        ));
    }

    let mut parser = CodeParser::new()?;
    parser.set_language(&lang)?;

    let code_file = parser.parse_file(file_path)?;

    let result = filter_code_file(&code_file, item_type, pattern);
    Ok(serde_json::to_string_pretty(&result)?)
}

fn detect_language(path: &Path) -> String {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            match ext_str.to_lowercase().as_str() {
                "rs" => return "rust".to_string(),
                "js" | "jsx" => return "javascript".to_string(),
                "ts" | "tsx" => return "typescript".to_string(),
                "py" => return "python".to_string(),
                "go" => return "go".to_string(),
                "sh" | "bash" | "zsh" => return "bash".to_string(),
                _ => {}
            }
        }
    }

    if let Ok(content) = std::fs::read_to_string(path) {
        let first_line = content.lines().next().unwrap_or("");
        if first_line.starts_with("#!") {
            if first_line.contains("python") {
                return "python".to_string();
            }
            if first_line.contains("bash") || first_line.contains("sh") {
                return "bash".to_string();
            }
            if first_line.contains("node") {
                return "javascript".to_string();
            }
            if first_line.contains("ruby") {
                return "ruby".to_string();
            }
        }
    }

    "unknown".to_string()
}

fn filter_code_file(
    code_file: &CodeFile,
    item_type: &str,
    pattern: Option<&str>,
) -> serde_json::Value {
    let pattern_lower = pattern.map(|p| p.to_lowercase());

    let matches_pattern = |name: &str| -> bool {
        pattern_lower
            .as_ref()
            .map_or(true, |p| name.to_lowercase().contains(p))
    };

    let functions: Vec<_> = code_file
        .functions
        .iter()
        .filter(|f| item_type == "all" || item_type == "functions")
        .filter(|f| matches_pattern(&f.name))
        .map(|f| {
            serde_json::json!({
                "name": f.name,
                "signature": f.signature,
                "start_line": f.start_line,
                "end_line": f.end_line,
                "docs": f.docs
            })
        })
        .collect();

    let structs: Vec<_> = code_file
        .structs
        .iter()
        .filter(|s| item_type == "all" || item_type == "structs")
        .filter(|s| matches_pattern(&s.name))
        .map(|s| {
            serde_json::json!({
                "name": s.name,
                "docs": s.docs,
                "fields": s.fields.iter().map(|f| serde_json::json!({
                    "name": f.name,
                    "type": f.type_name
                })).collect::<Vec<_>>()
            })
        })
        .collect();

    let enums: Vec<_> = code_file
        .enums
        .iter()
        .filter(|e| item_type == "all" || item_type == "enums")
        .filter(|e| matches_pattern(&e.name))
        .map(|e| {
            serde_json::json!({
                "name": e.name,
                "docs": e.docs,
                "variants": e.variants
            })
        })
        .collect();

    let imports: Vec<_> = code_file
        .imports
        .iter()
        .filter(|i| item_type == "all" || item_type == "imports")
        .map(|i| {
            serde_json::json!({
                "path": i.path,
                "items": i.items
            })
        })
        .collect();

    serde_json::json!({
        "path": code_file.path,
        "language": code_file.language,
        "functions": functions,
        "structs": structs,
        "enums": enums,
        "imports": imports,
        "counts": {
            "functions": functions.len(),
            "structs": structs.len(),
            "enums": enums.len(),
            "imports": imports.len()
        }
    })
}
