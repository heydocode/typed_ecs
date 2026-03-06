pub struct NoopGuard;

impl Drop for NoopGuard {
    #[inline(always)]
    fn drop(&mut self) {}
}
