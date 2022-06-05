use plotters::prelude::*;

use crate::backend::Rep;

pub fn plot(path: &str, mut data: Vec<Rep>, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    data.sort();

    // To get the max `deals` field,
    //  can use map or fold, but if vector is sorted, can just take the last
    // let max_deals = data
    //     .iter()
    //     .map(|r| r.deals)
    //     .collect::<Vec<u32>>()
    //     .iter()
    //     .max()
    //     .unwrap()
    //     .clone();

    //let max_deals = data.iter().fold(0, |acc, r| u32::max(acc, r.deals));

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(title, ("sans-serif", 50.0))
        .build_cartesian_2d(
            (0u32..10u32).into_segmented(),
            0u32..data.last().unwrap().deals,
        )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Deals")
        .x_desc("Rep")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(Rep::get_histogram_data(data).iter().map(|x: &u32| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", path);

    Ok(())
}
