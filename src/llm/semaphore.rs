use std::sync::OnceLock;

use tokio::sync::Semaphore;

static SEMAHORE: OnceLock<Semaphore> = OnceLock::new();

pub fn get_semaphore() -> &'static Semaphore {
    SEMAHORE.get_or_init(|| Semaphore::new(3))
}
