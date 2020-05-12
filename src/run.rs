mod linux;

pub(crate) fn run() {
    if cfg!(target_os = "linux") {
        linux::run()
    } else {
        todo!()
    }
}
