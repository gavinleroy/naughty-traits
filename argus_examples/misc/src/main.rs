
struct IsSync {
    inner: Box<Option<IsSync>>,
}

fn require_sync_item<T: Sync>(value: T) { }

fn main() {
    let is_sync = IsSync { inner: Box::new(None) };
    require_sync_item(is_sync);
}
