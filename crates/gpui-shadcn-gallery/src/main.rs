#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod catalog;
mod demos;
mod gallery;
use gpui_shadcn::{icon, motion, scroll, theme};

use gpui_shadcn::gpui::*;
use std::borrow::Cow;

struct GalleryAssets;

impl AssetSource for GalleryAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let avatar: Option<&'static [u8]> = match path {
            "avatars/02.png" => Some(include_bytes!("../assets/avatars/02.png")),
            "avatars/10.png" => Some(include_bytes!("../assets/avatars/10.png")),
            _ => None,
        };
        if let Some(avatar) = avatar {
            return Ok(Some(Cow::Borrowed(avatar)));
        }
        Ok(gpui_shadcn::load_icon(path))
    }

    fn list(&self, _: &str) -> Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}

const WINDOW_TITLE: &str = "GPUI Shadcn Gallery";

fn main() {
    gpui_platform::application()
        .with_assets(GalleryAssets)
        .run(|cx| {
            gpui_shadcn::init(cx);
            gpui_shadcn::load_geist(cx).expect("load bundled Geist fonts");
            let mut theme = gpui_shadcn::Theme::global(cx);
            theme.typography.sans = Some("Geist");
            theme.set(cx);
            let preferred = size(px(1200.), px(800.));
            let minimum = size(px(700.), px(540.));
            let bounds = cx.primary_display().map_or_else(
                || Bounds::new(point(px(0.), px(0.)), preferred),
                |display| {
                    let area = display.visible_bounds();
                    let available = size(
                        (area.size.width - px(32.)).max(minimum.width),
                        (area.size.height - px(32.)).max(minimum.height),
                    );
                    Bounds::centered_at(area.center(), preferred.min(&available).min(&area.size))
                },
            );
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(WINDOW_TITLE.into()),
                        appears_transparent: cfg!(any(target_os = "macos", target_os = "windows")),
                        traffic_light_position: cfg!(target_os = "macos")
                            .then(|| point(px(16.), px(13.))),
                    }),
                    window_min_size: Some(minimum.min(&bounds.size)),
                    ..Default::default()
                },
                |window, cx| {
                    let gallery = cx.new(|cx| gallery::Gallery::new(window, cx));
                    gpui_shadcn::root(gallery, window, cx)
                },
            )
            .expect("open component gallery");
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            cx.activate(true);
        });
}
