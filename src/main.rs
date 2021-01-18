use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox};
use std::rc::Rc;
use std::cell::RefCell;

struct State {
    label: String,
}

fn main() {
    // Initialize the UI and State
    let ui = UI::init().expect("Couldn't initialize UI library");
    let state = Rc::new(RefCell::new(State { label: "".into() }));

    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Hello, IUI!", 200, 100, WindowType::NoMenubar);
    
    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);
    
    // Create a new label.
    let label = Label::new(&ui, "");

    // Create a button and its callback
    let mut button = Button::new(&ui, "Click me!");
    button.on_clicked(&ui, {
        let state = state.clone();
        move |_| {
            state.borrow_mut().label = "Hello, world".to_owned();
        }
    });

    // Adding controls to the box, and box to window
    vbox.append(&ui, label.clone(), LayoutStrategy::Stretchy);
    vbox.append(&ui, button.clone(), LayoutStrategy::Compact);
    
    win.set_child(&ui, vbox);

    // Show the window
    win.show(&ui);

    // Run the application
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut ui_label = label.clone();
        move || {
            let state = state.borrow();

            // Update all the labels
            ui_label.set_text(&ui, &format!("{}", state.label));
        }
    });
    event_loop.run(&ui);
}
