extern "C" {
    #[link_name = "llvm.frameaddress"]
    pub fn frame_address(depth: i32) -> *const i8;
}
