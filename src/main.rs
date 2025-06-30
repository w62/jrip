use iced::{widget::text, Element};


#[derive(Debug, Default)]
struct AppState {
}

#[derive(Debug)]
enum Message {
    Exit
}

fn update (state: &mut AppState, message:Message){
}

fn view(state: &AppState) -> Element<Message>{
    text("Hello from app").into()

}

fn main() -> iced::Result{
    iced::application("Jrip", update, view).run()
}
