/// ZST, used for data that relies on the `Drop` implementation
/// to trigger events. A great example of such would be tracing
/// subscribers instances (for backends having one), and for spans.
/// 
/// More precisely, this NoopGuard is a drop-in replacement when
/// a feature is disabled, and we have to turn tracing profiling
/// stuff into completely dead & optimized away code.
pub struct NoopGuard;

impl Drop for NoopGuard {
    #[inline(always)]
    fn drop(&mut self) {}
}
