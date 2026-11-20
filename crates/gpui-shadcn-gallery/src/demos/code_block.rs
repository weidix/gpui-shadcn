use super::Demos;
use crate::{
    gallery::Gallery,
    theme::{Theme, Variant, column, row},
};
use gpui_shadcn::CodeBlock;
use gpui_shadcn::gpui::{prelude::FluentBuilder, *};
use std::{ops::Range, sync::OnceLock};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Example {
    Rust,
    Json,
    Diff,
}

type Highlights = Vec<(Range<usize>, HighlightStyle)>;

impl Example {
    const ALL: [(Self, &'static str); 3] = [
        (Self::Rust, "Rust"),
        (Self::Json, "JSON"),
        (Self::Diff, "Diff"),
    ];

    fn sample(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Rust => (
                "main.rs",
                "rs",
                r#"fn main() {
    // Start the component preview.
    let components = ["Button", "Tooltip", "Code Block", "Accordion", "Date Picker"];
    for component in components {
        println!("Previewing {component}");
    }
}"#,
            ),
            Self::Json => (
                "components.json",
                "json",
                r#"{
  "style": "new-york",
  "theme": "neutral",
  "components": ["Button", "Tooltip", "Code Block"],
  "animations": true,
  "radius": 14
}"#,
            ),
            Self::Diff => (
                "main.rs.diff",
                "diff",
                r#"@@ -1,4 +1,5 @@
 fn main() {
-    println!("Starting app");
+    println!("Starting component gallery");
+    println!("Preview ready");
 }"#,
            ),
        }
    }

    fn highlights(self, dark: bool) -> &'static Highlights {
        static SYNTAXES: OnceLock<SyntaxSet> = OnceLock::new();
        static THEMES: OnceLock<ThemeSet> = OnceLock::new();
        static CACHE: [OnceLock<Highlights>; 6] = [const { OnceLock::new() }; 6];
        CACHE[self as usize * 2 + usize::from(dark)].get_or_init(|| {
            let (_, language, code) = self.sample();
            let syntaxes = SYNTAXES.get_or_init(SyntaxSet::load_defaults_newlines);
            let themes = THEMES.get_or_init(ThemeSet::load_defaults);
            let theme = &themes.themes[if dark {
                "base16-ocean.dark"
            } else {
                "InspiredGitHub"
            }];
            let syntax = syntaxes
                .find_syntax_by_extension(language)
                .unwrap_or_else(|| syntaxes.find_syntax_plain_text());
            let mut parser = HighlightLines::new(syntax, theme);
            let mut offset = 0;
            let mut highlights = Vec::new();
            for line in LinesWithEndings::from(code) {
                let background = match (self, line.as_bytes().first()) {
                    (Self::Diff, Some(b'+')) => {
                        Some(rgb(if dark { 0x153a25 } else { 0xdbf6e5 }).into())
                    }
                    (Self::Diff, Some(b'-')) => {
                        Some(rgb(if dark { 0x48242b } else { 0xffe3e6 }).into())
                    }
                    _ => None,
                };
                if let Ok(tokens) = parser.highlight_line(line, syntaxes) {
                    let mut start = offset;
                    highlights.extend(tokens.into_iter().map(|(style, text)| {
                        let end = start + text.len();
                        let range = start..end;
                        start = end;
                        let color = if self == Self::Diff && background.is_some() {
                            if line.starts_with('+') {
                                rgb(if dark { 0x9ce6b6 } else { 0x14532d })
                            } else {
                                rgb(if dark { 0xffb4bc } else { 0x881337 })
                            }
                        } else {
                            rgb(u32::from_be_bytes([
                                0,
                                style.foreground.r,
                                style.foreground.g,
                                style.foreground.b,
                            ]))
                        };
                        (
                            range,
                            HighlightStyle {
                                color: Some(color.into()),
                                background_color: background,
                                ..Default::default()
                            },
                        )
                    }));
                }
                offset += line.len();
            }
            highlights
        })
    }
}

impl Demos {
    pub(super) fn code_block(&self, t: Theme, cx: &Context<Gallery>) -> Div {
        let example = self.extra.code_example;
        let wrap = self.extra.code_wrap;
        let (filename, language, code) = example.sample();
        let copied = self.status.as_deref() == Some("Code copied.");
        let source = CodeBlock::new(("highlighted-source", example as usize), code)
            .language(language)
            .highlights(move |_, theme| example.highlights(theme.dark).clone());
        let min_width = code
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as f32
            * 8.;

        column()
            .w_full()
            .max_w(t.unit(570.))
            .gap(t.unit(8.))
            .child(
                row()
                    .gap(t.unit(4.))
                    .children(Example::ALL.map(|(item, label)| {
                        gpui_shadcn::Button::new(
                            label,
                            label,
                            if example == item {
                                Variant::Secondary
                            } else {
                                Variant::Ghost
                            },
                        )
                        .selected(example == item)
                        .h(t.unit(28.))
                        .on_click(Self::action(cx, move |d| {
                            d.extra.code_example = item;
                            d.status = None;
                        }))
                    })),
            )
            .child(
                t.surface()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        row()
                            .justify_between()
                            .gap(t.unit(8.))
                            .px(t.unit(12.))
                            .py(t.unit(4.))
                            .border_b(t.borders.width)
                            .border_color(t.border)
                            .child(
                                row()
                                    .min_w_0()
                                    .gap(t.unit(8.))
                                    .child(crate::icon("terminal", t.unit(14.), t.muted_foreground))
                                    .child(
                                        div()
                                            .truncate()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(filename),
                                    ),
                            )
                            .child(
                                row()
                                    .flex_shrink_0()
                                    .gap(t.unit(4.))
                                    .child(
                                        gpui_shadcn::Button::new(
                                            "wrap-code",
                                            "Wrap",
                                            if wrap {
                                                Variant::Secondary
                                            } else {
                                                Variant::Ghost
                                            },
                                        )
                                        .h(t.unit(28.))
                                        .selected(wrap)
                                        .on_click(
                                            Self::action(cx, |d| {
                                                d.extra.code_wrap = !d.extra.code_wrap
                                            }),
                                        ),
                                    )
                                    .child(
                                        gpui_shadcn::Button::new(
                                            "copy-code",
                                            if copied { "Copied" } else { "Copy" },
                                            Variant::Ghost,
                                        )
                                        .h(t.unit(28.))
                                        .accessibility_label("Copy code")
                                        .on_click(
                                            cx.listener(move |g, _, _, cx| {
                                                cx.write_to_clipboard(ClipboardItem::new_string(
                                                    code.into(),
                                                ));
                                                g.demos.status = Some("Code copied.".into());
                                                cx.notify();
                                            }),
                                        ),
                                    ),
                            ),
                    )
                    .child(
                        row()
                            .items_start()
                            .bg(t.muted)
                            .when(!wrap, |s| {
                                s.child(
                                    column()
                                        .flex_shrink_0()
                                        .px(t.unit(12.))
                                        .py(t.unit(12.))
                                        .text_color(t.muted_foreground)
                                        .text_size(t.unit(t.typography.xs.size))
                                        .line_height(t.unit(22.))
                                        .children((1..=code.lines().count()).map(|i| {
                                            div().h(t.unit(22.)).text_right().child(i.to_string())
                                        })),
                                )
                            })
                            .child(
                                div()
                                    .id(("code-scroll", example as usize))
                                    .flex_1()
                                    .min_w_0()
                                    .when(!wrap, |s| self.extra.code_scroll.horizontal(s))
                                    .child(
                                        div()
                                            .w_full()
                                            .px(t.unit(12.))
                                            .py(t.unit(12.))
                                            .when(!wrap, |s| s.min_w(t.unit(min_width + 24.)))
                                            .child(source),
                                    ),
                            ),
                    ),
            )
    }
}
