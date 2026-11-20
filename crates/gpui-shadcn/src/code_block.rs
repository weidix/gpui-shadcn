use gpui::*;
use std::{ops::Range, sync::Arc};

use crate::Theme;

type Highlights = Arc<dyn Fn(&str, Theme) -> Vec<(Range<usize>, HighlightStyle)> + Send + Sync>;

/// Selectable, themed source text with optional syntax highlighting.
#[derive(IntoElement)]
pub struct CodeBlock {
    id: ElementId,
    source: SharedString,
    highlights: Option<Highlights>,
    language: SharedString,
}

impl CodeBlock {
    pub fn new(id: impl Into<ElementId>, source: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
            highlights: None,
            language: "".into(),
        }
    }

    /// Sets the language name used when copying selected source as Markdown.
    pub fn language(mut self, language: impl Into<SharedString>) -> Self {
        let language = language.into();
        if language
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || "+#._-".contains(ch))
        {
            self.language = language;
        }
        self
    }

    /// Accepts UTF-8 byte ranges in the source text.
    pub fn highlights(
        mut self,
        highlight: impl Fn(&str, Theme) -> Vec<(Range<usize>, HighlightStyle)> + Send + Sync + 'static,
    ) -> Self {
        self.highlights = Some(Arc::new(highlight));
        self
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::global(cx);
        let mut code_style = t
            .text(div().border_0().p_0(), t.typography.code)
            .font_family(t.typography.mono);
        let style = gpui_base::TextViewStyle::default()
            .with_foreground(t.foreground)
            .with_muted_foreground(t.muted_foreground)
            .with_code_background(t.muted)
            .with_selection(t.selection)
            .with_code_block(code_style.style().clone());

        let longest_fence = self
            .source
            .split(|ch| ch != '`')
            .map(str::len)
            .max()
            .unwrap_or(0);
        let fence = "`".repeat((longest_fence + 1).max(3));
        let markdown = format!("{fence}{}\n{}\n{fence}", self.language, self.source);
        let view = gpui_base::TextView::markdown(self.id, markdown)
            .style(style)
            .selectable(true)
            .code_block_actions(|_, _, _| Empty);
        match self.highlights {
            Some(highlights) => {
                let source = self.source;
                view.code_block_highlighter(move |_| highlights(&source, t))
            }
            None => view,
        }
    }
}
