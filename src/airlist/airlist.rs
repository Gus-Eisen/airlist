use crate::LandingScreen;
use chrono::prelude::*;
use pelican_ui::drawable::{Component, Drawable};
use pelican_ui::events::{Event, OnEvent};
use pelican_ui::layout::{Area, Layout, SizeRequest};
use pelican_ui::{Component, Context};
use pelican_ui_std::{
    AppPage, ButtonSize, ButtonState, ButtonStyle, Content, Header, IconButton, InputEditedEvent,
    NavigateEvent, Offset, Page, Stack, TextInput,
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Component)]
pub struct ListEditorScreen(Stack, Page, #[skip] String, #[skip] Option<usize>);

impl OnEvent for ListEditorScreen {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if event.downcast_ref::<InputEditedEvent>().is_some()
            && let Some(input) = self.1.content().find::<TextInput>()
        {
            self.2 = input.value().clone();
            println!("NewListScreen captured text: {}", self.2);
        }
        true
    }
}

// Implement the AppPage trait for navigation and UI behavior
impl AppPage for ListEditorScreen {
    // This screen does not have a navigation bar
    fn has_nav(&self) -> bool {
        false
    }

    fn navigate(
        mut self: Box<Self>,
        ctx: &mut Context,
        index: usize,
    ) -> Result<Box<dyn AppPage>, Box<dyn AppPage>> {
        match index {
            0 => {
                let string_from_text_input = self
                    .1
                    .content()
                    .find::<TextInput>()
                    .unwrap()
                    .value()
                    .clone();
                //fires if there are no prior lists and empty LES TextInput field.
                if self.3.is_none()
                    && string_from_text_input.is_empty()
                    && ctx
                        .state()
                        .get_named::<ListContainer>("list_container")
                        .map_or(true, |container| container.get_ref_veclist().is_empty())
                {
                    return Ok(Box::new(LandingScreen::new(ctx)));
                }
                //fires if there are no prior lists and LES TextInput field is populated.
                if self.3.is_none() && !string_from_text_input.is_empty() {
                    let list = List::new(
                        ctx.state()
                            .get_named::<AtomicCounterForListID>("atomic_counter")
                            .unwrap()
                            .generate_id(),
                        string_from_text_input.clone(),
                    );
                    let list_container: &mut ListContainer =
                        ctx.state().get_named_mut("list_container").unwrap();
                    list_container.set(list);
                    return Ok(Box::new(LandingScreen::with_list(ctx)));
                }
                //fires if User edits list but deletes all values from TextInput.
                if self.3.is_some() && string_from_text_input.is_empty() {
                    if let Some(list_id) = self.3 {
                        if string_from_text_input.is_empty() {
                            let list_container: &mut ListContainer =
                                ctx.state().get_named_mut("list_container").unwrap();
                            if let Some(position) = list_container
                                .get_refmut_veclist()
                                .iter()
                                .position(|list| list.get_id() == list_id)
                            {
                                let _ = list_container.get_refmut_veclist().remove(position);
                            }
                        }
                        return Ok(Box::new(LandingScreen::new(ctx)));
                    }
                    //we want to fire LS::new() if User doesn't enter text and has no prior Lists.
                    // if string_from_text_input.is_empty()
                    //     && ctx
                    //         .state()
                    //         .get_named::<ListContainer>("list_container")
                    //         .map_or(true, |container| container.get_ref_veclist().is_empty())
                    // {
                    //     return Ok(Box::new(LandingScreen::new(ctx)));
                    // }
                }
                // let list = List::new(
                //     ctx.state()
                //         .get_named::<AtomicCounterForListID>("atomic_counter")
                //         .unwrap()
                //         .generate_id(),
                //     string_from_text_input,
                // );
                // let list_container: &mut ListContainer =
                //     ctx.state().get_named_mut("list_container").unwrap();
                // list_container.set(list);
                // println!(
                //     "ListEditorScreen navigate to LandingScreen; list_container: {:?}",
                //     &list_container
                // );
                Ok(Box::new(LandingScreen::with_list(ctx)))
            }
            _ => Err(self),
        }
    }
}

impl ListEditorScreen {
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
            Some(return_to_landingscreen_icon),
        );
        let text_field = TextInput::new(
            ctx,
            None,
            None,
            "Enter list here.",
            None,
            TextInput::NO_ICON,
            true,
        );

        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text_field)],
        );
        ListEditorScreen(
            Stack::default(),
            Page::new(Some(header), content, None),
            String::new(),
            None,
        )
    }

    //variant of LES to edit list.
    pub fn edit(ctx: &mut Context, user_text: &str, list_id: usize) -> Self {
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
            Some(return_to_landingscreen_icon),
        );
        let text_field = TextInput::new(
            ctx,
            Some(user_text),
            None,
            "Edit list here.",
            None,
            TextInput::NO_ICON,
            true,
        );
        let content = Content::new(
            ctx,
            Offset::Start,
            // All items must be boxed as Box<dyn Drawable>
            vec![Box::new(text_field)],
        );
        ListEditorScreen(
            Stack::default(),
            Page::new(Some(header), content, None),
            user_text.to_owned(),
            Some(list_id),
        )
    }

    pub fn get_list(&mut self, ctx: &mut Context) -> List {
        /*TODO: unwrap will panic if string_from_text_input returns None. Does not handle
        scenario where users deletes TextInput.
         */
        let string_from_text_input = self.1.content().find::<TextInput>().unwrap().value();
        List::new(
            ctx.state()
                .get_named::<AtomicCounterForListID>("atomic_counter")
                .unwrap()
                .generate_id(),
            string_from_text_input.to_owned(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct List {
    id: usize,
    content: String,
}

impl List {
    pub fn new(id: usize, content: String) -> Self {
        Self { id, content }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Default)]
pub struct ListContainer {
    vec_of_lists: Vec<List>,
}

impl ListContainer {
    pub fn new(list: List) -> Self {
        Self {
            vec_of_lists: vec![list],
        }
    }

    pub fn set(&mut self, list: List) {
        self.vec_of_lists.push(list);
    }

    // pub fn remove(&mut self, )
    //
    // pub fn get_ref(&self) -> &Self {
    //     self
    // }

    pub fn get_ref_veclist(&self) -> &Vec<List> {
        &self.vec_of_lists
    }

    pub fn get_refmut_veclist(&mut self) -> &mut Vec<List> {
        &mut self.vec_of_lists
    }
}

#[derive(Debug)]
pub struct AtomicCounterForListID {
    id: AtomicUsize,
}

impl AtomicCounterForListID {
    pub fn new() -> Self {
        Self {
            id: AtomicUsize::new(0),
        }
    }
    pub fn generate_id(&self) -> usize {
        self.id.fetch_add(1, Ordering::Relaxed)
    }
}
