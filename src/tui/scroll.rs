//! Bounded, disposable viewport state shared by read-only workbench panels.

use std::cell::Cell;

use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

/// Retains a panel's wrapped-row offset and its most recently rendered navigation bounds.
#[derive(Debug, Clone, Default)]
pub(super) struct PanelScroll {
    offset: Cell<u16>,
    maximum: Cell<u16>,
    page_size: Cell<u16>,
}

impl PanelScroll {
    /// Consumes row, page, and boundary keys without allowing navigation past either end.
    pub(super) fn handle_key(&self, code: KeyCode) -> bool {
        let offset = self.offset.get();
        let page = self.page_size.get().max(1);
        let maximum = self.maximum.get();
        let next = match code {
            KeyCode::Up => offset.saturating_sub(1),
            KeyCode::Down => offset.saturating_add(1),
            KeyCode::PageUp => offset.saturating_sub(page),
            KeyCode::PageDown => offset.saturating_add(page),
            KeyCode::Home => 0,
            KeyCode::End => maximum,
            _ => return false,
        };
        self.offset.set(next.min(maximum));
        true
    }

    /// Measures an unbordered paragraph inside panel borders and clamps after resize or refresh.
    pub(super) fn update(&self, area: Rect, paragraph: &Paragraph<'_>) -> u16 {
        let width = area.width.saturating_sub(2);
        let height = area.height.saturating_sub(2);
        let maximum = paragraph
            .line_count(width)
            .saturating_sub(usize::from(height));
        let maximum = u16::try_from(maximum).unwrap_or(u16::MAX);
        self.maximum.set(maximum);
        self.page_size.set(height.max(1));
        let offset = self.offset.get().min(maximum);
        self.offset.set(offset);
        offset
    }
}
