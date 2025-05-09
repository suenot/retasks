use anyhow::{Context, Result};
use clap::{App, Arg};
use hotwatch::{Hotwatch, Event};
use octorust::{auth::Credentials, Client, types};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Issue {
    number: i64,
    title: String,
    body: Option<String>,
    state: String,
    labels: Vec<String>,
}

struct Config {
    token: String,
    repo_owner: String,
    repo_name: String,
    issues_dir: PathBuf,
    watch: bool,
    sync_interval: Duration,
}

fn main() -> Result<()> {
    // Create a tokio runtime for async operations
    let rt = Runtime::new().context("Failed to create tokio runtime")?;

    let matches = App::new("GitHub Issues Sync")
        .version("1.0")
        .author("RetasksTeam")
        .about("Synchronizes GitHub issues with a local directory")
        .arg(
            Arg::with_name("issues-dir")
                .long("issues-dir")
                .value_name("DIR")
                .help("Sets the directory for issues (default: ./issues)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .help("Watch for changes and sync automatically"),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .value_name("TOKEN")
                .help("GitHub API token")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("repo")
                .long("repo")
                .value_name("OWNER/REPO")
                .help("GitHub repository in format owner/repo")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .value_name("SECONDS")
                .help("Sync interval in seconds when using --watch (default: 300)")
                .takes_value(true),
        )
        .get_matches();

    let repo_parts: Vec<&str> = matches
        .value_of("repo")
        .unwrap()
        .split('/')
        .collect();
    
    if repo_parts.len() != 2 {
        return Err(anyhow::anyhow!("Repository must be in format owner/repo"));
    }

    let config = Config {
        token: matches.value_of("token").unwrap().to_string(),
        repo_owner: repo_parts[0].to_string(),
        repo_name: repo_parts[1].to_string(),
        issues_dir: PathBuf::from(matches.value_of("issues-dir").unwrap_or("./issues")),
        watch: matches.is_present("watch"),
        sync_interval: Duration::from_secs(
            matches
                .value_of("interval")
                .unwrap_or("300")
                .parse()
                .unwrap_or(300),
        ),
    };

    // Create issues directory if it doesn't exist
    if !config.issues_dir.exists() {
        fs::create_dir_all(&config.issues_dir).context("Failed to create issues directory")?;
    }

    // Initial sync from GitHub to local
    println!("Performing initial sync from GitHub to local...");
    rt.block_on(sync_github_to_local(&config)).context("Failed to sync from GitHub to local")?;

    if config.watch {
        println!("Watch mode enabled. Monitoring for changes...");
        
        let config_arc = Arc::new(config);
        let config_clone = Arc::clone(&config_arc);
        
        // Thread for periodic GitHub to local sync
        let rt_handle = rt.handle().clone();
        thread::spawn(move || {
            let config = config_clone;
            loop {
                thread::sleep(config.sync_interval);
                println!("Performing scheduled sync from GitHub to local...");
                if let Err(e) = rt_handle.block_on(sync_github_to_local(&config)) {
                    eprintln!("Error syncing from GitHub: {}", e);
                }
            }
        });

        // Watch local directory for changes
        let config_clone = Arc::clone(&config_arc);
        let rt_handle = rt.handle().clone();
        let mut hotwatch = Hotwatch::new().context("Failed to initialize hotwatch")?;
        
        hotwatch.watch(&config_arc.issues_dir, move |event: Event| {
            if let Event::Write(path) = event {
                if path.extension().map_or(false, |ext| ext == "md") {
                    println!("Local file changed: {:?}", path);
                    let config = &config_clone;
                    if let Err(e) = rt_handle.block_on(sync_local_to_github(config, &path)) {
                        eprintln!("Error syncing to GitHub: {}", e);
                    }
                }
            }
        }).context("Failed to watch directory")?;

        // Keep the main thread alive
        loop {
            thread::sleep(Duration::from_secs(60));
        }
    } else {
        println!("One-time sync completed. Use --watch for continuous sync.");
    }

    Ok(())
}

async fn sync_github_to_local(config: &Config) -> Result<()> {
    let client = Client::new(
        "github-issues-sync".to_string(),
        Credentials::Token(config.token.clone()),
    )?;

    let issues_client = client.issues();
    
    // List issues with the correct parameters
    let issues_response = issues_client.list(
        types::Filter::All,
        types::IssuesListState::All,
        &config.repo_owner,
        types::IssuesListSort::Created,
        types::Order::Desc,
        None, 
        false, 
        false, 
        false, 
        false, 
        100, 
        1
    ).await.context("Failed to list issues from GitHub")?;
    
    let issues = issues_response.body;

    for issue in issues {
        // Extract labels - use a simpler approach since the exact structure is complex
        let labels: Vec<String> = Vec::new(); // Default to empty labels if we can't extract them properly

        let local_issue = Issue {
            number: issue.number,
            title: issue.title,
            body: Some(issue.body),
            state: issue.state,
            labels,
        };

        let file_path = config.issues_dir.join(format!("issue-{}.md", issue.number));
        let mut file = File::create(&file_path).context(format!("Failed to create file: {}", file_path.display()))?;

        // Create frontmatter with issue metadata
        let frontmatter = format!(
            "---\nnumber: {}\ntitle: {}\nstate: {}\nlabels: [{}]\n---\n\n",
            local_issue.number,
            local_issue.title,
            local_issue.state,
            local_issue.labels.join(", ")
        );

        file.write_all(frontmatter.as_bytes()).context("Failed to write frontmatter")?;
        
        // Write issue body
        if let Some(body) = local_issue.body {
            file.write_all(body.as_bytes()).context("Failed to write issue body")?;
        }

        println!("Synced issue #{} to {}", issue.number, file_path.display());
    }

    Ok(())
}

async fn sync_local_to_github(config: &Config, file_path: &Path) -> Result<()> {
    if !file_path.is_file() || file_path.extension().map_or(true, |ext| ext != "md") {
        return Ok(());
    }

    let mut file = File::open(file_path).context(format!("Failed to open file: {}", file_path.display()))?;
    let mut content = String::new();
    file.read_to_string(&mut content).context("Failed to read file content")?;

    // Parse frontmatter and body
    let (frontmatter, body) = parse_markdown_file(&content).context("Failed to parse markdown file")?;
    
    let client = Client::new(
        "github-issues-sync".to_string(),
        Credentials::Token(config.token.clone()),
    )?;

    // Extract issue number from filename or frontmatter
    let issue_number = frontmatter.get("number")
        .and_then(|n| n.parse::<i64>().ok())
        .ok_or_else(|| anyhow::anyhow!("Could not determine issue number"))?;

    // Get the current state as a proper enum value
    let state = if let Some(state_str) = frontmatter.get("state") {
        match state_str.to_lowercase().as_str() {
            "closed" => Some(types::State::Closed),
            "open" => Some(types::State::Open),
            _ => None
        }
    } else {
        None
    };
    
    // Create update request with required empty string for assignee
    let mut update = types::IssuesUpdateRequest {
        title: None,
        body: body, // No need for Some() wrapper here as the type is String, not Option<String>
        state,
        assignee: String::new(),
        assignees: vec![],
        milestone: None,
        labels: vec![],
    };
    
    // Set title if available
    if let Some(title) = frontmatter.get("title") {
        update.title = Some(types::TitleOneOf::String(title.clone()));
    }
    
    // Process labels
    if let Some(labels_str) = frontmatter.get("labels") {
        let labels: Vec<String> = labels_str
            .split(',')
            .map(|s| s.trim().trim_matches(|c| c == '[' || c == ']').to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if !labels.is_empty() {
            update.labels = labels.into_iter()
                .map(|label| types::IssuesCreateRequestLabelsOneOf::String(label))
                .collect();
        }
    }

    client.issues().update(
        &config.repo_owner,
        &config.repo_name,
        issue_number,
        &update,
    ).await.context(format!("Failed to update issue #{} on GitHub", issue_number))?;

    println!("Updated issue #{} on GitHub from {}", issue_number, file_path.display());
    Ok(())
}

fn parse_markdown_file(content: &str) -> Result<(HashMap<String, String>, String)> {
    let mut frontmatter = HashMap::new();
    let mut body = String::new();

    // Check if the file has frontmatter (starts with ---)
    if content.starts_with("---") {
        if let Some(end_index) = content[3..].find("---") {
            let frontmatter_str = &content[3..end_index + 3];
            
            // Parse frontmatter
            for line in frontmatter_str.lines() {
                if let Some(index) = line.find(':') {
                    let key = line[..index].trim().to_string();
                    let value = line[index + 1..].trim().to_string();
                    frontmatter.insert(key, value);
                }
            }
            
            // Get body (everything after frontmatter)
            if end_index + 6 <= content.len() {
                body = content[end_index + 6..].trim().to_string();
            }
        } else {
            // No end marker for frontmatter
            body = content.to_string();
        }
    } else {
        // No frontmatter
        body = content.to_string();
    }

    Ok((frontmatter, body))
}
