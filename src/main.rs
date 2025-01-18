use relm4::RelmApp;
use rusty_minesweeper::app::App;
use rusty_minesweeper::game::models::game::GameDifficulty;

fn main() {
    let app = RelmApp::new("not4rts.minesweeper");
    let difficulty = GameDifficulty::default();

    relm4::set_global_css(include_str!("css/style.css"));
    app.run::<App>(difficulty);
}
