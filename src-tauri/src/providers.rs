//! Versioned state-file provider boundary. Remote providers intentionally reject operations.
use std::{fs, path::PathBuf};
use serde_json::Value;
pub trait SyncProvider {
    fn read(&self, key: &str) -> Result<Option<Value>, String>;
    fn write(&self, key: &str, value: &Value) -> Result<(), String>;
}
pub struct LocalProvider { pub directory: PathBuf }
impl LocalProvider {
    fn path(&self, key:&str)->Result<PathBuf,String>{
        if key.is_empty() || !key.bytes().all(|b|b.is_ascii_alphanumeric() || b==b'-' || b==b'_') {return Err("Invalid state key".into())}
        Ok(self.directory.join(format!("{key}.json")))
    }
}
impl SyncProvider for LocalProvider {
    fn read(&self,key:&str)->Result<Option<Value>,String>{let p=self.path(key)?;match fs::read(p){Ok(b)=>serde_json::from_slice(&b).map(Some).map_err(|e|e.to_string()),Err(e) if e.kind()==std::io::ErrorKind::NotFound=>Ok(None),Err(e)=>Err(e.to_string())}}
    fn write(&self,key:&str,value:&Value)->Result<(),String>{let p=self.path(key)?;fs::create_dir_all(&self.directory).map_err(|e|e.to_string())?;let bytes=serde_json::to_vec_pretty(value).map_err(|e|e.to_string())?;let temp=self.directory.join(format!("{}.tmp",uuid::Uuid::new_v4()));fs::write(&temp,bytes).map_err(|e|e.to_string())?;fs::rename(&temp,&p).map_err(|e|format!("State write failed (existing destination may require conflict resolution): {e}"))}
}
pub struct GitHubProvider;
pub struct FutureSelfHostedProvider;
macro_rules! planned {($t:ty)=>{impl SyncProvider for $t {fn read(&self,_:&str)->Result<Option<Value>,String>{Err("Provider not implemented".into())}fn write(&self,_:&str,_:&Value)->Result<(),String>{Err("Provider not implemented".into())}}};}
planned!(GitHubProvider); planned!(FutureSelfHostedProvider);
pub trait StorageProvider { fn description(&self)-> &'static str; }
pub struct LocalStorage; pub struct NASStorage; pub struct FutureCloudStorage;
impl StorageProvider for LocalStorage {fn description(&self)-> &'static str {"local"}}
impl StorageProvider for NASStorage {fn description(&self)-> &'static str {"planned"}}
impl StorageProvider for FutureCloudStorage {fn description(&self)-> &'static str {"planned"}}
#[cfg(test)] mod tests {use super::*;#[test]fn local_roundtrip_and_key_guard(){let p=std::env::temp_dir().join(format!("hub-provider-{}",uuid::Uuid::new_v4()));let local=LocalProvider{directory:p.clone()};local.write("task_1",&serde_json::json!({"v":1})).unwrap();assert_eq!(local.read("task_1").unwrap().unwrap()["v"],1);assert!(local.read("../outside").is_err());fs::remove_dir_all(p).unwrap();}}
