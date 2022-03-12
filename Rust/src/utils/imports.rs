extern "C" {
    #[link_name = "alert"]
    pub fn import_alert(msg: *const u8);

    #[link_name = "llvm.trap"]
    pub fn import_trap();
}
