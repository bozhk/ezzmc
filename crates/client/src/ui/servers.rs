//! Список серверов `servers.dat` (VANILLA: ServerList — несжатый NBT, `servers: [{name, ip}]`).
use ezzmc_protocol::nbt::{self, Tag};
use ezzmc_protocol::wire::Reader;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub struct Server {
    pub name: String,
    pub ip: String,
}

pub fn load(path: &Path) -> Vec<Server> {
    let Ok(data) = std::fs::read(path) else {
        return Vec::new();
    };
    let Ok(Some((_, Tag::Compound(root)))) = nbt::read_named(&mut Reader::new(&data)) else {
        return Vec::new();
    };
    let Some((_, Tag::List(list))) = root.into_iter().find(|(k, _)| k == "servers") else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|t| {
            Some(Server {
                name: t.get("name")?.as_str()?.to_owned(),
                ip: t.get("ip")?.as_str()?.to_owned(),
            })
        })
        .collect()
}

pub fn save(path: &Path, servers: &[Server]) {
    let list = servers
        .iter()
        .map(|s| {
            Tag::Compound(vec![
                ("name".into(), Tag::String(s.name.clone())),
                ("ip".into(), Tag::String(s.ip.clone())),
            ])
        })
        .collect();
    let root = Tag::Compound(vec![("servers".into(), Tag::List(list))]);
    let mut out = Vec::new();
    if nbt::write_named(&mut out, "", &root).is_ok() {
        if let Err(e) = std::fs::write(path, out) {
            tracing::error!("servers.dat: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let p = std::env::temp_dir().join("ezzmc_servers_test.dat");
        let v = vec![Server {
            name: "A".into(),
            ip: "1.2.3.4:25565".into(),
        }];
        save(&p, &v);
        assert_eq!(load(&p), v);
        let _ = std::fs::remove_file(&p);
    }
}
