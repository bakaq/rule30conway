use rule30conway::backends::{
    Backend,
    SdlBackend,
};

fn main() {
    let mut backend = SdlBackend::new(500, 300, 2);
    backend.main_loop();
}
