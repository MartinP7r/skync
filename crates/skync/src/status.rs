use anyhow::Result;
use console::style;
use std::path::Path;

use crate::config::Config;
use crate::discover;

/// Display the current status of the skync system.
pub fn show(config: &Config) -> Result<()> {
    println!(
        "{} {}",
        style("Library:").bold(),
        config.library_dir.display()
    );

    // Count skills in library
    let lib_count = count_entries(&config.library_dir);
    println!("  {} skills consolidated", style(lib_count).cyan());
    println!();

    // Sources
    println!("{}", style("Sources:").bold());
    if config.sources.is_empty() {
        println!("  (none configured)");
    } else {
        for source in &config.sources {
            let count = discover::discover_source(source)
                .map(|s| s.len())
                .unwrap_or(0);
            println!(
                "  {:<40} {} skills",
                style(source.path.display()).dim(),
                style(count).cyan()
            );
        }
    }
    println!();

    // Targets
    println!("{}", style("Targets:").bold());
    let mut any_target = false;
    for (name, t) in config.targets.iter() {
        any_target = true;
        let status = if t.enabled {
            style("enabled").green()
        } else {
            style("disabled").dim()
        };
        println!("  {:<20} {}", style(name).bold(), status);
    }
    if !any_target {
        println!("  (none configured)");
    }
    println!();

    // Health check
    let broken = count_broken_symlinks(&config.library_dir);
    if broken == 0 {
        println!("{} {}", style("Health:").bold(), style("All good").green());
    } else {
        println!(
            "{} {}",
            style("Health:").bold(),
            style(format!("{} broken symlinks", broken)).red()
        );
    }

    Ok(())
}

fn count_entries(dir: &Path) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| entries.count())
        .unwrap_or(0)
}

fn count_broken_symlinks(dir: &Path) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let path = e.path();
                    // is_symlink() checks the link itself; exists() follows it —
                    // a symlink that exists but whose target doesn't yields true + false
                    path.is_symlink() && !path.exists()
                })
                .count()
        })
        .unwrap_or(0)
}
