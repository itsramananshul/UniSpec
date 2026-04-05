use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub export_type: String,
    pub description: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexLink {
    pub topic: String,
    pub area: String,
    pub path: String,
    #[serde(rename = "type")]
    pub link_type: String,
    pub added: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub annotation: Option<String>,
    #[serde(default)]
    pub exports: Vec<Export>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    #[serde(default)]
    pub links: Vec<IndexLink>,
}

impl Default for Index {
    fn default() -> Self {
        Index { links: vec![] }
    }
}

pub fn index_path() -> PathBuf {
    crate::fs::spec_dir().join("index.toml")
}

pub fn code_analysis_path() -> PathBuf {
    crate::fs::spec_dir().join("code_analysis.toml")
}

pub fn load_index() -> Result<Index> {
    let path = index_path();
    if !path.exists() {
        return Ok(Index::default());
    }
    let content = fs::read_to_string(&path)?;
    let index: Index = toml::from_str(&content)?;
    Ok(index)
}

pub fn save_index(index: &Index) -> Result<()> {
    let path = index_path();
    let content = toml::to_string_pretty(index)?;
    fs::write(&path, content)?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CodeAnalysisStore {
    pub topics: std::collections::HashMap<String, CodeAnalysisTopic>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeAnalysisTopic {
    pub area: String,
    pub source_path: String,
    pub analyzed: String,
    pub files: Vec<CodeAnalysisFile>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeAnalysisFile {
    pub path: String,
    pub language: String,
    pub functions: Vec<CodeElement>,
    pub structs: Vec<CodeElement>,
    pub enums: Vec<CodeElement>,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeElement {
    pub name: String,
    pub signature: Option<String>,
    pub start_line: Option<u32>,
    pub end_line: Option<u32>,
    pub calls: Vec<String>,
}

pub fn load_code_analysis() -> Result<CodeAnalysisStore> {
    let path = code_analysis_path();
    if !path.exists() {
        return Ok(CodeAnalysisStore::default());
    }
    let content = fs::read_to_string(&path)?;
    let store: CodeAnalysisStore = toml::from_str(&content)?;
    Ok(store)
}

pub fn save_code_analysis(store: &CodeAnalysisStore) -> Result<()> {
    let path = code_analysis_path();
    let content = toml::to_string_pretty(store)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn add_code_analysis(
    topic: &str,
    area: &str,
    source_path: &str,
    files: Vec<CodeAnalysisFile>,
) -> Result<()> {
    let mut store = load_code_analysis()?;

    let topic_entry = CodeAnalysisTopic {
        area: area.to_string(),
        source_path: source_path.to_string(),
        analyzed: chrono::Utc::now().to_rfc3339(),
        files,
    };

    store.topics.insert(topic.to_string(), topic_entry);
    save_code_analysis(&store)
}

pub fn add_link(topic: &str, area: &str, path: &str, link_type: &str) -> Result<()> {
    add_link_with_metadata(topic, area, path, link_type, &[], None)
}

pub fn add_link_with_metadata(
    topic: &str,
    area: &str,
    path: &str,
    link_type: &str,
    tags: &[String],
    annotation: Option<&str>,
) -> Result<()> {
    add_link_with_exports(topic, area, path, link_type, tags, annotation, &[])
}

pub fn add_link_with_exports(
    topic: &str,
    area: &str,
    path: &str,
    link_type: &str,
    tags: &[String],
    annotation: Option<&str>,
    exports: &[Export],
) -> Result<()> {
    let mut index = load_index()?;

    if index
        .links
        .iter()
        .any(|l| l.topic == topic && l.path == path)
    {
        return Err(anyhow::anyhow!(
            "Link already exists: {} -> {}",
            topic,
            path
        ));
    }

    let link = IndexLink {
        topic: topic.to_string(),
        area: area.to_string(),
        path: path.to_string(),
        link_type: link_type.to_string(),
        added: chrono_now(),
        tags: tags.to_vec(),
        annotation: annotation.map(String::from),
        exports: exports.to_vec(),
    };

    index.links.push(link);
    save_index(&index)?;
    Ok(())
}

pub fn remove_link(topic: &str, path: &str) -> Result<()> {
    let mut index = load_index()?;
    let original_len = index.links.len();
    index
        .links
        .retain(|l| !(l.topic == topic && l.path == path));

    if index.links.len() == original_len {
        return Err(anyhow::anyhow!("Link not found: {} -> {}", topic, path));
    }

    save_index(&index)?;
    Ok(())
}

pub fn find_by_topic(topic: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| l.topic.to_lowercase() == topic.to_lowercase())
        .collect();
    Ok(links)
}

pub fn find_by_path(search_path: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let search_lower = search_path.to_lowercase();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| l.path.to_lowercase().contains(&search_lower))
        .collect();
    Ok(links)
}

pub fn list_all() -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    Ok(index.links)
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    let day = secs / 86400;
    format!("{}T{:02}:{:02}:00Z", day, hours, minutes)
}

pub fn cleanup() -> Result<(Vec<String>, Vec<String>)> {
    let mut index = load_index()?;
    let spec_dir = crate::fs::spec_dir();

    let mut removed_topics: Vec<String> = Vec::new();
    let mut removed_paths: Vec<String> = Vec::new();

    let original_links = index.links.clone();
    index.links.clear();

    for link in original_links {
        let topic_path = spec_dir.join(&link.area).join(&link.topic);
        let path_exists = std::path::Path::new(&link.path).exists();

        if topic_path.exists() && path_exists {
            index.links.push(link);
        } else {
            if !topic_path.exists() {
                removed_topics.push(format!("{}/{}", link.area, link.topic));
            }
            if !path_exists {
                removed_paths.push(link.path.clone());
            }
        }
    }

    save_index(&index)?;
    Ok((removed_topics, removed_paths))
}

pub fn find_by_tag(tag: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let tag_lower = tag.to_lowercase();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| l.tags.iter().any(|t| t.to_lowercase() == tag_lower))
        .collect();
    Ok(links)
}

pub fn find_by_annotation(query: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let query_lower = query.to_lowercase();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| {
            l.annotation
                .as_ref()
                .map(|a| a.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
        })
        .collect();
    Ok(links)
}

pub fn find_by_tags_any(tags: &[String]) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let tags_lower: Vec<String> = tags.iter().map(|t| t.to_lowercase()).collect();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| {
            l.tags
                .iter()
                .any(|t| tags_lower.contains(&t.to_lowercase()))
        })
        .collect();
    Ok(links)
}

pub fn list_all_tags() -> Result<Vec<String>> {
    let index = load_index()?;
    let mut tags_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for link in &index.links {
        for tag in &link.tags {
            tags_set.insert(tag.clone());
        }
    }
    let mut tags: Vec<String> = tags_set.into_iter().collect();
    tags.sort();
    Ok(tags)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub topic: String,
    pub area: String,
    pub path: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub edge_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

pub fn export_graph() -> Result<GraphData> {
    let index = load_index()?;

    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut seen_paths: std::collections::HashSet<String> = std::collections::HashSet::new();

    for link in &index.links {
        if !seen_paths.contains(&link.path) {
            seen_paths.insert(link.path.clone());
            nodes.push(GraphNode {
                id: format!("path-{}", link.path.replace('/', "-").replace('.', "-")),
                topic: link.topic.clone(),
                area: link.area.clone(),
                path: link.path.clone(),
                tags: link.tags.clone(),
            });
        }

        edges.push(GraphEdge {
            source: format!("topic-{}", link.topic.replace('/', "-")),
            target: format!("path-{}", link.path.replace('/', "-").replace('.', "-")),
            edge_type: "links_to".to_string(),
        });
    }

    let topics: std::collections::HashSet<String> =
        index.links.iter().map(|l| l.topic.clone()).collect();
    for topic in topics {
        nodes.push(GraphNode {
            id: format!("topic-{}", topic.replace('/', "-")),
            topic: topic.clone(),
            area: String::new(),
            path: String::new(),
            tags: vec![],
        });
    }

    Ok(GraphData { nodes, edges })
}

pub fn generate_backlinks() -> Result<std::collections::HashMap<String, Vec<IndexLink>>> {
    let index = load_index()?;
    let mut backlinks: std::collections::HashMap<String, Vec<IndexLink>> =
        std::collections::HashMap::new();

    for link in &index.links {
        backlinks
            .entry(link.topic.clone())
            .or_insert_with(Vec::new)
            .push(link.clone());
    }

    Ok(backlinks)
}

pub fn generate_backlinks_file(topic: &str, area: &str) -> Result<String> {
    let links = find_by_topic(topic)?;

    let mut md = String::new();
    md.push_str(&format!("# Backlinks: {}\n\n", topic));
    md.push_str(&format!("Area: {}\n\n", area));
    md.push_str("## Linked Files\n\n");

    if links.is_empty() {
        md.push_str("_No files linked to this topic._\n");
    } else {
        for link in &links {
            md.push_str(&format!(
                "- [{}]({}) - {}\n",
                link.path,
                link.path,
                link.annotation.as_deref().unwrap_or(&link.link_type)
            ));
        }
    }

    md.push_str("\n## Tags\n\n");
    let all_tags: std::collections::HashSet<String> =
        links.iter().flat_map(|l| l.tags.clone()).collect();
    if all_tags.is_empty() {
        md.push_str("_No tags._\n");
    } else {
        for tag in all_tags {
            md.push_str(&format!("- {}\n", tag));
        }
    }

    Ok(md)
}

pub fn get_exports_for_topic(topic: &str) -> Result<Vec<Export>> {
    let index = load_index()?;
    let topic_lower = topic.to_lowercase();
    let mut exports: Vec<Export> = Vec::new();

    for link in &index.links {
        if link.topic.to_lowercase() == topic_lower {
            exports.extend(link.exports.clone());
        }
    }

    Ok(exports)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportQueryResult {
    pub id: String,
    pub topic: String,
    pub path: String,
    pub name: String,
    pub export_type: String,
    pub description: String,
    pub signature: Option<String>,
}

pub fn find_exports(query: &str, by: &str) -> Result<Vec<ExportQueryResult>> {
    let index = load_index();
    if index.is_err() {
        return Ok(vec![]);
    }
    let index = index.unwrap();
    let query_lower = query.to_lowercase();
    let mut results: Vec<ExportQueryResult> = Vec::new();

    for link in &index.links {
        for export in &link.exports {
            let matches = match by {
                "name" => export.name.to_lowercase().contains(&query_lower),
                "type" => export.export_type.to_lowercase().contains(&query_lower),
                "description" => export.description.to_lowercase().contains(&query_lower),
                "id" => export.id.to_lowercase().contains(&query_lower),
                _ => false,
            };

            if matches {
                results.push(ExportQueryResult {
                    id: export.id.clone(),
                    topic: link.topic.clone(),
                    path: link.path.clone(),
                    name: export.name.clone(),
                    export_type: export.export_type.clone(),
                    description: export.description.clone(),
                    signature: export.signature.clone(),
                });
            }
        }
    }

    Ok(results)
}

pub fn find_export_by_id(full_id: &str) -> Result<Option<ExportQueryResult>> {
    let index = load_index();
    if index.is_err() {
        return Ok(None);
    }
    let index = index.unwrap();

    for link in &index.links {
        for export in &link.exports {
            if export.id == full_id {
                return Ok(Some(ExportQueryResult {
                    id: export.id.clone(),
                    topic: link.topic.clone(),
                    path: link.path.clone(),
                    name: export.name.clone(),
                    export_type: export.export_type.clone(),
                    description: export.description.clone(),
                    signature: export.signature.clone(),
                }));
            }
        }
    }

    Ok(None)
}

pub fn get_dependents(topic: &str) -> Result<Vec<ExportQueryResult>> {
    let index = load_index();
    if index.is_err() {
        return Ok(vec![]);
    }
    let index = index.unwrap();
    let topic_prefix = format!("{}:", topic.to_lowercase());
    let mut dependents: Vec<ExportQueryResult> = Vec::new();

    for link in &index.links {
        if link.topic.to_lowercase() != topic.to_lowercase() {
            for export in &link.exports {
                if export.id.to_lowercase().starts_with(&topic_prefix) {
                    dependents.push(ExportQueryResult {
                        id: export.id.clone(),
                        topic: link.topic.clone(),
                        path: link.path.clone(),
                        name: export.name.clone(),
                        export_type: export.export_type.clone(),
                        description: export.description.clone(),
                        signature: export.signature.clone(),
                    });
                }
            }
        }
    }

    Ok(dependents)
}

pub fn find_callers(symbol_name: &str) -> Result<Vec<String>> {
    let store = load_code_analysis()?;
    let mut callers = Vec::new();

    for (topic, topic_data) in &store.topics {
        for file in &topic_data.files {
            for func in &file.functions {
                if func.calls.iter().any(|c| c == symbol_name) {
                    callers.push(format!("{}::{}", topic, func.name));
                }
            }
        }
    }
    Ok(callers)
}
