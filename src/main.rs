mod blacklist;
mod blp_comparator;
mod cli;
mod file_collector;
mod file_copier;

use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

use blacklist::Blacklist;
use blp_comparator::BlpComparator;
use cli::Args;
use file_collector::FileCollector;
use file_copier::FileCopier;

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🎮 Uncensor Icon");
    println!("======================");
    println!("📁 Global: {}", args.global_dir);
    println!("📁 CN: {}", args.cn_dir);
    println!("📁 Output: {}", args.output_dir);
    println!();

    let blacklist = Blacklist::new(&args.blacklist);
    if blacklist.get_excluded_count() > 0 {
        println!("🚫 Blacklist ({} files):", blacklist.get_excluded_count());
        for file in blacklist.get_excluded_files() {
            println!("   - {}", file);
        }
        println!();
    }

    let start_time = Instant::now();
    let global_path = Path::new(&args.global_dir);
    let cn_path = Path::new(&args.cn_dir);
    let output_path = Path::new(&args.output_dir);

    // Validate directories
    if !global_path.exists() {
        anyhow::bail!("❌ Global directory does not exist: {:?}", global_path);
    }
    if !cn_path.exists() {
        anyhow::bail!("❌ CN directory does not exist: {:?}", cn_path);
    }

    println!("🔍 Step 1: Scanning for BLP files...");
    let mut collector = FileCollector::new();
    collector.collect_files(global_path, cn_path, &blacklist)?;

    let global_files = collector.get_global_files();
    let cn_files = collector.get_cn_files();

    println!("📊 Global version: {} files", global_files.len());
    println!("📊 CN version: {} files", cn_files.len());
    println!(
        "📊 After filtering: {} files",
        collector.get_filtered_count()
    );
    println!();

    let files_to_compare = collector.get_files_to_compare();
    let global_only_files = collector.get_global_only_files();

    println!("⚖️  Files to compare: {}", files_to_compare.len());
    println!("➕ Files only in global: {}", global_only_files.len());
    println!();

    println!("🔬 Step 2: Comparing icons pixel by pixel...");
    let comparator = BlpComparator::new();
    let mut different_files: Vec<PathBuf> = Vec::new();

    // Create progress bar for comparison
    let comparison_progress = ProgressBar::new(files_to_compare.len() as u64);
    comparison_progress.set_style(
        ProgressStyle::default_bar()
            .template(
                "🔬 {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .expect("Invalid progress template")
            .progress_chars("#>-"),
    );
    comparison_progress.set_message("Comparing icons...");

    let comparison_results: Vec<(String, bool)> = files_to_compare
        .par_iter()
        .enumerate()
        .map(|(_index, relative_path): (usize, &String)| {
            let global_file = &global_files[relative_path];
            let cn_file = &cn_files[relative_path];

            // Update progress and compare
            comparison_progress.inc(1);
            match comparator.are_files_different(&global_file.path, &cn_file.path) {
                Ok(different) => (relative_path.clone(), different),
                Err(_) => (relative_path.clone(), true),
            }
        })
        .collect();

    comparison_progress.finish();

    // Collect different files from comparison results
    for (relative_path, different) in comparison_results.iter() {
        if *different {
            if let Some(global_file) = global_files.get(relative_path) {
                different_files.push(global_file.path.clone());
            }
        }
    }

    // Add global-only files
    different_files.extend(global_only_files.iter().map(|f| f.path.clone()));

    println!("🎯 Found {} different icons!", different_files.len());
    println!();

    if !different_files.is_empty() {
        println!("📋 Step 3: Copying better icons...");
        let file_copier = FileCopier::with_progress(different_files.len() as u64);
        let copied_count = file_copier.copy_files(&different_files, output_path)?;
        file_copier.finish_progress();

        println!("✅ Successfully copied {} icons! 🎉", copied_count);
    } else {
        println!("✨ No different icons found - versions are identical!");
    }

    let duration = start_time.elapsed();
    println!();
    println!("📈 === FINAL RESULTS ===");
    println!("📊 Global files: {}", global_files.len());
    println!("📊 CN files: {}", cn_files.len());
    println!("🔬 Compared: {}", files_to_compare.len());
    println!("➕ Global-only: {}", global_only_files.len());
    println!("🎯 Icons copied: {}", different_files.len());
    println!("⏱️  Time: {:.2}s", duration.as_secs_f64());
    println!("📁 Output: {:?}", output_path);

    Ok(())
}
