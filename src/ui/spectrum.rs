//! Spectrum visualizer widget for displaying audio frequency data.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
    buffer::Buffer,
};
use crate::models::AudioSpectrum;

/// A widget that renders an audio spectrum visualizer
pub struct SpectrumWidget<'a> {
    spectrum: &'a AudioSpectrum,
    block: Option<Block<'a>>,
    bar_width: u16,
    bar_gap: u16,
    style: Style,
}

impl<'a> SpectrumWidget<'a> {
    /// Create a new spectrum widget
    pub fn new(spectrum: &'a AudioSpectrum) -> Self {
        Self {
            spectrum,
            block: None,
            bar_width: 2,
            bar_gap: 1,
            style: Style::default(),
        }
    }

    /// Set the block (border) for the widget
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set the width of each frequency bar
    pub fn bar_width(mut self, width: u16) -> Self {
        self.bar_width = width.max(1);
        self
    }

    /// Set the gap between frequency bars
    pub fn bar_gap(mut self, gap: u16) -> Self {
        self.bar_gap = gap;
        self
    }

    /// Set the style for the spectrum bars
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for SpectrumWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if area.height < 2 || area.width < 3 {
            return; // Not enough space to render
        }

        let bands = self.spectrum.get_bands();
        if bands.is_empty() {
            return;
        }

        // Calculate how many bars we can fit
        let total_bar_space = (self.bar_width + self.bar_gap) as usize;
        let max_bars = (area.width as usize) / total_bar_space;
        let bars_to_show = bands.len().min(max_bars);

        if bars_to_show == 0 {
            return;
        }

        // Center the bars horizontally
        let total_width = bars_to_show * total_bar_space - self.bar_gap as usize;
        let start_x = area.x + (area.width.saturating_sub(total_width as u16)) / 2;

        for (i, &band_value) in bands.iter().take(bars_to_show).enumerate() {
            let bar_x = start_x + (i * total_bar_space) as u16;
            
            // Ensure minimum bar height for visualization (even with zero values)
            let normalized_value = band_value.max(0.05); // Minimum 5% height
            let bar_height = ((normalized_value * area.height as f32) as u16).max(1).min(area.height);
            let bar_start_y = area.y + area.height - bar_height;

            // Render each bar
            for bar_col in 0..self.bar_width {
                let x = bar_x + bar_col;
                if x >= area.x + area.width {
                    break;
                }

                for y in bar_start_y..(area.y + area.height) { // Use full area height
                    let height_from_bottom = (area.y + area.height - y) as f32;
                    let height_ratio = height_from_bottom / area.height as f32;
                    let color = get_spectrum_color(height_ratio, band_value);
                    
                    let style = Style::default().fg(color);
                    
                    // Use different characters for different heights to create a smooth effect
                    let character = if height_ratio > 0.8 {
                        "█" // Full block
                    } else if height_ratio > 0.6 {
                        "▆" // 3/4 block
                    } else if height_ratio > 0.4 {
                        "▄" // Half block
                    } else if height_ratio > 0.2 {
                        "▂" // Quarter block
                    } else {
                        "▁" // Thin block
                    };

                    buf[(x, y)].set_symbol(character).set_style(style);
                }
            }
        }
    }
}

/// Get color for spectrum bar based on height and intensity
fn get_spectrum_color(height_ratio: f32, intensity: f32) -> Color {
    // Create a gradient from green (low) to yellow (mid) to red (high)
    // Also consider the intensity for brightness
    let brightness = (intensity * 255.0) as u8;
    
    match height_ratio {
        h if h > 0.8 => Color::Rgb(brightness, 0, 0),           // Red (high frequencies)
        h if h > 0.6 => Color::Rgb(brightness, brightness/2, 0), // Orange-red
        h if h > 0.4 => Color::Rgb(brightness, brightness, 0),   // Yellow
        h if h > 0.2 => Color::Rgb(brightness/2, brightness, 0), // Yellow-green  
        _ => Color::Rgb(0, brightness, 0),                      // Green (low frequencies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_color_gradients() {
        // Test that colors change appropriately with height
        let low_color = get_spectrum_color(0.1, 1.0);
        let mid_color = get_spectrum_color(0.5, 1.0);
        let high_color = get_spectrum_color(0.9, 1.0);
        
        // Colors should be different for different heights
        assert_ne!(format!("{:?}", low_color), format!("{:?}", high_color));
        assert_ne!(format!("{:?}", mid_color), format!("{:?}", high_color));
    }

    #[test]
    fn test_widget_creation() {
        let spectrum = AudioSpectrum::new(8);
        let widget = SpectrumWidget::new(&spectrum);
        
        // Basic creation should work
        assert_eq!(widget.bar_width, 2);
        assert_eq!(widget.bar_gap, 1);
    }

    #[test]
    fn test_widget_configuration() {
        let spectrum = AudioSpectrum::new(8);
        let widget = SpectrumWidget::new(&spectrum)
            .bar_width(3)
            .bar_gap(2);
        
        assert_eq!(widget.bar_width, 3);
        assert_eq!(widget.bar_gap, 2);
    }
}
