mod airlist;

use crate::airlist::airlist::{AtomicCounterForListID, List, ListContainer, ListEditorScreen};
use pelican_ui::drawable::{Align, Component, Drawable};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::runtime::{ServiceList, Services};
use pelican_ui::{
    Application, Component, Context, MaverickOS, PelicanEngine, Plugin, Plugins, maverick_start,
    start,
};
use pelican_ui_std::{
    AppPage, AvatarContent, AvatarIconStyle, Content, ExpandableText, Header, IconButton,
    Interface, ListItem, NavigateEvent, Offset, Page, Stack, Text, TextStyle,
};

// Define the main application struct. This is our entry point type.
pub struct MyApp;

impl Services for MyApp {
    fn services() -> ServiceList {
        ServiceList::default()
    }
}

impl Plugins for MyApp {
    fn plugins(_ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
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
    fn on_event(&mut self, _ctx: &mut Context, _event: &mut dyn Event) -> bool {
        true
    }
}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for LandingScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool {
        false
    }

    fn navigate(
        self: Box<Self>,
        ctx: &mut Context,
        index: usize,
    ) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(ListEditorScreen::new(ctx))),
            1 => {
                /*
                 * This finds the correct List based off of it's ID, then injects that content
                 * into ListEditorScreen String field for population into TextInput field.
                 */
                let list_id = ctx.state().get_named::<usize>("editing_list_id").copied();

                if let Some(list_id) = list_id {
                    let list_content = ctx
                        .state()
                        .get_named::<ListContainer>("list_container")
                        .and_then(|list_container| {
                            list_container
                                .get_ref_veclist()
                                .iter()
                                .find(|list| list.get_id() == list_id)
                                .map(|list| list.get_content().clone())
                        });

                    if let Some(content) = list_content {
                        return Ok(Box::new(ListEditorScreen::edit(ctx, &content, list_id)));
                    }
                }
                //fallback option in event where editing_list_id isn't found. Delete this?
                Ok(Box::new(ListEditorScreen::new(ctx)))
            }
            _ => Err(self),
        }
    }
}

impl LandingScreen {
    pub fn new(ctx: &mut Context) -> Self {
        //create new list_container if none exists.
        if ctx
            .state()
            .get_named::<ListContainer>("list_container")
            .is_none()
        {
            let list_container = ListContainer::default();
            ctx.state()
                .set_named(String::from("list_container"), list_container);
        }
        if ctx
            .state()
            .get_named::<AtomicCounterForListID>("atomic_counter")
            .is_none()
        {
            let atomic_counter = AtomicCounterForListID::new();
            ctx.state()
                .set_named(String::from("atomic_counter"), atomic_counter);
        }
        let new_list_icon = IconButton::navigation(ctx, "add", |ctx: &mut Context| {
            println!("new_list_icon clicked.");
            ctx.trigger_event(NavigateEvent(0));
        });

        // Create a header for the page
        let header = Header::home(
            // The majority of UI components will require the app context.
            ctx,
            // The text on this header will say "AirList"
            "AirList",
            Some(new_list_icon),
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
            Align::Center,
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
            None,
        );

        // Combine icon, heading, and subtext into page content
        let content = Content::new(
            ctx,
            // Vertically center items
            Offset::Center,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text), Box::new(subtext)],
        );

        LandingScreen(Stack::default(), Page::new(Some(header), content, None))
    }

    //a constructor to receive the text from NewListScreen during navigation.
    pub fn with_list(ctx: &mut Context) -> Self {
        let mut screen = Self::new(ctx);
        let items = screen.1.content().items();
        if items.is_empty() {
            return Self::new(ctx);
        }
        // this if block removes stock text on LandingScreen.
        if items.len() >= 2 {
            let first_is_text = items[0].as_any().is::<Text>();
            let second_is_expandable = items[1].as_any().is::<ExpandableText>();
            if first_is_text && second_is_expandable {
                items.remove(0);
                items.remove(0);
            }
        }

        /* Get a vec of Lists, then build a vec of list_items, then iterate through and push to Content.*/
        let veclist: Vec<List> = ctx
            .state()
            .get_named::<ListContainer>("list_container")
            .unwrap()
            .get_ref_veclist()
            .clone();

        let vec_listitem = Self::vec_listitem_builder(ctx, &veclist);
        for list_item in vec_listitem {
            screen.1.content().items().push(Box::new(list_item));
        }
        screen
    }
    pub fn vec_listitem_builder(ctx: &mut Context, vec_list: &[List]) -> Vec<ListItem> {
        let vec_listitem: Vec<ListItem> = vec_list
            .iter()
            .map(|list| {
                let list_id = list.get_id();
                ListItem::new(
                    ctx,
                    false,
                    list.get_content(),
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
                        println!("Clicked edit for list id: {}", list_id);
                        ctx.state()
                            .set_named(String::from("editing_list_id"), list_id);
                        ctx.trigger_event(NavigateEvent(1));
                    },
                )
            })
            .collect();

        vec_listitem
    }
}
