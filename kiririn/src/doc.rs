/// # Creating plugins
///
/// ```
/// pub extern "C" fn _krr_init() -> *const u8 {
///   Plugin {
///     r#type: plugin::Type::Service.into(),
///     name: "kr-test".to_string(),
///     desc: "aaa".to_string(),
///     url: "aaa".to_string(),
///   }
///   .encode_to_vec()
///   .as_ptr()
/// }
/// ```
pub mod creating_plugins {}
