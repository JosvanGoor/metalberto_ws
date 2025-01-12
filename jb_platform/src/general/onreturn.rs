
#[derive(Default)]
pub struct OnReturn<F: FnOnce()> {
    function: Option<F>,
}

impl<F: FnOnce()> OnReturn<F> {

    pub fn new(call: F) -> Self {
        Self { function: Some(call) }
    }

    pub fn update(&mut self, function: F) {
        self.function = Some(function);
    }

    pub fn cancel(&mut self) {
        self.function = None;
    }
}

impl<F: FnOnce()> Drop for OnReturn<F> {
    fn drop(&mut self) {
        if self.function.is_some() {
            self.function.take().unwrap()();
        }
    }
}