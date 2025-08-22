use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align, Color};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::OnEvent;
use std::collections::BTreeMap;

use pelican_ui_std::{Interface, Stack, Page, Text, TextStyle, Offset, Content, Icon, ExpandableText, Header, AppPage, IconButton, ButtonSize, ButtonStyle, ButtonState};
use crate::LandingScreen;

#[derive(Debug, Component)]
pub struct NewListScreen(Stack, Page);

impl OnEvent for NewListScreen {}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for NewListScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool { false }

    // Handle page navigation. Always returns Err(self) because this page cannot navigate.
    fn navigate(self: Box<Self>, _ctx: &mut Context, _index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        Err(self)
    }
}

impl NewListScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let return_to_landingscreen_icon = IconButton::new(
            ctx,
            "backspace",
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
            Some(return_to_landingscreen_icon)
        );
        //TODO: why is text not aligning left?
        let text = ExpandableText::new(ctx, "Type a name for your list or leave blank for today's date.", TextStyle::White, 25.0, Align::Left, Some(2));
        
        // Combine heading into page content
        let content = Content::new(
            ctx,
            // Vertically center items
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text)]
        );

        // new Page containing our header, content, and no bumper.
        NewListScreen(Stack::default(), Page::new(Some(header), content, None))
    }
}
