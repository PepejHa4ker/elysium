use core::ptr;

pub mod create_move;
pub mod draw_model_execute;
pub mod frame_stage_notify;

pub struct Hook {
    /// An arbitary address.
    pub address: *const *const (),

    /// Original value that existed at `address`.
    pub original: *const (),

    /// New value to replace the original with.
    pub replacement: *const (),

    /// Remember if the address is protected.
    pub protected: bool,
}

impl Hook {
    pub fn new(address: *const (), replacement: *const (), protected: bool) -> Self {
        Self {
            address: address as *const *const (),
            original: ptr::null(),
            replacement,
            protected,
        }
    }

    /// Check if the address has already been replaced.
    pub fn is_replaced(&self) -> bool {
        // SAFETY: Hope `self.address` is valid.
        unsafe { *self.address == self.replacement }
    }

    /// Apply the hook.
    pub fn apply_protected(&mut self) {
        unsafe {
            // We need a mutable address.
            let address = self.address as *mut *const ();

            if !self.is_replaced() {
                let protection = if self.protected {
                    // Remove protection and store it.
                    providence_util::unprotect(address as *const ())
                } else {
                    0
                };

                let original = address.replace(self.replacement);
                println!("hooking address={address:?} original={original:?}");

                if self.protected {
                    // Reapply protection.
                    providence_util::protect(address as *const (), protection);
                }

                // Store the original value.
                self.original = original;
            }
        }
    }

    /// Restore the original.
    pub fn restore(&mut self) {
        unsafe {
            // We need a mutable address.
            let address = self.address as *mut *const ();

            if self.is_replaced() {
                let protection = if self.protected {
                    // Remove protection and store it.
                    providence_util::unprotect(address as *const ())
                } else {
                    0
                };

                address.write(self.original);

                if self.protected {
                    // Reapply protection.
                    providence_util::protect(address as *const (), protection);
                }
            }
        }
    }
}

impl Drop for Hook {
    fn drop(&mut self) {
        self.restore();
    }
}
