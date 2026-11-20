// These adapters keep GPUI's composition traits while owning the public type.
macro_rules! element {
    ($(#[$meta:meta])* $name:ident($inner:path) $(+ $kind:ident)*) => {
        $(#[$meta])*
        #[derive(gpui::IntoElement)]
        pub struct $name($inner);

        impl gpui::RenderOnce for $name {
            fn render(self, _: &mut gpui::Window, _: &mut gpui::App) -> impl gpui::IntoElement {
                self.0
            }
        }

        $(element!(@trait $name, $kind);)*
    };
    (@trait $name:ident, Styled) => {
        impl gpui::Styled for $name {
            fn style(&mut self) -> &mut gpui::StyleRefinement {
                gpui::Styled::style(&mut self.0)
            }
        }
    };
    (@trait $name:ident, Parent) => {
        impl gpui::ParentElement for $name {
            fn extend(&mut self, children: impl IntoIterator<Item = gpui::AnyElement>) {
                gpui::ParentElement::extend(&mut self.0, children);
            }
        }
    };
    (@trait $name:ident, Interactive) => {
        impl gpui::InteractiveElement for $name {
            fn interactivity(&mut self) -> &mut gpui::Interactivity {
                gpui::InteractiveElement::interactivity(&mut self.0)
            }
        }
        impl gpui::StatefulInteractiveElement for $name {}
    };
}

macro_rules! builders {
    ($($method:ident($($arg:ident: $arg_type:ty),* $(,)?);)*) => {
        $(
            pub fn $method(mut self, $($arg: $arg_type),*) -> Self {
                self.0 = self.0.$method($($arg),*);
                self
            }
        )*
    };
}

macro_rules! constructor {
    ($name:ident($($arg:ident: $arg_type:ty),* $(,)?)) => {
        impl $name {
            pub fn new($($arg: $arg_type),*) -> Self {
                Self(gpui_base::$name::new($($arg),*))
            }
        }
    };
}
