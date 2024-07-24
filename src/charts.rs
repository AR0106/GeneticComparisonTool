use charts::{Chart, VerticalBarView, ScaleBand, ScaleLinear};
use std::collections::HashMap;

const OUTFILE: &str = "output.svg";

pub fn generate_chart(data: HashMap<String, u32>) {
    // Define chart related sizes.
    let width = data.len() as i32 as isize * 60;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let mut keys: Vec<String> = data.keys().cloned().collect();
    let values: Vec<i32> = data.values().map(|uVal| {*uVal as i32}).collect();

    let mut converted_data: Vec<(&str, i32)> = keys.iter().zip(values.iter()).map(|(k, v)| (k.as_str(), *v)).collect();
    converted_data.sort_by(|a, b| a.1.cmp(&b.1));

    let keys = converted_data.iter().map(|(k, _)| String::from(*k)).rev().collect();

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(keys)
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, *data.values().max().unwrap() as f32])
        .set_range(vec![height - top - bottom, 0]);

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(&converted_data).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Nucleotide Sequence Frequency"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Occurrences")
        .add_bottom_axis_label("Nucleotide Sequence")
        .save(OUTFILE).unwrap();
}