use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;

#[macro_use]
extern crate clap;

pub mod plugins;
pub mod plugin;
pub mod error;

use plugins::*;
use error::*;

fn get_var(key: &str) -> Result<Option<String>, Error> {
    use std::env::VarError::*;

    match std::env::var(key) {
        Ok(value) => Ok(Some(value)),
        Err(NotPresent) => Ok(None),
        Err(NotUnicode(value)) => Err(Error::EnvironmentVariableNotUnicode { key: key.to_string(), value: value} ),
    }
}

pub fn add_and_save(zr_home: &PathBuf, plugin: &str, file: Option<&str>) -> Result<(), Error> {
    let mut plugins = plugins_from(&zr_home);
    plugins.add(plugin, file)?;
    plugins.save()
}

pub fn plugins_from(zr_home: &PathBuf) -> Plugins {
    let mut plugins = Plugins::new(zr_home.clone());
    let zr_init = &zr_home.join("init.zsh");
    let plugin_home = &zr_home.join("plugins");

    if zr_init.exists() {
        let init_file = OpenOptions::new().read(true).open(&zr_init).unwrap();
        for filepath in BufReader::new(&init_file)
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| line.starts_with("source"))
            .map(|line| PathBuf::from(line.split_whitespace().last().unwrap()))
            .map(|filepath| filepath.strip_prefix(&plugin_home).ok().unwrap().to_owned() )
            .collect::<Vec<_>>() {
                let filename = filepath.to_str().to_owned().unwrap();
                let name = filename.split('/').collect::<Vec<_>>()[0..2].join("/");
                let file = filename.split('/').collect::<Vec<_>>()[2..].join("/");
                let _ = plugins.add(&name, Some(&file));
            }
    }

    plugins
}

pub fn load_plugins(zr_home: &PathBuf, parameters: Vec<String>) -> Result<(), Error> {
    let mut plugins: Plugins = Plugins::new(zr_home.clone());

    let mut params = parameters.iter().peekable();

    while params.peek().is_some() {
        let param = params.next().unwrap();
        if params.peek().is_some() && params.peek().unwrap().contains(".") {
            let _ = plugins.add(param, Some(params.next().unwrap()));
        } else {
            let _ = plugins.add(param, None);
        }
    }

    plugins.save()
}

pub fn run() -> Result<(), Error> {
    let zr_home = get_var("ZR_HOME")?;
    let home = get_var("HOME")?;
    let default_home = format!("{}/.zr", home.unwrap());
    let path = PathBuf::from(zr_home.unwrap_or(default_home));

    let mut zr = clap_app!(zr =>
        (version: crate_version!())
        (author: "Jonathan Dahan <hi@jonathan.is>")
        (about: "z:rat: - zsh plugin manager")
        (@subcommand reset => (about: "delete init file") )
        (@subcommand list => (about: "list plugins") )
        (@subcommand update => (about: "update plugins") )
        (@subcommand load => (about: "load plugins fresh")
            (@arg plugins: +required +multiple +takes_value "plugin/name [path/to/file.zsh] [[plugin/name [..]..]")
        )
        (@subcommand add =>
            (about: "add plugin to init file")
            (@arg plugin: +required "plugin/name")
            (@arg file: "optional/path/to/file.zsh")
        )
    );

    match zr.clone().get_matches().subcommand() {
        ("add", Some(m)) => add_and_save(&path, m.value_of("plugin").unwrap(), m.value_of("file")),
        ("load", Some(m)) => load_plugins(&path, m.values_of_lossy("plugins").unwrap()),
        ("list", _) => plugins_from(&path).list(),
        ("reset", _) => plugins_from(&path).reset(),
        ("update", _) => plugins_from(&path).update(),
        (_, _) => zr.print_help().map_err(Error::Clap),
    }
}