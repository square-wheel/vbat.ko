#[lang="sized"]
trait Sized {}

#[lang="sync"]
trait Sync {}

#[lang="copy"]
trait Copy {}

#[cold]
#[lang="panic"]
pub fn panic(_ : &(&'static str, &'static str, usize)) -> ! {
    unsafe { abort() }
}

extern "rust-intrinsic" {
    pub fn abort() -> !;
}
