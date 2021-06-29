use crate::compositor::{Component, Context};
use helix_view::graphics::{Margin, Rect, Style};
use helix_view::info::Info;
use tui::buffer::Buffer as Surface;
use tui::widgets::{Block, Borders, Widget};

impl Component for Info {
    fn render(&self, viewport: Rect, surface: &mut Surface, cx: &mut Context) {
        let block = Block::default().title(self.title).borders(Borders::ALL);
        let Info { width, height, .. } = self;
        let (w, h) = (*width + 2, *height + 2);
        // -2 to subtract command line + statusline. a bit of a hack, because of splits.
        let area = Rect::new(viewport.width - w, viewport.height - h - 2, w, h);
        let margin = Margin {
            vertical: 1,
            horizontal: 1,
        };
        let Rect { x, y, .. } = area.inner(&margin);
        for (y, line) in (y..).zip(self.text.lines()) {
            surface.set_string(x, y, line, Style::default());
        }
        block.render(area, surface);
    }
}