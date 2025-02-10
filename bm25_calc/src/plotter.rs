use std::collections::HashSet;
use plotters::prelude::*;

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
fn fullness_histogram(histogram: Vec<HashSet<u32>>, sorted: bool, title: &String) -> Result<()> {
    // Count items in each bin
    let mut bin_counts: Vec<(usize, usize)> = histogram
        .iter()
        .enumerate()
        .map(|(idx, set)| (idx, set.len()))
        .collect();

    if sorted {
        bin_counts.sort_by(|a, b| b.1.cmp(&a.1));
    }

    let max_count = bin_counts.iter().map(|(_, count)| count).max().unwrap_or(&0);
    let num_bins = bin_counts.len();

    // Create a drawing area on an 800x600 bitmap
    let root = BitMapBackend::new("histogram.png", (800, 600))
        .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(&title, ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0..num_bins,
            0..*max_count
        ).unwrap();

    chart.configure_mesh().draw().unwrap();

    // Draw the histogram bars
    chart.draw_series(
        bin_counts.iter().map(|(idx, count)| {
            let x = *idx;
            let y = *count;
            Rectangle::new(
                [(x, 0), (x + 1, y)],
                RED.filled(),
            )
        }),
    ).unwrap();

    root.present().unwrap();
    Ok(())
}