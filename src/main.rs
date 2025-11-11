use clap::Parser;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub playlists: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub name: String,
    pub last_modified_date: String,
    pub collaborators: Vec<Value>,
    pub items: Vec<Item>,
    pub description: Value,
    pub number_of_followers: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub track: Track,
    pub episode: Value,
    pub audiobook: Value,
    pub local_track: Value,
    pub added_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub track_name: String,
    pub artist_name: String,
    pub album_name: String,
    pub track_uri: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Convert Spotify playlists JSON to Markdown or HTML files", long_about = None)]
struct Args {
    /// Input JSON file path
    #[arg(short, long)]
    input: String,

    /// Output directory for files
    #[arg(short, long, default_value = "output")]
    output: String,

    /// Output format: markdown or html
    #[arg(short, long, default_value = "markdown")]
    format: String,
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn get_common_styles() -> &'static str {
    r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background-color: white;
            border-radius: 8px;
            padding: 30px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 {
            color: #1db954;
            margin-bottom: 20px;
        }
        a {
            color: #1db954;
            text-decoration: none;
        }
        a:hover {
            text-decoration: underline;
        }
        .back-to-top {
            position: fixed;
            bottom: 20px;
            right: 20px;
            background-color: #1db954;
            color: white;
            padding: 12px 20px;
            border-radius: 25px;
            text-decoration: none;
            box-shadow: 0 2px 8px rgba(0,0,0,0.2);
            transition: background-color 0.3s;
        }
        .back-to-top:hover {
            background-color: #1ed760;
            text-decoration: none;
        }
        .nav-link {
            display: inline-block;
            margin-bottom: 20px;
            padding: 8px 16px;
            background-color: #f0f0f0;
            border-radius: 4px;
        }
    "#
}

fn generate_markdown(playlist: &Playlist) -> String {
    let mut md = String::new();

    // Header
    md.push_str(&format!("# {}\n\n", playlist.name));

    // Back to index link
    md.push_str("[← Back to Index](index.md)\n\n");

    // Metadata
    md.push_str("## Playlist Information\n\n");
    md.push_str(&format!(
        "- **Last Modified:** {}\n",
        playlist.last_modified_date
    ));
    md.push_str(&format!(
        "- **Followers:** {}\n",
        playlist.number_of_followers
    ));
    md.push_str(&format!("- **Total Tracks:** {}\n\n", playlist.items.len()));

    if !playlist.items.is_empty() {
        md.push_str("## Tracks\n\n");
        md.push_str("| # | Track Name | Artist | Album | Added Date |\n");
        md.push_str("|---|------------|--------|-------|------------|\n");

        for (idx, item) in playlist.items.iter().enumerate() {
            let track = &item.track;
            md.push_str(&format!(
                "| {} | [{}]({}) | {} | {} | {} |\n",
                idx + 1,
                escape_markdown(&track.track_name),
                track.track_uri,
                escape_markdown(&track.artist_name),
                escape_markdown(&track.album_name),
                item.added_date
            ));
        }
    }

    md.push_str("\n[↑ Back to Top](#)\n\n");
    md.push_str("[← Back to Index](index.md)\n");

    md
}

fn escape_markdown(text: &str) -> String {
    text.replace('|', "\\|")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

fn generate_html(playlist: &Playlist) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str(
        "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
    );
    html.push_str(&format!(
        "    <title>{}</title>\n",
        escape_html(&playlist.name)
    ));
    html.push_str("    <style>\n");
    html.push_str(get_common_styles());
    html.push_str("        .metadata {\n");
    html.push_str("            background-color: #f9f9f9;\n");
    html.push_str("            padding: 15px;\n");
    html.push_str("            border-radius: 5px;\n");
    html.push_str("            margin-bottom: 30px;\n");
    html.push_str("        }\n");
    html.push_str("        .metadata p {\n");
    html.push_str("            margin: 5px 0;\n");
    html.push_str("        }\n");
    html.push_str("        table {\n");
    html.push_str("            width: 100%;\n");
    html.push_str("            border-collapse: collapse;\n");
    html.push_str("        }\n");
    html.push_str("        th {\n");
    html.push_str("            background-color: #1db954;\n");
    html.push_str("            color: white;\n");
    html.push_str("            padding: 12px;\n");
    html.push_str("            text-align: left;\n");
    html.push_str("        }\n");
    html.push_str("        td {\n");
    html.push_str("            padding: 12px;\n");
    html.push_str("            border-bottom: 1px solid #ddd;\n");
    html.push_str("        }\n");
    html.push_str("        tr:hover {\n");
    html.push_str("            background-color: #f5f5f5;\n");
    html.push_str("        }\n");
    html.push_str("        .track-number {\n");
    html.push_str("            color: #999;\n");
    html.push_str("            text-align: center;\n");
    html.push_str("            width: 50px;\n");
    html.push_str("        }\n");
    html.push_str("    </style>\n");
    html.push_str("</head>\n<body>\n");
    html.push_str("    <div class=\"container\">\n");

    // Back to index link
    html.push_str("        <a href=\"index.html\" class=\"nav-link\">← Back to Index</a>\n");

    // Header
    html.push_str(&format!(
        "        <h1>{}</h1>\n",
        escape_html(&playlist.name)
    ));

    // Metadata
    html.push_str("        <div class=\"metadata\">\n");
    html.push_str(&format!(
        "            <p><strong>Last Modified:</strong> {}</p>\n",
        escape_html(&playlist.last_modified_date)
    ));
    html.push_str(&format!(
        "            <p><strong>Followers:</strong> {}</p>\n",
        playlist.number_of_followers
    ));
    html.push_str(&format!(
        "            <p><strong>Total Tracks:</strong> {}</p>\n",
        playlist.items.len()
    ));
    html.push_str("        </div>\n");

    // Tracks table
    if !playlist.items.is_empty() {
        html.push_str("        <h2>Tracks</h2>\n");
        html.push_str("        <table>\n");
        html.push_str("            <thead>\n");
        html.push_str("                <tr>\n");
        html.push_str("                    <th class=\"track-number\">#</th>\n");
        html.push_str("                    <th>Track Name</th>\n");
        html.push_str("                    <th>Artist</th>\n");
        html.push_str("                    <th>Album</th>\n");
        html.push_str("                    <th>Added Date</th>\n");
        html.push_str("                </tr>\n");
        html.push_str("            </thead>\n");
        html.push_str("            <tbody>\n");

        for (idx, item) in playlist.items.iter().enumerate() {
            let track = &item.track;
            html.push_str("                <tr>\n");
            html.push_str(&format!(
                "                    <td class=\"track-number\">{}</td>\n",
                idx + 1
            ));
            html.push_str(&format!(
                "                    <td><a href=\"{}\">{}</a></td>\n",
                escape_html(&track.track_uri),
                escape_html(&track.track_name)
            ));
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                escape_html(&track.artist_name)
            ));
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                escape_html(&track.album_name)
            ));
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                escape_html(&item.added_date)
            ));
            html.push_str("                </tr>\n");
        }

        html.push_str("            </tbody>\n");
        html.push_str("        </table>\n");
    }

    html.push_str("    </div>\n");

    // Floating back to top button
    html.push_str("    <a href=\"#\" class=\"back-to-top\">↑ Top</a>\n");

    html.push_str("</body>\n</html>");

    html
}

fn generate_index_markdown(playlists: &[Playlist], filenames: &[String]) -> String {
    let mut md = String::new();

    md.push_str("# My Spotify Playlists\n\n");

    let total_tracks: usize = playlists.iter().map(|p| p.items.len()).sum();
    md.push_str(&format!("**Total Playlists:** {}\n\n", playlists.len()));
    md.push_str(&format!("**Total Tracks:** {}\n\n", total_tracks));

    md.push_str("## Playlists\n\n");

    for (playlist, filename) in playlists.iter().zip(filenames.iter()) {
        md.push_str(&format!(
            "- [**{}**]({}) - {} tracks, {} followers\n",
            playlist.name,
            filename,
            playlist.items.len(),
            playlist.number_of_followers
        ));
    }

    md
}

fn generate_index_html(playlists: &[Playlist], filenames: &[String]) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str(
        "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
    );
    html.push_str("    <title>My Spotify Playlists</title>\n");
    html.push_str("    <style>\n");
    html.push_str(get_common_styles());
    html.push_str("        .stats {\n");
    html.push_str("            display: flex;\n");
    html.push_str("            gap: 30px;\n");
    html.push_str("            margin-bottom: 30px;\n");
    html.push_str("        }\n");
    html.push_str("        .stat-card {\n");
    html.push_str("            background-color: #f9f9f9;\n");
    html.push_str("            padding: 20px;\n");
    html.push_str("            border-radius: 8px;\n");
    html.push_str("            flex: 1;\n");
    html.push_str("        }\n");
    html.push_str("        .stat-card h3 {\n");
    html.push_str("            margin: 0 0 10px 0;\n");
    html.push_str("            color: #666;\n");
    html.push_str("            font-size: 14px;\n");
    html.push_str("            text-transform: uppercase;\n");
    html.push_str("        }\n");
    html.push_str("        .stat-card p {\n");
    html.push_str("            margin: 0;\n");
    html.push_str("            font-size: 32px;\n");
    html.push_str("            font-weight: bold;\n");
    html.push_str("            color: #1db954;\n");
    html.push_str("        }\n");
    html.push_str("        .playlist-grid {\n");
    html.push_str("            display: grid;\n");
    html.push_str("            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));\n");
    html.push_str("            gap: 20px;\n");
    html.push_str("        }\n");
    html.push_str("        .playlist-card {\n");
    html.push_str("            background-color: #f9f9f9;\n");
    html.push_str("            padding: 20px;\n");
    html.push_str("            border-radius: 8px;\n");
    html.push_str("            transition: transform 0.2s, box-shadow 0.2s;\n");
    html.push_str("        }\n");
    html.push_str("        .playlist-card:hover {\n");
    html.push_str("            transform: translateY(-2px);\n");
    html.push_str("            box-shadow: 0 4px 12px rgba(0,0,0,0.15);\n");
    html.push_str("        }\n");
    html.push_str("        .playlist-card h3 {\n");
    html.push_str("            margin: 0 0 10px 0;\n");
    html.push_str("            color: #333;\n");
    html.push_str("        }\n");
    html.push_str("        .playlist-card h3 a {\n");
    html.push_str("            color: #333;\n");
    html.push_str("        }\n");
    html.push_str("        .playlist-meta {\n");
    html.push_str("            color: #666;\n");
    html.push_str("            font-size: 14px;\n");
    html.push_str("        }\n");
    html.push_str("    </style>\n");
    html.push_str("</head>\n<body>\n");
    html.push_str("    <div class=\"container\">\n");

    html.push_str("        <h1>My Spotify Playlists</h1>\n");

    // Stats
    let total_tracks: usize = playlists.iter().map(|p| p.items.len()).sum();
    html.push_str("        <div class=\"stats\">\n");
    html.push_str("            <div class=\"stat-card\">\n");
    html.push_str("                <h3>Total Playlists</h3>\n");
    html.push_str(&format!("                <p>{}</p>\n", playlists.len()));
    html.push_str("            </div>\n");
    html.push_str("            <div class=\"stat-card\">\n");
    html.push_str("                <h3>Total Tracks</h3>\n");
    html.push_str(&format!("                <p>{}</p>\n", total_tracks));
    html.push_str("            </div>\n");
    html.push_str("        </div>\n");

    // Playlist grid
    html.push_str("        <h2>Playlists</h2>\n");
    html.push_str("        <div class=\"playlist-grid\">\n");

    for (playlist, filename) in playlists.iter().zip(filenames.iter()) {
        html.push_str("            <div class=\"playlist-card\">\n");
        html.push_str(&format!(
            "                <h3><a href=\"{}\">{}</a></h3>\n",
            escape_html(filename),
            escape_html(&playlist.name)
        ));
        html.push_str("                <div class=\"playlist-meta\">\n");
        html.push_str(&format!(
            "                    {} tracks<br>\n",
            playlist.items.len()
        ));
        html.push_str(&format!(
            "                    {} followers\n",
            playlist.number_of_followers
        ));
        html.push_str("                </div>\n");
        html.push_str("            </div>\n");
    }

    html.push_str("        </div>\n");
    html.push_str("    </div>\n");
    html.push_str("</body>\n</html>");

    html
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validate format
    let format = args.format.to_lowercase();
    if format != "markdown" && format != "html" {
        eprintln!("Error: format must be either 'markdown' or 'html'");
        std::process::exit(1);
    }

    let extension = if format == "html" { "html" } else { "md" };

    // Read and parse JSON
    println!("Reading JSON file: {}", args.input);
    let json_content = fs::read_to_string(&args.input)?;
    let root: Root = serde_json::from_str(&json_content)?;

    // Create output directory
    fs::create_dir_all(&args.output)?;
    println!("Output directory: {}", args.output);
    println!("Output format: {}", format);

    let mut filenames = Vec::new();

    // Process each playlist
    println!("\nProcessing {} playlists...", root.playlists.len());
    for playlist in &root.playlists {
        let filename = format!("{}.{}", sanitize_filename(&playlist.name), extension);
        let filepath = Path::new(&args.output).join(&filename);

        let content = if format == "html" {
            generate_html(playlist)
        } else {
            generate_markdown(playlist)
        };

        fs::write(&filepath, content)?;
        filenames.push(filename.clone());

        println!(
            "  ✓ Created: {} ({} tracks)",
            filename,
            playlist.items.len()
        );
    }

    // Generate index file
    let index_filename = format!("index.{}", extension);
    let index_filepath = Path::new(&args.output).join(&index_filename);

    let index_content = if format == "html" {
        generate_index_html(&root.playlists, &filenames)
    } else {
        generate_index_markdown(&root.playlists, &filenames)
    };

    fs::write(&index_filepath, index_content)?;
    println!("\n  ✓ Created: {}", index_filename);

    println!(
        "\nDone! Generated {} {} files plus index.",
        root.playlists.len(),
        format
    );
    println!("Open {} to get started!", index_filepath.display());

    Ok(())
}
