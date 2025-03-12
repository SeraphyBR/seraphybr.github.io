use std::fs;
use std::io::Write;
use std::process::Command;

fn main() {
  let npm_result = Command::new("npm").args(["run", "build"]).output();

  if let Err(err) = npm_result {
    eprintln!("{}", err);
  }

  create_markdown_posts_with_files_to_include();
}

fn create_markdown_posts_with_files_to_include() {
  let posts_folder = "./posts";

  let mut out_file =
    fs::File::create("src/markdown_posts.rs").expect("Falha ao criar o arquivo de saída");

  writeln!(out_file, "// Generated By build.rs").unwrap();
  writeln!(out_file, "pub const FILES: &[(&str, &str)] = &[").unwrap();

  let files = fs::read_dir(posts_folder).expect("Falha ao listar os arquivos no diretório");

  for file in files {
    let file = file.expect("Falha ao ler o arquivo");
    let path = file.path();

    if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
      let file_name = path.file_stem().unwrap().to_str().unwrap();

      writeln!(
        out_file,
        "    (\"{}\", include_str!(\"../{}\")),",
        file_name,
        path.display()
      )
      .unwrap();
    }
  }

  writeln!(out_file, "];").unwrap();
}
