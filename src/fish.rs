//! Local catalogue adapter. Documents stay at their original sync paths;
//! the editor index stores only navigation metadata, never a second copy.
use crate::{shared_store::SharedStore, store::{Entry, Store}};
use std::{collections::HashMap, path::PathBuf};

fn root() -> PathBuf { Store::root_dir().parent().unwrap_or(std::path::Path::new(".")).to_path_buf() }
fn catalogue() -> Vec<(String, PathBuf)> {
    let root = root();
    let Ok(bytes) = std::fs::read(root.join(".sync/catalog.json")) else { return vec![] };
    let Ok(items) = serde_json::from_slice::<Vec<(String, String)>>(&bytes) else { return vec![] };
    items.into_iter().filter_map(|(name, file)| {
        let path = PathBuf::from(file).canonicalize().ok()?;
        let base = root.canonicalize().ok()?;
        (path.starts_with(base) && path.extension().is_some_and(|e| e == "md") && path.is_file()).then_some((name, path))
    }).collect()
}
pub fn important(e: &Entry) -> bool {
    e.meta.path.as_ref().is_some_and(|p| catalogue().iter().any(|(_, f)| f.to_string_lossy() == p.as_str()))
}
fn category(name: &str) -> &'static str {
    if name == "MEMORY.md" { "核心记忆" }
    else if name.contains("B站") { "学习与知识/B站创作" }
    else if name.contains("学习") { "学习与知识" }
    else { "人格与约定" }
}
pub fn refresh(store: &mut SharedStore) -> anyhow::Result<()> {
    let marker = Store::root_dir().join("fish-folders.json");
    let mut folders: HashMap<String, String> = std::fs::read(&marker).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
    let entries = store.list();
    for name in ["人格与约定", "核心记忆", "经历与日记", "学习与知识", "学习与知识/B站创作", "创作与项目", "归档"] {
        // Keep renamed/moved/deleted categories as the user arranged them.
        if folders.contains_key(name) { continue; }
        let (parent, title) = name.rsplit_once('/').map_or((None, name), |(p, t)| (folders.get(p).map(String::as_str), t));
        let id = store.create_folder(title, parent)?;
        folders.insert(name.to_string(), id);
    }
    // Seed a stable section order once; subsequent user moves own the order.
    for (position, name) in ["人格与约定", "核心记忆", "经历与日记", "学习与知识", "创作与项目", "归档"].iter().enumerate() {
        if let Some(id) = folders.get(*name) && store.list().iter().any(|e| &e.id == id && e.meta.parent.is_none() && e.meta.root_order.is_none()) {
            store.update_meta(id, |m| m.root_order = Some(position as u64 + 1))?;
        }
    }
    let tmp = marker.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_vec(&folders)?)?;
    std::fs::rename(tmp, marker)?;
    for (name, path) in catalogue() {
        if entries.iter().any(|e| e.meta.path.as_deref() == path.to_str()) { continue; }
        // Quiet may rename a Markdown file when its title changes. Preserve
        // the editor identity and the user's placement for that same sync slot.
        if let Some(old) = entries.iter().find(|e| e.meta.path.as_ref().is_some_and(|p| { let p = std::path::Path::new(p); !p.exists() && p.parent() == path.parent() })) {
            store.update_meta(&old.id, |m| m.path = Some(path.to_string_lossy().into_owned()))?;
            continue;
        }
        let id = store.open_local(&path)?;
        if let Some(parent) = folders.get(category(&name)) && store.list().iter().any(|e| &e.id == parent) { store.set_parent(&id, Some(parent))?; }
    }
    Ok(())
}
