extern "C" {
    pub fn http_fetch(action: *const u8, action_length: u32) -> u32;
    pub fn call_result_write(result: *const u8, result_length: u32) -> u32;
}