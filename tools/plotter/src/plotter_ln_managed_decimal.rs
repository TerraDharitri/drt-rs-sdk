use crate::DrawResult;
use dharitri_sc::types::{LnDecimals, ManagedDecimalSigned};
use dharitri_sc_scenario::api::StaticApi;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

pub fn draw_md_ln(
    canvas: HtmlCanvasElement,
    max_x: f32,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let root = CanvasBackend::with_canvas_object(canvas)
        .unwrap()
        .into_drawing_area();

    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=ln(x), x=1..{max_x}"), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(0f32..max_x, -5f32..5f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    const RANGE_MAX: i32 = 1000;
    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, ln_baseline(x))),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, managed_decimal_ln(x))),
        &GREEN,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

pub fn draw_md_ln_error(
    canvas: HtmlCanvasElement,
    max_x: f32,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let root = CanvasBackend::with_canvas_object(canvas)
        .unwrap()
        .into_drawing_area();

    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=logarithm error, x=1..{max_x}"), font)
        .x_label_area_size(30u32)
        .y_label_area_size(50u32)
        .build_cartesian_2d(0f32..max_x, -0.0001f32..0.0001f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    const RANGE_MAX: i32 = 1000;
    chart.draw_series(LineSeries::new(
        (0..=RANGE_MAX)
            .map(|x| x as f32 * max_x / RANGE_MAX as f32)
            .map(|x| (x, managed_decimal_ln(x) - ln_baseline(x))),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}

fn managed_decimal_ln(x: f32) -> f32 {
    let dec = ManagedDecimalSigned::<StaticApi, LnDecimals>::from(x);
    if let Some(ln_dec) = dec.ln() {
        ln_dec.to_big_float().to_f64() as f32
    } else {
        0.0
    }
}

fn ln_baseline(x: f32) -> f32 {
    x.ln()
}

#[cfg(test)]
mod test {
    #[test]
    fn sc_ln_test() {
        assert_eq!(super::managed_decimal_ln(0.0), 0.0);
        println!("{}", super::managed_decimal_ln(1.0));
        println!("{}", super::managed_decimal_ln(2.0));
        // assert!(super::big_uint_ln(1.0) >= 0.0);
        // assert!(super::big_uint_ln(1.0) < 0.01);
        // assert!(super::big_uint_ln(2.0) > 0.6);
    }
}
