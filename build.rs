use std::fs;
use std::io::Write;

fn main() {
    // Diretório que contém os arquivos Markdown
    let diretorio = "./posts";

    // Abre ou cria um arquivo para escrita
    let mut arquivo_saida =
        fs::File::create("src/markdown_posts.rs").expect("Falha ao criar o arquivo de saída");

    // Escreve o cabeçalho do arquivo de saída
    writeln!(arquivo_saida, "pub const FILES: &[(&str, &str)] = &[").unwrap();

    // Lista todos os arquivos no diretório
    let arquivos = fs::read_dir(diretorio).expect("Falha ao listar os arquivos no diretório");

    // Itera sobre cada arquivo no diretório
    for arquivo in arquivos {
        let arquivo = arquivo.expect("Falha ao ler o arquivo");
        let path = arquivo.path();

        // Verifica se o arquivo é um arquivo regular e termina com extensão .md
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            // Obtém o nome do arquivo
            let nome_arquivo = path.file_stem().unwrap().to_str().unwrap();

            // Escreve uma entrada na lista
            writeln!(
                arquivo_saida,
                "    (\"{}\", include_str!(\"../{}\")),",
                nome_arquivo,
                path.display()
            )
            .unwrap();
        }
    }

    // Escreve o rodapé do arquivo de saída
    writeln!(arquivo_saida, "];").unwrap();
}
