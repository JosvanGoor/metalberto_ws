use std::io::Write;

#[derive(Clone)]
pub struct ProgressBar {
    title: String,
    total: usize,
    current: usize,
}

impl ProgressBar {

    pub fn new<T: Into<String>>(title: T, total: usize) -> Self {
        Self { title: title.into(), total: total, current: 0}
    }

    pub fn update(&mut self) {
        self.current += 1;
    }

    pub fn output(&self) {
        print!("{}: {} / {}                   \r", self.title, self.current, self.total);
        std::io::stdout().flush().ok().expect("failed to flush stdout");
    }

}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        println!();
    }
}