use std::time::Duration;

use gm::ToF32;

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

pub async fn sleep(duration: impl ToF32) {
    #[cfg(not_wasm)]
    async_std::task::sleep(Duration::from_secs_f32(duration.to_f32())).await;
    #[cfg(wasm)]
    gloo_timers::future::TimeoutFuture::new((duration.to_f32() * 1000.0) as _).await;
}
