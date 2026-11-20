macro_rules! catalog {
    ($($group:literal, $icon:literal => { $($page:ident: $name:literal, $description:literal;)+ })+) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum Page { $($($page,)+)+ }

        pub const GROUPS: &[(&str, &str, &[Page])] = &[
            $(($group, $icon, &[$(Page::$page),+])),+
        ];

        impl Page {
            pub fn title(self) -> &'static str {
                match self { $($(Self::$page => $name,)+)+ }
            }

            pub fn description(self) -> &'static str {
                match self { $($(Self::$page => $description,)+)+ }
            }

            pub fn group(self) -> &'static str {
                match self { $($(Self::$page)|+ => $group,)+ }
            }
        }
    };
}

catalog! {
    "Actions", "mouse-pointer-2" => {
        Button: "Button", "Displays a button or a component that looks like a button.";
        ButtonGroup: "Button Group", "Groups related buttons into a single control.";
        Toggle: "Toggle", "A two-state button that can be on or off.";
        ToggleGroup: "Toggle Group", "A set of two-state buttons for related options.";
    }
    "Forms", "sliders-horizontal" => {
        Input: "Input", "A single-line field for entering and editing text.";
        Textarea: "Textarea", "A multiline field for longer messages.";
        Checkbox: "Checkbox", "A control that lets the user choose whether an option is enabled.";
        Radio: "Radio Group", "A set of options where only one can be selected.";
        Switch: "Switch", "Toggles a setting between on and off.";
        Slider: "Slider", "Selects a value from a continuous range.";
        Select: "Select", "Presents a list of choices in a popup.";
        Field: "Field", "Combines a label, a control, and supporting text.";
        InputOtp: "Input OTP", "Enter a one-time passcode using individual slots.";
        Calendar: "Calendar", "Browse months and select a day.";
        DatePicker: "Date Picker", "Pick a single date or a date range from a calendar popup.";
        Combobox: "Combobox", "Search a list of options and select one.";
        InputGroup: "Input Group", "Combine an input with an icon and an action.";
        Label: "Label", "Associate a label with a form control.";
        Questionnaire: "Questionnaire", "Answer a short sequence of questions.";
    }
    "Data display", "layout-grid" => {
        Accordion: "Accordion", "Vertically stacked sections that reveal content when expanded.";
        Avatar: "Avatar", "An image or initials representing a person or account.";
        Badge: "Badge", "A small label for a status, category, or count.";
        Card: "Card", "Groups a heading, description, content, and actions.";
        Empty: "Empty", "Explains why content is missing and what to do next.";
        Kbd: "Kbd", "Displays a keyboard shortcut or key combination.";
        Separator: "Separator", "Visually separates related content.";
        Table: "Table", "Organizes structured data into rows and columns.";
        Typography: "Typography", "Headings and body styles for clear content hierarchy.";
        DataTable: "Data Table", "Sort, filter, and select rows.";
        Chart: "Chart", "Compare monthly activity.";
        Carousel: "Carousel", "Browse a series of cards.";
        CodeBlock: "Code Block", "Read and copy a code snippet.";
        Collapsible: "Collapsible", "Reveal a list of related items.";
        Item: "Item", "Display a title, description, and action in a row.";
        AspectRatio: "Aspect Ratio", "Maintain proportions as the preview resizes.";
        Direction: "Direction", "Compare left-to-right and right-to-left layouts.";
    }
    "Navigation", "compass" => {
        Breadcrumb: "Breadcrumb", "Shows where the current page sits in a hierarchy.";
        Pagination: "Pagination", "Moves between pages of a larger collection.";
        Tabs: "Tabs", "Switches between related panels on the same page.";
        Sidebar: "Sidebar", "Navigate a workspace with collapsible groups and an icon rail.";
        Menubar: "Menubar", "Open actions from a persistent menu bar.";
        NavigationMenu: "Navigation Menu", "Browse groups of navigation links.";
    }
    "Overlays", "panels-top-left" => {
        Dialog: "Dialog", "A window over the page that asks the user to focus on a task.";
        AlertDialog: "Alert Dialog", "Interrupts the user to confirm an important action.";
        Dropdown: "Dropdown Menu", "Reveals a list of actions when a button is pressed.";
        Popover: "Popover", "Displays rich content anchored to a trigger.";
        Sheet: "Sheet", "Opens a panel at the edge of the window.";
        Tooltip: "Tooltip", "Reveals a short description when a control is hovered.";
        Command: "Command", "Filter and run commands.";
        ContextMenu: "Context Menu", "Right-click to open actions for an item.";
        HoverCard: "Hover Card", "Preview a profile by hovering over its name.";
        Drawer: "Drawer", "Open from any edge and drag the drawer to dismiss it.";
    }
    "Feedback", "message-square" => {
        Alert: "Alert", "Draws attention to information that needs to be noticed.";
        Progress: "Progress", "Shows how much of a task has been completed.";
        Skeleton: "Skeleton", "Reserves space for content while it loads.";
        Spinner: "Spinner", "Shows that work is in progress.";
        Toast: "Toast", "Shows a brief notification without blocking the page.";
        ScrollArea: "Scroll Area", "Compare smooth and discrete scrolling in a bounded region.";
        Resizable: "Resizable", "Drag a divider to resize neighboring panels.";
    }
    "Messages", "message-square" => {
        Attachment: "Attachment", "Show file metadata and remove a file from a draft.";
        Bubble: "Bubble", "Style messages by sender.";
        Marker: "Marker", "Mark a boundary in a conversation.";
        Message: "Message", "Display a conversation row with an avatar, bubble, and optional footer.";
        MessageScroller: "Message Scroller", "Follow a streamed reply and return to the latest message.";
        Trace: "Trace", "Inspect the steps and outcomes of a run.";
    }
}
