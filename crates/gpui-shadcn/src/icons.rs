use gpui::*;
use std::borrow::Cow;

/// Loads built-in icon assets. Delegate to this from your application asset source.
pub fn load_icon(path: &str) -> Option<Cow<'static, [u8]>> {
    let markup = match path.strip_prefix("gpui-shadcn/") {
        Some("arrow-up") => "<path d='M12 19V5m-7 7 7-7 7 7'/>",
        Some("rotate-cw") => "<path d='M20 7v5h-5'/><path d='M20 12a8 8 0 1 0-2.34 5.66L20 15'/>",
        Some("message-circle-dashed") => {
            "<path stroke-dasharray='2 4' d='M21 12a9 9 0 0 1-13 8L3 21l1-5a9 9 0 1 1 17-4Z'/>"
        }
        Some("plus") => "<path d='M12 5v14M5 12h14'/>",
        Some("chevrons-up-down") => "<path d='m8 9 4-4 4 4m-8 6 4 4 4-4'/>",
        Some("book-open") => {
            "<path d='M12 7v14m0-14C9 4 5 4 2 5v14c3-1 7-1 10 2 3-3 7-3 10-2V5c-3-1-7-1-10 2Z'/>"
        }
        Some("circle-user") => {
            "<circle cx='12' cy='12' r='9'/><circle cx='12' cy='10' r='3'/><path d='M6 18c1-3 3-4 6-4s5 1 6 4'/>"
        }
        Some("credit-card") => {
            "<rect x='2' y='5' width='20' height='14' rx='2'/><path d='M2 10h20'/>"
        }
        Some("bell") => {
            "<path d='M18 8a6 6 0 0 0-12 0c0 6-3 7-3 8h18c0-1-3-2-3-8m-8 12a2 2 0 0 0 4 0'/>"
        }
        Some("layers") => {
            "<path d='m12 2 10 5-10 5L2 7l10-5Z'/><path d='m2 12 10 5 10-5M2 17l10 5 10-5'/>"
        }
        Some("search") => "<circle cx='11' cy='11' r='8'/><path d='m21 21-4.3-4.3'/>",
        Some("mouse-pointer-2") => "<path d='m4 4 7.07 17 2.51-7.39L21 11 4 4Z'/>",
        Some("sliders-horizontal") => {
            "<path d='M3 6h7m4 0h7M3 12h12m4 0h2M3 18h3m4 0h11M10 4v4m5 2v4M6 16v4'/>"
        }
        Some("layout-grid") => {
            "<rect x='3' y='3' width='7' height='7' rx='1'/><rect x='14' y='3' width='7' height='7' rx='1'/><rect x='3' y='14' width='7' height='7' rx='1'/><rect x='14' y='14' width='7' height='7' rx='1'/>"
        }
        Some("compass") => {
            "<circle cx='12' cy='12' r='10'/><path d='m16 8-2.5 5.5L8 16l2.5-5.5L16 8Z'/>"
        }
        Some("panels-top-left") => {
            "<rect x='3' y='3' width='18' height='18' rx='2'/><path d='M3 9h18M9 9v12'/>"
        }
        Some("message-square") => {
            "<path d='M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2Z'/>"
        }
        Some("chevron-right") => "<path d='m9 18 6-6-6-6'/>",
        Some("chevron-down") => "<path d='m6 9 6 6 6-6'/>",
        Some("check") => "<path d='m20 6-11 11-5-5'/>",
        Some("arrow-down") => "<path d='M12 5v14m-7-7 7 7 7-7'/>",
        Some("arrow-right") => "<path d='M5 12h14m-7-7 7 7-7 7'/>",
        Some("arrow-left") => "<path d='M19 12H5m7 7-7-7 7-7'/>",
        Some("moon") => "<path d='M20.9 13.35A9 9 0 0 1 10.65 3.1 9 9 0 1 0 20.9 13.35Z'/>",
        Some("sun") => {
            "<circle cx='12' cy='12' r='4'/><path d='M12 2v2m0 16v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2m-17.07 7.07 1.42-1.42m11.3-11.3 1.42-1.42'/>"
        }
        Some("circle-info") => "<circle cx='12' cy='12' r='10'/><path d='M12 11v5m0-8h.01'/>",
        Some("circle-alert") => "<circle cx='12' cy='12' r='10'/><path d='M12 8v4m0 4h.01'/>",
        Some("terminal") => "<path d='m4 17 6-5-6-5m8 12h8'/>",
        Some("inbox") => {
            "<path d='M22 12h-6l-2 3h-4l-2-3H2'/><path d='M5.45 5.1 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.9A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.1Z'/>"
        }
        Some("x") => "<path d='m18 6-12 12M6 6l12 12'/>",
        Some("minus") => "<path d='M5 12h14'/>",
        Some("maximize") => "<rect x='5' y='5' width='14' height='14' rx='2'/>",
        Some("restore") => {
            "<rect x='4' y='8' width='12' height='12' rx='2'/><path d='M8 8V6a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2'/>"
        }
        Some("loader") => "<path d='M21 12a9 9 0 1 1-9-9'/>",
        _ => return None,
    };
    Some(Cow::Owned(format!(
            "<svg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='black' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'>{markup}</svg>"
        ).into_bytes()))
}

pub fn icon(name: &'static str, size: Pixels, color: Hsla) -> Svg {
    svg()
        .path(format!("gpui-shadcn/{name}"))
        .size(size)
        .flex_shrink_0()
        .text_color(color)
}
