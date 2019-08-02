pub struct Store<S, A> {
    state: S,
    reducer: fn(&mut S, A),
}

impl<S, A> Store<S, A> {
    pub fn new(state: S, reducer: fn(&mut S, A)) -> Store<S, A> {
        Store {
            state,
            reducer,
        }
    }

    pub fn dispatch(&mut self, action: A) {
        (self.reducer)(&mut self.state, action);
    }
}
