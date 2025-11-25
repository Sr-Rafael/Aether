// aether/src/main.rs

// === Inclusão das dependencias externas ===
use std::error::Error;
use std::env; // Para a ler a entrada no terminal

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
  println!("--- Aether: Varredura de arquivos iniciada!\n");
  let arquivos_encontrados = includes::listagem::list_files(caminho)?;

  // __ TEMPORARIO, PARA O COMPILADOR NÃO RECLAMAR __
  println!("Total de arquivos encontrados: {}\n", arquivos_encontrados.len());
  // __ __

  // === Chama a função de agrupamento ===
  println!("\n--- Iniciando agrupamento por tamanho ---");
  let grupos_tamahos = includes::filtro_tamanho::agrupar_por_tamanho(arquivos_encontrados);

  // __ Exibe quantos agrupamentos foi realizado __
  println!("\nAgrupamento concluído!\n");
  
  // Exibe quantos grupos tem
  println!("Total de tamanhos diferente foi: {}", grupos_tamahos.len());

  let mut potencial_duplicatas_tamanho = 0;
  for (_tamanho, lista) in &grupos_tamahos {
    if lista.len() > 1 {
      potencial_duplicatas_tamanho += lista.len();
    }
  }
  println!("Arquivos que contém o mesmo tamanho: {}", potencial_duplicatas_tamanho);

  // === Retorna sucesso em caso de der certo ===
  Ok(())
}
