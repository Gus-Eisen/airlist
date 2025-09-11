use pelican_ui::{Component, Context};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui_std::{InputEditedEvent, Stack, Page, TextInput, Offset, Content, Header, AppPage, IconButton, ButtonSize, ButtonStyle, ButtonState, NavigateEvent};
use crate::{LandingScreen,};
use chrono::prelude::*;

#[derive(Debug, Component)]
pub struct ListEditor(Stack, Page, #[skip]String);

impl OnEvent for ListEditor {
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
impl AppPage for ListEditor {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool { false }

    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => Ok(Box::new(LandingScreen::with_list(ctx, self.2.clone()))),
            // 1 => Ok(Box::new(LandingScreen::))
            _ => Err(self),
        }
    }
}

impl ListEditor {
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
        ListEditor(Stack::default(), Page::new(Some(header), content, None), String::new())
    }

    pub fn edit(ctx: &mut Context, user_text: &str) -> Self {
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
        let header = Header::home(
            // The majority of UI components will require the app context.
            ctx,
            // The text on this header will say "AirList"
            "AirList",
            Some(return_to_landingscreen_icon)
        );
        let text_field = TextInput::new(
            ctx,
            Some(user_text),
            None,
            "Edit list here.",
            None,
            TextInput::NO_ICON,
            true);
        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text_field)]
        );
        ListEditor(Stack::default(), Page::new(Some(header), content, None), user_text.to_owned())
    }
}

pub struct List {
    date_time: DateTime<Utc>,
    content: String
}

impl List {
    pub fn new(content: String) -> Self {
        Self {
            date_time: Utc::now(),
            content
        }
    }
}

pub struct ListContainer {
    vec_of_lists: Vec<List>
}

impl ListContainer {
    pub fn new(list: List) -> Self {
        Self {
            vec_of_lists: vec![list]
        }
    }
    
}
