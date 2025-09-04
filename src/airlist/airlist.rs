use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align, Color};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::{Event, OnEvent};
use std::collections::BTreeMap;
use maverick_os::window::EventHandler;
use pelican_ui_std::{Interface, InputEditedEvent, Stack, Page, Text, TextInput, TextStyle, Offset, Content, Icon, ExpandableText, Header, AppPage, IconButton, ButtonSize, ButtonStyle, ButtonState, NavigateEvent};
use crate::{airlist, LandingScreen,};

#[derive(Debug, Component)]
pub struct NewListScreen(Stack, Page, #[skip]String);

impl OnEvent for NewListScreen {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if event.downcast_ref::<InputEditedEvent>().is_some() {
            if let Some(input) = self.1.content().find::<TextInput>() {
                self.2 = input.value().clone();
                println!("NewListScreen captured text: {}", self.2);
            }
        }
        true
    }
}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for NewListScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool { false }

    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(LandingScreen::new(ctx))),
            _ => Err(self),
        }
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
            Box::new(|ctx: &mut Context| {
                println!("return_to_landingscreen_icon clicked.");
                ctx.trigger_event(NavigateEvent(0));
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
        let text_field = TextInput::new(
            ctx,
            None,
            None,
            "Enter list here.",
            None,
            TextInput::NO_ICON,
            true);

        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text_field)]
        );
        NewListScreen(Stack::default(), Page::new(Some(header), content, None), String::new())
    }
}
