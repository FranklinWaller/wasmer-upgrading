pub fn http_fetch<URL: ToString>(url: URL) -> String {
    let url = url.to_string();
    let result_length = unsafe { super::raw::http_fetch(url.as_ptr(), url.len() as u32) };
    let mut result_data_ptr = vec![0; result_length as usize];

    unsafe {
        super::raw::call_result_write(result_data_ptr.as_mut_ptr(), result_length);
    }

    String::from_utf8_lossy(&result_data_ptr).to_string()
}
