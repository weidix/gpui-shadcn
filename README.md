# gpui-shadcn

A Rust component library for native [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui)
applications, inspired by the Neutral theme from [shadcn/ui](https://ui.shadcn.com/).
The library provides theme tokens, a window root for overlays, and reusable
controls. Gallery is an interactive desktop application built with the same
public APIs.

## Try Gallery

Download a package from [GitHub Releases](https://github.com/weidix/gpui-shadcn/releases).
Builds from individual commits are available as
[GitHub Actions artifacts](https://github.com/weidix/gpui-shadcn/actions/workflows/gallery.yml).
Each Actions artifact contains a platform archive. Extract the archive before
launching.

| Platform | Package | Launch |
| --- | --- | --- |
| Windows x64 | ZIP | Open `gpui-shadcn.exe`. |
| macOS Apple Silicon or Intel | ZIP | Open `GPUI Shadcn Gallery.app`. The bundle is ad-hoc signed and has no Apple notarization. |
| Linux x64 | TAR.GZ | Run `./gpui-shadcn` from the extracted directory. Built on Ubuntu 24.04. Use a graphical Wayland or X11 session with a working Vulkan driver. |

The sidebar groups components by purpose. Use the search box or press
**Cmd/Ctrl + K** to find a component. The **Settings** menu changes the color
mode, density, and corner radius. Multi-example pages let you switch between
examples without losing their input state.

## Build from source

Install [rustup](https://rustup.rs/). The repository pins Rust 1.100.0 in
[`rust-toolchain.toml`](rust-toolchain.toml). Install your platform's native
build dependencies:

- **Windows.** Visual Studio Build Tools with Desktop development with C++ and
  the Windows SDK.
- **macOS.** Xcode with its command-line tools selected. If
  `xcrun -sdk macosx metal --version` fails, install the Metal toolchain with
  `xcodebuild -downloadComponent MetalToolchain`.
- **Ubuntu 24.04.** Install the packages below. The Vulkan loader and a graphics
  driver are also required to run Gallery.

```sh
sudo apt-get install clang cmake libasound2-dev libfontconfig1-dev libfreetype6-dev \
  libssl-dev libwayland-dev libx11-xcb-dev libxcb1-dev libxkbcommon-dev \
  libxkbcommon-x11-dev pkg-config
```

```sh
cargo run --locked -p gpui-shadcn-gallery
cargo build --locked --release -p gpui-shadcn-gallery
```

## Use the library in a GPUI application

Add the library from a local checkout. Use a matching GPUI platform backend in
your executable.

```toml
[dependencies]
gpui-shadcn = { path = "../gpui-shadcn/crates/gpui-shadcn" }
gpui_platform = { package = "gpui-pre-platform", version = "=0.3.10" }
```

On Linux, enable the `x11` and/or `wayland` features on `gpui_platform`.
Import GPUI types through `gpui_shadcn::gpui` for compatibility with the library.
Call `gpui_shadcn::init(cx)` at startup and wrap each window's root view in
`gpui_shadcn::root` to install the host for overlays and text selection.

```rust
use gpui_shadcn::{
    Button, Density, Theme, Variant,
    gpui::{App, AppContext, Context, IntoElement, Render, Window, WindowOptions},
};

struct Example;

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Button::new("toggle-theme", "Toggle theme", Variant::Primary)
            .on_click(|_, _, cx| {
                let theme = Theme::global(cx);
                theme.with_dark(!theme.dark).set(cx);
            })
    }
}

fn main() {
    gpui_platform::application().run(|cx| {
        open_example(cx).expect("open example window");
        cx.activate(true);
    });
}

fn open_example(cx: &mut App) -> gpui_shadcn::gpui::Result<()> {
    gpui_shadcn::init(cx);
    Theme::new(false, Density::Default, Theme::DEFAULT_RADIUS).set(cx);
    cx.open_window(WindowOptions::default(), |window, cx| {
        let view = cx.new(|_| Example);
        gpui_shadcn::root(view, window, cx)
    })?;
    Ok(())
}
```

`Theme::set(cx)` refreshes open windows. Components resolve the current theme
when they render. Controls such as Button, Select, and Input also accept local
theme overrides with `with_theme`. The library theme and host theme can be
configured independently.

- `input` provides text fields, text areas, and their state and events.
- `form` and `calendar` provide form fields, sliders, toggle and radio groups,
  verification code inputs, and date and date-range pickers.
- `display` provides badges, cards, progress indicators, tables, tabs, and
  other display components.
- `overlay` provides popovers, dialogs, sheets, hover cards, tooltips, and
  toasts.

For built-in icons, delegate to `gpui_shadcn::load_icon(path)` in your
application's `AssetSource::load`. For Geist, enable the library's `geist`
feature and call `gpui_shadcn::load_geist(cx)` during startup.

## Development and releases

```sh
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo test --locked --workspace --all-features
```

GitHub Actions runs these checks and builds the Gallery packages on native
runners. Pushing a version tag such as `v0.1.0` publishes a GitHub Release with
all platform packages and SHA-256 checksums after every build succeeds.

When dependencies change, regenerate the Gallery notices with
[`cargo-about` 0.9.3](https://github.com/EmbarkStudios/cargo-about/releases/tag/0.9.3).

```sh
cargo about generate --locked --fail --config .github/scripts/licenses.toml \
  --output-file THIRD-PARTY-LICENSES.html .github/scripts/licenses.hbs
```

## License

Project source is [MIT licensed](LICENSE). Bundled Geist fonts are distributed
under the [SIL Open Font License](crates/gpui-shadcn/assets/OFL.txt).
The [Lucide and Feather notices](crates/gpui-shadcn/assets/LUCIDE-LICENSE.txt)
cover the built-in icons derived from those projects. Gallery distributions
also include [third-party dependency notices](THIRD-PARTY-LICENSES.html).
