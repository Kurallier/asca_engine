use asca_lib::ASCAEngine;

pub fn main() {
    let mut app = ASCAEngine::init();

    app.start().run();
}
