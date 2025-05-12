use std::collections::HashMap;
use std::{fs, io};
use std::path::PathBuf;
use bytebuffer::ByteBuffer;
use log::{error, info};
use mvengine::ui::res::runtime::RuntimeResources;
use mvutils::bytebuffer::ByteBufferExtras;
use mvutils::save::Savable;

pub struct LocalModManager {
    resources: HashMap<String, RuntimeResources<'static>>
}

impl LocalModManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn load_all(&mut self, directory: &PathBuf) -> io::Result<()>{
        info!("Loading client resources...");
        let paths = fs::read_dir(directory)?;
        for path in paths {
            let path = path?;
            let path = path.path();
            if path.is_file() {
                let path2 = path.clone();
                let modid = path.file_prefix().unwrap();
                if let Some(modid) = modid.to_str() {
                    let raw = fs::read(path2)?;
                    let mut buffer = ByteBuffer::from_vec_le(raw);
                    match RuntimeResources::load(&mut buffer) {
                        Ok(resources) => {
                            self.resources.insert(modid.to_string(), resources);
                            info!("Loaded client resources for mod: {modid}");
                        },
                        Err(e) => {
                            error!("Error when constructing RuntimeResources for mod {modid}: {e}");
                        } 
                    }
                }
            }
        }
        info!("Loading done");
        Ok(())
    }
    
    pub fn is_res_loaded(&self, mod_id: &str) -> bool {
        self.resources.contains_key(mod_id)
    }
    
    pub fn get(&self, modid: &str) -> Option<&RuntimeResources<'static>> {
        self.resources.get(modid)
    }
}