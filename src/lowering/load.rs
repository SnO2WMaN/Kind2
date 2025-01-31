use crate::book::name::Ident;
use crate::book::new_type::NewType;

use crate::driver::config::Config;
use crate::parser::new_type::read_newtype;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

// TODO: Remove this from the adjust layer. I think that we need to move it
// to the driver.
fn load_newtype(config: &Config, name: &Ident) -> Result<Box<NewType>, String> {
    let path = config.kind2_path.clone();
    let root = Path::new(&path).join(name.to_string().replace('.', "/"));
    let path = root.join("_.type");

    let newcode = match std::fs::read_to_string(&path) {
        Err(_) => {
            return Err(format!("File not found: '{}'.", path.display()));
        }
        Ok(code) => code,
    };
    let newtype = match read_newtype(&newcode) {
        Err(err) => {
            return Err(format!("\x1b[1m[{}]\x1b[0m\n{}", path.display(), err));
        }
        Ok(book) => book,
    };
    Ok(newtype)
}

pub fn load_newtype_cached(config: &Config, cache: &mut HashMap<Ident, Rc<NewType>>, name: &Ident) -> Result<Rc<NewType>, String> {
    if !cache.contains_key(name) {
        let newtype = Rc::new(*load_newtype(config, name)?);
        cache.insert(name.clone(), newtype);
    }
    return Ok(cache.get(name).unwrap().clone());
}
