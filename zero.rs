#[lang="sized"]
trait Sized {}

#[lang="sync"]
trait Sync {}

#[cold]
#[lang="panic"]
pub fn panic(_ : &(&'static str, &'static str, uint)) -> ! {
    unsafe { abort() }
}

extern "rust-intrinsic" {
    pub fn abort() -> !;
}
