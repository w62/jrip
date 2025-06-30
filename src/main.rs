use iced::{
    widget::{button, column, text}, 
    window, Element, Task,
};


#[derive(Debug, Default)]
struct AppState {
}

#[derive(Debug, Clone)]
enum Message {
    Exit
}

fn update (state: &mut AppState, message:Message) -> Task<Message>{
    match message {
        Message:: Exit => window::get_latest().and_then(window::close),
    }
}

fn view(state: &AppState) -> Element<Message>{
    column![
    text("Hello from app").size(24),
    button(text("Exit").size(24)).on_press(Message::Exit)
    ]
    .into()

}

fn main() -> iced::Result{
    iced::application("Jrip", update, view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}
