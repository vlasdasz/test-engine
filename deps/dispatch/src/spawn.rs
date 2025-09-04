#[cfg(wasm)]
pub fn spawn<F>(future: F)
where F: Future<Output = ()> + 'static {
    wasm_bindgen_futures::spawn_local(future);
}

#[cfg(not_wasm)]
pub fn spawn<F, O>(future: F)
where
    F: Future<Output = O> + Send + 'static,
    O: Send + 'static, {
    std::thread::spawn(|| async_std::task::block_on(future));
}

#[cfg(wasm)]
pub fn block_on<F>(future: F)
where F: Future<Output = ()> + 'static {
    wasm_bindgen_futures::spawn_local(future);
}

#[cfg(not_wasm)]
pub fn block_on<F, O>(future: F)
where F: Future<Output = O> {
    async_std::task::block_on(future);
}

#[cfg(not_wasm)]
pub fn unasync<F, Out>(future: F) -> Out
where F: Future<Output = Out> {
    async_std::task::block_on(future)
}
