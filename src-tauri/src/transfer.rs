use serde::{Deserialize,Serialize};
use std::{fs,io::{Read,Write},path::Path};

const MAX_BYTES:u64=1024*1024;
#[derive(Debug,Serialize,Deserialize,Clone)]
#[serde(deny_unknown_fields)]
pub struct Packet {
 pub format:String, pub packet_id:String, pub source_task_id:String, pub repository:String,
 pub branch:String, pub head:String, pub created_at:String, pub agent:String, pub title:String,
 pub goal:String, pub completed:Vec<String>, pub remaining:Vec<String>, pub next_action:String, pub warnings:String,
}
pub fn repository_id(remote:&str)->Result<String,String>{
 let raw=remote.trim();
 let path=if let Some(p)=raw.strip_prefix("https://github.com/"){p}
 else if let Some(p)=raw.strip_prefix("git@github.com:"){p}
 else if let Some(p)=raw.strip_prefix("ssh://git@github.com/"){p}
 else{return Err("仅支持不含凭据的 GitHub HTTPS 或 SSH origin 地址".into())};
 let path=path.trim_end_matches('/').strip_suffix(".git").unwrap_or(path.trim_end_matches('/'));
 let parts:Vec<_>=path.split('/').collect();
 if parts.len()!=2 || parts.iter().any(|p|(p.is_empty()||*p=="."||*p=="..")||!p.bytes().all(|c|c.is_ascii_alphanumeric()||b"-_.".contains(&c))) {return Err("GitHub origin 地址格式不正确".into())}
 Ok(format!("github.com/{}/{}",parts[0].to_ascii_lowercase(),parts[1].to_ascii_lowercase()))
}
fn valid_id(s:&str)->bool { !s.is_empty() && s.len()<=128 && s.bytes().all(|c|c.is_ascii_alphanumeric()||b"-_.".contains(&c)) }
pub fn validate(p:&Packet)->Result<(),String>{
 if p.format!="lkmt-handoff-v1" || !valid_id(&p.packet_id) || !valid_id(&p.source_task_id){return Err("交接文件版本或标识无效".into())}
 if p.head.len()!=40 || !p.head.bytes().all(|c|c.is_ascii_hexdigit()) || p.branch.trim().is_empty() || p.branch.len()>512 {return Err("交接文件中的提交或分支无效".into())}
 if !p.repository.starts_with("github.com/") || repository_id(&format!("https://{}",p.repository))?!=p.repository {return Err("交接文件中的仓库标识无效".into())}
 if p.title.trim().is_empty() || p.title.len()>1024 || p.agent.len()>1024 || p.goal.len()>65536 || p.next_action.len()>65536 || p.warnings.len()>65536 || p.completed.len()>1000 || p.remaining.len()>1000 || p.completed.iter().chain(&p.remaining).any(|v|v.len()>65536) {return Err("交接文件内容超出限制".into())}
 Ok(())
}
pub fn write_new(path:&Path,packet:&Packet)->Result<(),String>{validate(packet)?;if path.extension().and_then(|v|v.to_str())!=Some("json"){return Err("请选择以 .json 结尾的新文件".into())}let bytes=serde_json::to_vec_pretty(packet).map_err(|e|e.to_string())?;if bytes.len() as u64>MAX_BYTES{return Err("交接文件超过大小限制".into())}let mut f=fs::OpenOptions::new().write(true).create_new(true).open(path).map_err(|e|format!("无法创建新交接文件（不会覆盖已有文件）：{e}"))?;f.write_all(&bytes).and_then(|_|f.sync_all()).map_err(|e|format!("写入交接文件失败，文件可能不完整：{e}"))}
pub fn read(path:&Path)->Result<Packet,String>{let mut f=fs::File::open(path).map_err(|e|format!("无法读取交接文件：{e}"))?;let mut b=Vec::new();std::io::Read::by_ref(&mut f).take(MAX_BYTES+1).read_to_end(&mut b).map_err(|e|e.to_string())?;if b.len() as u64>MAX_BYTES{return Err("交接文件超过大小限制".into())}let p:Packet=serde_json::from_slice(&b).map_err(|e|format!("交接文件无效：{e}"))?;validate(&p)?;Ok(p)}
#[cfg(test)] mod tests { use super::*;
 #[test]fn normalize_and_reject_secrets(){assert_eq!(repository_id("git@github.com:Owner/Repo.git").unwrap(),"github.com/owner/repo");assert_eq!(repository_id("https://github.com/owner/repo").unwrap(),"github.com/owner/repo");assert!(repository_id("https://name:secret@github.com/owner/repo").is_err());assert!(repository_id("https://github.com/owner/../repo").is_err());}
 #[test]fn packet_refuses_overwrite_and_bad_data(){let p=Packet{format:"lkmt-handoff-v1".into(),packet_id:"abc".into(),source_task_id:"task-1".into(),repository:"github.com/a/b".into(),branch:"feature/a".into(),head:"a".repeat(40),created_at:"2026-01-01T00:00:00Z".into(),agent:"codex".into(),title:"demo".into(),goal:"".into(),completed:vec![],remaining:vec![],next_action:"".into(),warnings:"".into()};let dir=std::env::temp_dir().join(format!("lkmt-transfer-{}",uuid::Uuid::new_v4()));fs::create_dir_all(&dir).unwrap();let file=dir.join("handoff.json");write_new(&file,&p).unwrap();assert!(write_new(&file,&p).is_err());assert_eq!(read(&file).unwrap().head,p.head);fs::write(&file,b"{invalid").unwrap();assert!(read(&file).is_err());fs::remove_dir_all(dir).unwrap();}
}
