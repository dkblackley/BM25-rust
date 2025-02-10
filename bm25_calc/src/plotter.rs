use std::collections::HashSet;
use plotters::prelude::*;

use crate::error::Result;


/// Takes in a Hashset and plots a histogram showing the size of each bin.
///
/// # Arguments
///
/// * `histogram`: A hashset of u32, each item in the hashset
///
/// returns: Result<(), BM25Error>
fn fullness_histogram(histogram: HashSet<u32>) -> Result<()> {
    // Create a drawing area on an 800x600 bitmap
    let root = BitMapBackend::new("line-chart.png", (800, 600))
        .into_drawing_area();

    // Clean the drawing area
    root.fill(&WHITE)?;

    // Create a chart with a specified dimension
    let mut chart = ChartBuilder::on(&root)
        .caption("Sample Line Chart", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)?;

    // Configure the mesh grid
    chart.configure_mesh()
        .draw()?;

    // Draw the sine wave
    chart.draw_series(LineSeries::new(
        (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}