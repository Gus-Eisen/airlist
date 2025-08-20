mod airlist;

use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align, Color};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::OnEvent;
use std::collections::BTreeMap;

use pelican_ui_std::{Interface, Stack, Page, Text, TextStyle, Offset, Content, Icon, ExpandableText, Header, AppPage, IconButton, ButtonSize, ButtonStyle, ButtonState};

// Define the main application struct. This is our entry point type.
pub struct MyApp;

impl Services for MyApp {
    fn services() -> ServiceList {
        ServiceList::default()
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![]
    }
}

impl Application for MyApp {
    // Asynchronously create the main drawable UI component
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        // Create the first screen
        let home = LandingScreen::new(ctx);
        // Create the main interface with the first screen as the starting page
        let interface = Interface::new(ctx, Box::new(home), None, None);
        // Return the interface wrapped in a Box
        Box::new(interface)
    }
}

start!(MyApp);

#[derive(Debug, Component)]
pub struct LandingScreen(Stack, Page);

// Implement event handling for FirstScreen (empty for now)
impl OnEvent for LandingScreen {}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for LandingScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool { false }

    // Handle page navigation. Always returns Err(self) because this page cannot navigate.
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        Err(self)
    }
}

impl LandingScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let new_list_icon = IconButton::new(
            ctx,
            "add",
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Box::new(|_ctx: &mut Context| {
                ()
            }),
            None,
        );
        // Create a header for the page
        let header = Header::home(
            // The majority of UI components will require the app context.
            ctx,
            // The text on this header will say "AirList"
            "AirList",
            // TODO: delete this if keeping Some: There will not be an icon button on this header
            Some(new_list_icon)
        );

        let font_size = ctx.theme.fonts.size;
        // let color = ctx.theme.colors.text.heading;


        // // Create an icon element
        // let icon = Icon::new(
        //     // This element requires the app context
        //     ctx,
        //     "add",
        //     // The color of the icon
        //     color,
        //     // The size of the icon. Icons are always square.
        //     50.0
        // );

        // Create the main heading text
        let text = Text::new(
            ctx,
            "Welcome to AirList",
            // The style of this text will be heading
            TextStyle::Heading,
            // The size will be h2
            font_size.h3,
            // The text alignment
            Align::Center
        );

        // Create subtext.
        let subtext = ExpandableText::new(
            ctx,
            "Click the icon in the top right corner to create a new list.",
            // This text will have primary text style.
            TextStyle::Primary,
            // Medium font size
            font_size.md,
            // Center the text
            Align::Center,
            // No max lines
            None
        );

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            // Vertically center items
            Offset::Center,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text), Box::new(subtext)]
        );

        // Return the FirstScreen with a default Stack and a
        // new Page containing our header, content, and no bumper.
        LandingScreen(Stack::default(), Page::new(Some(header), content, None))
    }
}