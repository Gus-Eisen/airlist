mod airlist;

use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align, Color};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::{Event, OnEvent};
use std::collections::BTreeMap;

use pelican_ui_std::{Interface, Stack, Page, Text, TextStyle, Offset, Content, Icon, ExpandableText, Header, AppPage, IconButton, ButtonSize, ButtonStyle, ButtonState, NavigateEvent, TextInput, InputEditedEvent};
use crate::airlist::NewListScreen;

#[derive(Debug)]
pub struct LoggedInput(pub String);

impl Event for LoggedInput {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        todo!()
    }
}

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

impl OnEvent for LandingScreen {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        // if let Some(LoggedInput(text)) = event.downcast_ref::<LoggedInput>() {
        //     let font_size = ctx.theme.fonts.size;
        //     let t = Text::new(
        //         ctx,
        //         text.as_str(),
        //         TextStyle::Primary,
        //         font_size.md,
        //         Align::Left,
        //     );
        //     self.1.content().push(Box::new(t));
        //     return true;
        // }
        // if event.downcast_ref::<InputEditedEvent>().is_some() {
        //     let current = NewListScreen.1;
        // }
        true
    }
}


// Implement the AppPage trait for navigation and UI behavior
impl AppPage for LandingScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool { false }

    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(airlist::NewListScreen::new(ctx))),
            _ => Err(self),
        }
    }
}

impl LandingScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let new_list_icon = IconButton::navigation(
            ctx,
            "add",
            |ctx: &mut Context| {
                println!("new_list_icon clicked.");
                ctx.trigger_event(NavigateEvent(0));
            },
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

        LandingScreen(Stack::default(), Page::new(Some(header), content, None))
    }
}