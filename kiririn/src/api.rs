extern crate std;

use wasmer::{FunctionEnvMut, Memory, Store, WasmPtr};

/// Fire indexer
pub extern "C" fn kr_index() {
  //
}

pub extern "C" fn kr_http_req(uri: *const i8) -> () {
  //
}

pub fn kr_shell_cmd(
  env: FunctionEnvMut<(Store, Memory)>,
  command: WasmPtr<u8>,
  args: WasmPtr<u8>,
) {
  let (st, mem) = env.data();
  let mv = mem.view(&st);
  let res = std::process::Command::new(command.read_utf8_string_with_nul(&mv).unwrap())
    .args(&[args.read_utf8_string_with_nul(&mv).unwrap()])
    .output()
    .unwrap()
    .stdout;
  std::println!(
    "{}",
    std::string::String::from(std::str::from_utf8(res.as_slice()).unwrap())
  );
}
