use std::future::Future;

/// Windows stack size as the build flags are not necessarily
/// provided at time properly to resize the stack size
#[cfg(windows)]
const WINDOWS_STACK_SIZE: usize = 4 * 1024 * 1024;

/// If in Windows, it will run the underlying function with a given
/// stack size
pub fn run_with_stack_size<F: FnOnce() -> T + Send + 'static, T: Send + 'static>(function: F) -> T {
    #[cfg(unix)]
    {
        function()
    }
    #[cfg(windows)]
    {
        // Windows requires to set the stack because the package compiler puts too much on the
        // stack for the default size.  A quick internet search has shown the new thread with
        // a custom stack size is the easiest course of action.
        let child_thread = std::thread::Builder::new()
            .name("windows-support".to_string())
            .stack_size(WINDOWS_STACK_SIZE)
            .spawn(|| function())
            .expect("Expected to spawn release thread");
        child_thread
            .join()
            .expect("Expected to join release thread")
    }
}

/// If in Windows, it will run the underlying function with a given
/// stack size.  This is the async version
pub async fn run_async_with_stack_size<
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Send + Sync,
>(
    async_function: F,
) -> T {
    #[cfg(unix)]
    {
        async_function().await
    }
    #[cfg(windows)]
    {
        // Windows requires to set the stack because the package compiler puts too much on the
        // stack for the default size.  A quick internet search has shown the new thread with
        // a custom stack size is the easiest course of action.
        let child_thread = std::thread::Builder::new()
            .name("windows-support".to_string())
            .stack_size(WINDOWS_STACK_SIZE)
            .spawn(async_function)
            .expect("Expected to spawn release thread");
        child_thread
            .join()
            .expect("Expected to join release thread")
            .await
    }
}
