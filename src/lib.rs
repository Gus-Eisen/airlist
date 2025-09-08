mod airlist;

use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align, Color};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::{Event, OnEvent};
use std::collections::BTreeMap;

use pelican_ui_std::{Interface, Stack, Page, Text, TextStyle, Offset, Content, Icon, ExpandableText, Header, AppPage, IconButton, ButtonSize, ButtonStyle, ButtonState, NavigateEvent, TextInput, InputEditedEvent, ListItem, AvatarContent, AvatarIconStyle};
use crate::airlist::NewListScreen;

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
pub struct LandingScreen(Stack, Page, #[skip]String);

impl OnEvent for LandingScreen {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        true
    }
}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for LandingScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool { false }

    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(NewListScreen::new(ctx))),
            1 => {
                let text = self.2.clone();
                Ok(Box::new(NewListScreen::edit(ctx, &text)))
            }
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

        LandingScreen(Stack::default(), Page::new(Some(header), content, None), String::new())
    }

    //a constructor to receive the text from NewListScreen during navigation.
    pub fn with_list(ctx: &mut Context, mut text: String) -> Self {
        let mut screen = Self::new(ctx);
        screen.2 = text;
        println!("with_list's captured String from NewListScreen: {}", &screen.2);
        if screen.2.is_empty() {
            return screen
        }
        let items = screen.1.content().items();
        /*
        TODO: this if block removes stock text on LandingScreen. It is not elegant.
         */
        if items.len() >= 2 {
            let first_is_text = items[0].as_any().is::<Text>();
            let second_is_expandable = items[1].as_any().is::<ExpandableText>();
            if first_is_text && second_is_expandable {
                items.remove(0);
                items.remove(0);
            }
        }
        let test_list_item = ListItem::new(
            ctx,
            false,
            &format!("{}", screen.2.chars().take(10).collect::<String>()),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(AvatarContent::Icon("edit", AvatarIconStyle::Primary)),
            None,
            true,
            move |ctx: &mut Context| {
                println!("Clicked edit");
                ctx.trigger_event(NavigateEvent(1));
            }
        );
        // let expandable_text = ExpandableText::new(
        //     ctx,
        //     screen.2.as_str(),
        //     TextStyle::Primary,
        //     ctx.theme.fonts.size.md,
        //     Align::Center,
        //     Some(1)
        // );
        screen.1.content().items().push(Box::new(test_list_item));
        screen
    }
}