use pelican_ui::{Component, Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS};
use pelican_ui::drawable::{Drawable, Component, Align, Color};
use pelican_ui::runtime::{Services, ServiceList};
use pelican_ui::layout::{Layout, SizeRequest, Area};
use pelican_ui::events::OnEvent;
use std::collections::BTreeMap;
use maverick_os::window::EventHandler;
use pelican_ui_std::{Interface, InputEditedEvent, Stack, Page, Text, TextInput, TextStyle, Offset, Content, Icon, ExpandableText, Header, AppPage, IconButton, ButtonSize, ButtonStyle, ButtonState, NavigateEvent};
use crate::{airlist, LandingScreen, LoggedInput};

#[derive(Debug, Component)]
pub struct InputLogger(Stack, TextInput);

impl InputLogger {
    fn new(ctx: &mut Context, input: TextInput) -> Self {
        InputLogger(Stack::default(), input)
    }
}

impl OnEvent for InputLogger {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn pelican_ui::events::Event) -> bool {
        if event.downcast_ref::<InputEditedEvent>().is_some() {
            println!("In NewListScreen's text field, User entered: {}", self.1.value());
        }

        if let Some(edited) = event.downcast_ref::<InputEditedEvent>() {
            ctx.trigger_event(LoggedInput(self.1.value().clone()));
        }

        true
    }

}


#[derive(Debug, Component)]
pub struct NewListScreen(Stack, Page, #[skip]String);

impl OnEvent for NewListScreen {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn pelican_ui::events::Event) -> bool {
        // if event.downcast_ref::<InputEditedEvent>().is_some() {
                //print statement fires when user enters in text to text_field.
        //     println!("text_field edited");
        // }
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
        let mut text_field = TextInput::new(
            ctx,
            None,
            None,
            "Enter list here.",
            None,
            TextInput::NO_ICON,
            true);

        let captured_text = &mut text_field.value().clone();

        let logger = InputLogger::new(ctx, text_field);

        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(logger)]
        );
        NewListScreen(Stack::default(), Page::new(Some(header), content, None), captured_text.to_string())
    }
}
