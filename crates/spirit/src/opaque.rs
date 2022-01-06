extern "C" {
    pub type Opaque;
}

unsafe impl Send for Opaque {}
unsafe impl Sync for Opaque {}
