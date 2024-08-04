use std::{
    cell::{RefCell, RefMut},
    collections::{HashMap, HashSet},
    io::{BufRead, Read, Write},
    rc::Rc,
};

use super::consts::*;

pub struct BundlerContext {
    cache: Rc<Cache>,
}

fn get_replace_headers() -> Vec<(&'static str, &'static str)> {
    return vec![("inner/", PROJECT_DIR), ("atcoder/", PROJECT_DIR)];
}

impl BundlerContext {
    pub fn new() -> Self {
        Self {
            cache: Rc::new(Cache::default()),
        }
    }

    pub fn clear_target(&self, exec: &str) -> std::io::Result<()> {
        let path = format!("{}/{}.cc", BUNDLE_OUT_DIR, exec);
        match std::fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn bundle_target(&mut self, exec: &str, source: &str) -> std::io::Result<()> {
        let mut ctx_inner = ContextInner {
            cache: self.cache.clone(),
            has_found: HashSet::new(),
            all_deps: Vec::new(),
        };
        ctx_inner.get_file_quote_deps_recusive(source)?;
        let deps = ctx_inner.all_deps;
        let mut out = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(format!("{}/{}.cc", BUNDLE_OUT_DIR, exec))?;
        for dep in deps.iter() {
            let res = self
                .cache
                .clone()
                .replace_header_with_file_content(dep, &deps)?;
            out.write_all(res.as_bytes())?;
        }
        out.flush()
    }

    pub fn clear_all(&self) -> std::io::Result<()> {
        let sources = ICPC_MAIN_SOURCE.split(";").collect::<Vec<_>>();
        let execs = ICPC_EXECUTABLE_LIST.split(";").collect::<Vec<_>>();
        if sources.len() != execs.len() {
            panic!("`ICPC_EXECUTABLE_LIST` and `ICPC_MAIN_SOURCE` has not equal len, use `bundle help gen` to get help");
        }
        let deal_len = sources.len();
        for i in 0..deal_len {
            self.clear_target(execs[i])?;
        }
        Ok(())
    }

    pub fn bundle_all(&mut self) -> std::io::Result<()> {
        let sources = ICPC_MAIN_SOURCE.split(";").collect::<Vec<_>>();
        let execs = ICPC_EXECUTABLE_LIST.split(";").collect::<Vec<_>>();
        if sources.len() != execs.len() {
            panic!("`ICPC_EXECUTABLE_LIST` and `ICPC_MAIN_SOURCE` has not equal len, use `bundle help gen` to get help");
        }
        let deal_len = sources.len();
        for i in 0..deal_len {
            self.bundle_target(execs[i], sources[i])?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Cache {
    file_except_some_def_map: RefCell<HashMap<String, Rc<String>>>,
    file_quote_dep_headers: RefCell<HashMap<String, Rc<Vec<String>>>>,
    replace_header_with_file_content: RefCell<HashMap<String, Rc<String>>>,
}

impl Cache {
    fn get_or_insert<H, T>(
        mut inner: RefMut<HashMap<String, Rc<T>>>,
        h: H,
        path: &str,
    ) -> std::io::Result<Rc<T>>
    where
        H: Fn(&str) -> std::io::Result<T>,
    {
        match inner.get(path) {
            Some(res) => Ok(res.clone()),
            None => {
                let res = Rc::new(h(path)?);
                inner.insert(path.to_string(), res.clone());
                Ok(res)
            }
        }
    }

    fn get_file_except_some_def(self: Rc<Self>, path: &str) -> std::io::Result<Rc<String>> {
        Self::get_or_insert(
            self.file_except_some_def_map.borrow_mut(),
            |path| {
                // TODO impl #define xxx
                let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                Ok(buf)
            },
            path,
        )
    }

    fn get_file_quote_dep_headers(self: Rc<Self>, path: &str) -> std::io::Result<Rc<Vec<String>>> {
        Self::get_or_insert(
            self.file_quote_dep_headers.borrow_mut(),
            |path: &str| {
                let re = regex::Regex::new("(#include \"([^\"]*)\")").unwrap();
                let mut deps = vec![];
                let file = std::fs::OpenOptions::new().read(true).open(path)?;
                let reader = std::io::BufReader::new(file);
                for line in reader.lines() {
                    if let Some(line_deps) = re.captures(line?.as_str()) {
                        deps.push(line_deps[2].to_string());
                    }
                }
                Ok(deps)
            },
            path,
        )
    }

    fn replace_header_with_file_content(
        self: Rc<Self>,
        path: &str,
        deps: &Vec<String>,
    ) -> std::io::Result<Rc<String>> {
        let file_str = self.clone().get_file_except_some_def(path)?;
        Self::get_or_insert(
            self.replace_header_with_file_content.borrow_mut(),
            |_path: &str| {
                let mut res = String::new();
                let re = regex::Regex::new("(#include \"([^\"]*)\")").unwrap();
                for line in file_str.lines() {
                    if let Some(line_deps) = re.captures(line) {
                        if !deps.iter().any(|e| e.ends_with(&line_deps[2])) {
                            res.push_str(line);
                            res.push_str("\n");
                        }
                    } else {
                        res.push_str(line);
                        res.push_str("\n");
                    }
                }
                Ok(res)
            },
            path,
        )
    }
}

struct ContextInner {
    cache: Rc<Cache>,
    has_found: HashSet<String>,
    all_deps: Vec<String>,
}

impl ContextInner {
    fn get_file_quote_deps_recusive(&mut self, path: &str) -> std::io::Result<()> {
        for x in self.cache.clone().get_file_quote_dep_headers(path)?.iter() {
            for (match_prefix, replace) in get_replace_headers() {
                if x.starts_with(match_prefix) {
                    let sub_path = format!("{}/{}", replace, x);
                    if self.has_found.contains(&sub_path) {
                        break;
                    }
                    self.has_found.insert(sub_path.clone());
                    self.get_file_quote_deps_recusive(sub_path.as_str())?;
                    break;
                }
            }
        }
        self.all_deps.push(path.to_string());
        Ok(())
    }
}
