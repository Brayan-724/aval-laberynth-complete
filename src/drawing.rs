use std::borrow::Borrow;
use std::path::Path;

use plotters::coord::types::RangedCoordf32;
use plotters::coord::Shift;
use plotters::element::{CoordMapper, Drawable, PointCollection};
use plotters::prelude::*;

type Chart<'a, 'b> =
    ChartContext<'a, BitMapBackend<'b>, Cartesian2d<RangedCoordf32, RangedCoordf32>>;
type Root<'a> = DrawingArea<BitMapBackend<'a>, Shift>;

pub enum DrawingContext<'root: 'chart, 'chart> {
    NoDraw,
    GIF(Chart<'chart, 'root>),
}

impl DrawingContext<'_, '_> {
    pub fn create_root<'a>(
        file: impl AsRef<Path>,
        size: u32,
    ) -> Result<Root<'a>, Box<dyn std::error::Error>> {
        Ok(BitMapBackend::gif(file, (size, size), 60)?.into_drawing_area())
    }

    pub fn create_chart<'root, 'chart>(
        root: &'chart Root<'root>,
        cells: f32,
    ) -> Result<Chart<'chart, 'root>, Box<dyn std::error::Error>>
    where
        'root: 'chart,
    {
        Ok(ChartBuilder::on(&root)
            .top_x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(0f32..cells, cells..0f32)?)
    }

    pub fn new_gif<'root: 'chart, 'chart>(
        root: &'chart Root<'root>,
        cells: f32,
    ) -> Result<DrawingContext<'root, 'chart>, Box<dyn std::error::Error>> {
        let mut chart = DrawingContext::create_chart(root, cells)?;

        root.fill(&WHITE)?;

        chart
            .configure_mesh()
            .x_labels(20)
            .y_labels(20)
            .max_light_lines(4)
            .disable_axes()
            .disable_mesh()
            .draw()?;

        Ok(DrawingContext::GIF(chart))
    }
}

impl DrawingContext<'_, '_> {
    pub fn present(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let DrawingContext::GIF(chart) = self {
            chart.plotting_area().present()?;
        }

        Ok(())
    }

    pub fn draw<Co, El>(
        &mut self,
        element: impl Borrow<El>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        Co: CoordMapper,
        for<'b> &'b El: PointCollection<
            'b,
            <Cartesian2d<RangedCoordf32, RangedCoordf32> as CoordTranslate>::From,
            Co,
        >,
        for<'root> El: Drawable<BitMapBackend<'root>, Co>,
    {
        if let DrawingContext::GIF(chart) = self {
            chart.draw_series(std::iter::once(element))?;
        }

        Ok(())
    }

    // pub fn draw_series<Co, El, Rf>(
    //     &mut self,
    //     series: impl IntoIterator<Item = Rf>,
    // ) -> Result<(), Box<dyn std::error::Error>>
    // where
    //     Co: CoordMapper,
    //     for<'b> &'b El: PointCollection<
    //         'b,
    //         <Cartesian2d<RangedCoordf32, RangedCoordf32> as CoordTranslate>::From,
    //         Co,
    //     >,
    //     for<'root> El: Drawable<BitMapBackend<'root>, Co>,
    //     Rf: Borrow<El>,
    // {
    //     if let DrawingContext::GIF(chart) = self {
    //         chart.draw_series(series)?;
    //     }
    //
    //     Ok(())
    // }

    pub fn draw_text<'a>(
        &mut self,
        text: &str,
        pos: (f32, f32),
        style: impl Into<TextStyle<'a>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let DrawingContext::GIF(chart) = self {
            chart.plotting_area().draw(&Text::new(text, pos, style))?;
        }

        Ok(())
    }

    pub fn fill(&mut self, color: &impl Color) -> Result<(), Box<dyn std::error::Error>> {
        if let DrawingContext::GIF(chart) = self {
            chart.plotting_area().fill(color)?;
        }

        Ok(())
    }
}
