#![no_std]
extern crate alloc;
extern crate std;

extern crate wasmer;
extern crate wasmer_wasi;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::cell::RefCell;
use std::{error::Error, fs::read_dir, println};

use kirino_proto as proto;
use proto::Message;
use wasmer::{
  Extern, Function, FunctionEnv, FunctionType, HostFunction, Imports, Instance, Module,
  Pages, Store, TypedFunction, WasmPtr,
};
use wasmer_wasi::WasiState;

use crate::api;

type PbRetFn = TypedFunction<(), WasmPtr<u8>>;

/// A plugin
pub struct Plugin {
  info: proto::kiririn::Plugin,
  st: Store,
  ins: Instance,
}
impl Plugin {
  pub fn name(&self) -> &str { &self.info.name }
}

/// Plugin loader
///
/// Facilitates loading and unloading of plugins
pub struct Loader<'a> {
  plugin_dir: &'a str,
  plugins: Arc<RefCell<Vec<Plugin>>>,
}
impl<'a> Loader<'a> {
  /// Create a [Loader] for **`plugin_dir`**
  pub fn new(plugin_dir: &'a str) -> Self {
    Self {
      plugin_dir,
      plugins: Arc::new(RefCell::new(Vec::new())),
    }
  }
  /// Load all plugins into the runtime
  pub fn load_all(&mut self) -> Result<(), Box<dyn Error>> {
    // Iterate over plugins in plugin_dir
    //
    // Create an instance of each one
    for pl in read_dir(self.plugin_dir)? {
      // Create a store
      let mut st = Store::default();

      // Read module
      let mdl = Module::new(&st, std::fs::read(pl?.path())?)?;

      // Create a WASI context
      let wasi = WasiState::new("init").finalize(&mut st)?;
      // Import WASM module into WASI context
      let mut imp = wasi.import_object(&mut st, &mdl)?;
      // Add our imports
      imports(&mut st, &mut imp);

      // Initialize an instance of the module
      let ins = Instance::new(&mut st, &mdl, &imp)?;

      // Get plugin init info
      let info = {
        let init: PbRetFn = ins.exports.get_function("_krr_init")?.typed(&st)?;
        let mem = ins.exports.get_memory("memory")?;
        mem.grow(&mut st, Pages(200))?;
        let mv = mem.view(&st);
        let pb = init.call(&mut st)?.read_utf8_string_with_nul(&mv)?;
        proto::kiririn::Plugin::decode::<&[u8]>(pb.as_bytes()).unwrap()
      };

      // // Serialize for later
      // mdl.serialize_to_file(alloc::format!("{}/{}.pl", self.plugin_dir,
      // info.name))?;

      (*self.plugins.borrow_mut()).push(Plugin { info, st, ins });
    }
    Ok(())
  }
  /// Take ownership of all plugins from runtime
  pub fn all(self) -> Vec<Plugin> { self.plugins.take() }
  /// Run each plugin
  pub fn run_all(&self) -> Result<(), Box<dyn Error>> {
    let pls = &mut self.plugins.borrow_mut();
    for pl in pls.iter_mut() {
      runner(pl)?;
    }
    Ok(())
  }

  /// Interact with only plugins of [proto::plugin::Type] **`ty`**
  pub fn only<F>(&self, ty: proto::kiririn::plugin::Type, mut f: F)
  where
    F: FnMut(Vec<&Plugin>),
  {
    f((*self.plugins.borrow())
      .iter()
      .filter(|x| x.info.r#type() == ty)
      .collect())
  }
}

fn runner(pl: &mut Plugin) -> Result<(), Box<dyn Error>> {
  use proto::kiririn::plugin::Type::*;

  // Borrow exports
  let exp = &pl.ins.exports;
  // Get mem and create a view into it
  let mem = exp.get_memory("memory")?.view(&pl.st);

  match pl.info.r#type() {
    Service => {
      let run: PbRetFn = exp.get_typed_function(&pl.st, "_krr_run")?;
      run.call(&mut pl.st)?;
    }
    Metadata => {}
    Fetcher => {}
    Playback => {}
  }

  Ok(())
}

fn imports(st: &mut Store, i: &mut Imports) {
  i.define(
    "env",
    "kr_index",
    Function::new_typed(st, || {
      println!("kr_index");
    }),
  );
/*
  i.define(
    "env",
    "kr_shell_cmd",
    Function::new_typed_with_env(st, env, api::kr_shell_cmd),
  );
*/
}
