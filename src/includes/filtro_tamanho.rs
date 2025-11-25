// aether/src/includes/filtro_tamanhos.rs

// === Depêndencias ===
use std::collections::HashMap; // Para armazenar os conjuntos de arquivos
use std::path::PathBuf; // Para o tipo de dado do caminho
use std::fs::{self, File}; // Para acessar o disco
use std::io::{Seek, SeekFrom};

// === Função para agrupamento ===
pub fn agrupar_por_tamanho(arquivos: Vec<PathBuf>) -> HashMap<u64, Vec<PathBuf>> {
	// Cria a variável que armazenará os agrupamentos
	let mut grupos: HashMap<u64, Vec<PathBuf>> = HashMap::new();

	// Loop par apassar todos os caminho para o agrupador
	for caminho in arquivos {
		// [PASSO A]: Tenta abrir o arquivo
		match File::open(&caminho) {

			// Abre o arquivo
			Ok(mut arquivo_aberto) => {
				
				// Se aberto executa o bloco entre as chaves
				match arquivo_aberto.seek(SeekFrom::End(0)) {
					Ok(tamanho_bytes) => { // Conseguiu abrir e não deu falha ao mover o cursor
						grupos
							.entry(tamanho_bytes)
							.or_insert(Vec::new())
							.push(caminho);
					},
					Err(erro_seek) => {
						// Abriu o arquivo mas falhou ao mover o arquivo
						eprintln!("[ERRO] Falha ao ler o tamanho do arquivo: {:?}. Erro: {}", caminho, erro_seek);
					}
				}
			},
			Err(erro_abertura) => {
				// Se falhar ao abrir o arquivo
				eprintln!("[ERRO]: Falha ao abrir o arquivo: {:?}. Erro: {}", caminho, erro_abertura);
			}
		}
	}

	// Retorna o mapa com os agrupamentos
	grupos
}
