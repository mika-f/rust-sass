#![deny(clippy::all)]

use std::path::{Path, PathBuf};

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct SassOptions {
  pub file: String,
  pub source_map: bool,
  pub style: String,
  pub syntax: String,
}

#[napi(object)]
pub struct LegacySassOptions {
  pub charset: bool,
  pub data: String,
  pub file: String,
  pub include_paths: Vec<String>,
  pub indented_syntax: bool,
  pub omit_source_map_url: bool,
  pub out_file: String,
  pub output_style: String,
  pub source_map: bool,
  pub source_map_contents: bool,
  pub source_map_embed: bool,
}

#[napi(object)]
pub struct SassError {
  pub file: Option<String>,
  pub line: Option<u32>,
  pub column: Option<u32>,
  pub message: Option<String>,
  pub formatted: Option<String>,
}

#[napi(object)]
pub struct SassStats {
  pub included_files: Vec<String>,
}

#[napi(object)]
pub struct SassResult {
  pub css: String,
  pub loaded_urls: Vec<String>,
  pub source_map: String,
  pub stats: SassStats,
  pub map: String,
}

#[napi(object)]
pub struct CompileResult {
  pub success: Option<SassResult>,
  pub failure: Option<SassError>,
}

fn get_entires_of_node_modules(file: String, paths: Option<Vec<String>>) -> Vec<PathBuf> {
  let mut entries = Vec::<PathBuf>::new();

  if let Some(paths) = &paths {
    let mut paths = paths
      .iter()
      .map(|w| PathBuf::from(w))
      .collect::<Vec<PathBuf>>();

    entries.append(&mut paths);
  }

  let dirname = if let Some(paths) = &paths {
    std::path::Path::new(&paths[0])
  } else {
    std::path::Path::new(&file).parent().unwrap()
  };
  let node_modules = dirname.join("node_modules");

  if node_modules.exists() && node_modules.is_dir() {
    /*
    for entry in std::fs::read_dir(node_modules).unwrap() {
      if let Ok(entry) = entry {
        if entry.path().is_dir() {
          entries.push(entry.path().strip_prefix(dirname).unwrap().to_path_buf());
        }
      }
    }
    */
    entries.push(node_modules);
  }

  return entries;
}

fn to_grass_options(
  opts: Option<SassOptions>,
  legacy: Option<LegacySassOptions>,
) -> grass::Options<'static> {
  if let Some(opts) = opts {
    return grass::Options::default()
      .style(if opts.style == "compressed" {
        grass::OutputStyle::Compressed
      } else {
        grass::OutputStyle::Expanded
      })
      .load_paths(&get_entires_of_node_modules(opts.file, None))
      .input_syntax(if opts.syntax == "indented" {
        grass::InputSyntax::Sass
      } else if opts.syntax == "scss" {
        grass::InputSyntax::Scss
      } else {
        grass::InputSyntax::Css
      });
  }

  if let Some(opts) = legacy {
    return grass::Options::default()
      .style(if opts.output_style == "compressed" {
        grass::OutputStyle::Compressed
      } else {
        grass::OutputStyle::Expanded
      })
      .load_paths(&get_entires_of_node_modules(
        opts.file,
        Some(opts.include_paths),
      ))
      .allows_charset(opts.charset)
      .input_syntax(if opts.indented_syntax {
        grass::InputSyntax::Sass
      } else {
        grass::InputSyntax::Scss
      });
  }

  grass::Options::default()
}

fn to_sass_result(css: String) -> SassResult {
  SassResult {
    css,
    loaded_urls: Vec::new(),
    source_map: "".to_owned(),
    stats: SassStats {
      included_files: Vec::new(),
    },
    map: "".to_owned(),
  }
}

fn to_sass_error(err: Box<grass::Error>) -> SassError {
  match err.kind() {
    grass::ErrorKind::ParseError {
      message,
      loc,
      unicode: _,
    } => SassError {
      file: Some(loc.file.to_owned().name().to_string()),
      line: Some(loc.begin.line as u32),
      column: Some(loc.begin.column as u32),
      message: Some(message.to_string()),
      formatted: None,
    },
    grass::ErrorKind::IoError(_) => todo!(),
    grass::ErrorKind::FromUtf8Error(_) => todo!(),
    _ => todo!(),
  }
}

#[napi]
pub fn compile(source: String, options: Option<SassOptions>) -> CompileResult {
  let ret = grass::from_string(source, &to_grass_options(options, None));

  if ret.is_ok() {
    return CompileResult {
      success: Some(to_sass_result(ret.unwrap())),
      failure: None,
    };
  }

  return CompileResult {
    success: None,
    failure: Some(to_sass_error(ret.err().unwrap())),
  };
}

#[napi]
pub fn compile_legacy(source: String, options: Option<LegacySassOptions>) -> CompileResult {
  let ret = grass::from_string(source, &to_grass_options(None, options));

  if ret.is_ok() {
    return CompileResult {
      success: Some(to_sass_result(ret.unwrap())),
      failure: None,
    };
  }

  return CompileResult {
    success: None,
    failure: Some(to_sass_error(ret.err().unwrap())),
  };
}
