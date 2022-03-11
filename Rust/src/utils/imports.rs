extern "C" {
    #[link_name = "alert"]
    pub fn import_alert(msg: *mut u8);
}
