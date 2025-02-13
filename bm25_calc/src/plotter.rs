use plotters::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct ExperimentResult {
    #[tabled(rename = "Experiment Name")]
    name: String,
    #[tabled(rename = "# Bins")]
    bins: usize,
    #[tabled(rename = "Items Removed")]
    removed: usize,
    #[tabled(rename = "Total Items")]
    total: usize,
    #[tabled(rename = "Avg Load")]
    avg_load: usize,
    #[tabled(rename = "Keywords w/Overlap")]
    keywords: usize,
    #[tabled(rename = "EMD")]
    emd: f64,
}

use crate::error::Result;

/// Takes in a Hashset and plots a histogram showing the number of items in each bin
///
/// # Arguments
///
/// * `histogram`: A vec of 'bins', each bin should be a integer that relates to a document ID
/// * `sorted`: If set to true, puts the largest bin on the left of the histogram
///
/// returns: Result<(), Box<dyn std::error::Error>>
#[allow(unsafe_code)] // allow unwraps because plotters has a silly generic
pub fn fullness_histogram(
    mut histogram: Vec<HashSet<u32>>,
    sorted: bool,
    title: &String,
    granularity: i32,
) -> Result<()> {
    if sorted {
        histogram.sort_by(|a, b| b.len().cmp(&a.len()));
    }

    let bin_counts: Vec<(usize, usize)> = histogram
        .iter()
        .enumerate()
        .map(|(idx, set)| (idx, set.len()))
        .collect();

    // Consolidate bins into 30 groups
    let target_bins = granularity;
    let bins_per_group = (bin_counts.len() as f64 / target_bins as f64).ceil() as usize;
    let consolidated_bins: Vec<(usize, usize)> = bin_counts
        .chunks(bins_per_group)
        .enumerate()
        .map(|(idx, chunk)| {
            let total = chunk.iter().map(|(_, count)| count).sum();
            (idx, total)
        })
        .collect();

    let max_count = consolidated_bins
        .iter()
        .map(|(_, count)| count)
        .max()
        .unwrap_or(&0);
    let y_max = (*max_count as f64 * 1.1) as usize;
    let num_bins = consolidated_bins.len();

    let output = String::from(title) + "_histogram.png";
    let root = BitMapBackend::new(&output, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(&title, ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..num_bins, 0..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh() // Remove grid lines
        .x_desc("Bin Number")
        .y_desc("Count")
        .axis_style(BLACK.mix(0.8))
        .draw()
        .unwrap();

    chart
        .draw_series(
            consolidated_bins
                .iter()
                .map(|(idx, count)| Rectangle::new([(*idx, 0), (idx + 1, *count)], RED.filled())),
        )
        .unwrap();

    root.present().unwrap();
    Ok(())
}

pub fn print_table(
    experiment_names: &Vec<String>,
    metadata_vec: &Vec<crate::bm_calc::metadata>,
    emd_vec: &Vec<f64>,
) -> Result<()> {
    let mut results = Vec::new();

    for ((name, meta), emd) in experiment_names
        .iter()
        .zip(metadata_vec.iter())
        .zip(emd_vec.iter())
    {
        results.push(ExperimentResult {
            name: name.clone(),
            bins: meta.num_bins,
            removed: meta.removed_items,
            total: meta.total_items,
            avg_load: meta.average_load_per_bin,
            keywords: meta.keywords_with_overlap,
            emd: *emd,
        });
    }

    let table = Table::new(results).to_string();
    println!("{}", table);

    Ok(())
}
