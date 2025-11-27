// aether/src/main.rs

// === Inclusão das dependencias externas ===
use std::error::Error;
use std::env; // Para a ler a entrada no terminal
use std::collections::HashMap;
use std::path::PathBuf;

// === Inclusão de arquivos ===
mod includes;

// === Inicio da main, função principal, que orquestrará tudo. ===
fn main() -> Result<(), Box<dyn Error>> {
  // === Coleta os argumentos da linha de comando ===
  let argumentos: Vec<String> = env::args().collect(); // Cria uma variável (vector) que armazena os caracteres do argumento passado no terminal

  // === Valida o argumento coletado ===
  if argumentos.len() < 2 {
    // Caso seja falso, imprime um erro
    eprintln!("[ERRO]: Por favor, forneça o diretório para varredura.\n");
    
    // Retorna um erro antes de fechar.
    return Err("Caminho do diretório não fornecido.".into());
  }

  // === Bloco para armazenar o caminho do usuário ===
  let caminho = &argumentos[1];
  println!("Caminho de destino detectado: {}\n", caminho);

  // === Notifica o início da varredura e passa o caminho para o "responsáveç" ===
  println!("--- Aether: Varredura de arquivos iniciada!");
  let arquivos_encontrados = includes::listagem::list_files(caminho)?;

  // __ TEMPORARIO, PARA O COMPILADOR NÃO RECLAMAR __
  println!("Total de arquivos encontrados: {}\n", arquivos_encontrados.len());

  // === Chama a função de agrupamento ===
  println!("--- Iniciando agrupamento por tamanho ---");
  let grupos_tamahos = includes::filtro_tamanho::agrupar_por_tamanho(arquivos_encontrados);

  // __ Exibe quantos agrupamentos foi realizado __
  println!("Agrupamento concluído!\n");
  
  // Exibe quantos grupos tem
  println!("Total de grupos com tamanhos diferente foi: {}", grupos_tamahos.len());

  let mut potencial_duplicatas_tamanho = 0;
  for (_tamanho, lista) in &grupos_tamahos {
    if lista.len() > 1 {
      potencial_duplicatas_tamanho += lista.len();
    }
  }
  println!("Arquivos que contém o mesmo tamanho: {}\n", potencial_duplicatas_tamanho);

  // Exibe quantos arquivos com hashers iniciais parecidos tem
  println!("--- Iniciando agrupamento por Hash curto 4Kb (SHA-512) ---");

  // Chama a função e passa o antigo agrupamento para a função de calculo de hash curto
  let grupos_hash_curto: HashMap<String, Vec<PathBuf>> = includes::filtro_hash_curto::agrupar_por_hash(grupos_tamahos);
  println!("Agrupamento concluído!\n");

  // Conta os arquivos duplicados, com hashers parecidos
  let mut potencial_duplicatas_hash = 0;
  for (_hash, list) in &grupos_hash_curto {
    if list.len() > 1 {
      potencial_duplicatas_hash += list.len();
    }
  }

  // Exibe a quantidade de arquivos com hashers parecidos
  println!("Arquivos que contém o mesmo hasher inicial: {}", potencial_duplicatas_hash);
  
  // Exibe os resultados dos arquivos com hasher completamente parecidos
  println!("\n--- Iniciando agrupamento por Hash (SHA-512) ---");

  // Chama a função e passa o antigo agrupamento para a função de calculo de hash curto
  let grupos_hash: HashMap<String, Vec<PathBuf>> = includes::filtro_hash_completo::agrupar_por_hash_completo(grupos_hash_curto);
  println!("Agrupamento concluído!\n");

  // Conta os arquivos duplicados, com hashers parecidos
  let mut duplicatas_hash = 0;
  for (_hash, list) in &grupos_hash {
    if list.len() > 1 {
      duplicatas_hash += list.len();
    }
  }

  // Exibe a quantidade de arquivos com hashers parecidos
  println!("Arquivos que contém o mesmo hasher: {}", duplicatas_hash);
  // __ __

  // === Retorna sucesso em caso de der certo ===
  Ok(())
}
