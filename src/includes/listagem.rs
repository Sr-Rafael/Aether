// aether/src/includes/listagem.rs

// === Inclui as dependencias do Módulo ===
use walkdir::WalkDir; // O que caminha nas pastas
use std::path::PathBuf; // Para o tipo de dado do caminho dos arquivos
use std::error::Error; // Para sinalizar o erro

// === Função pública de listagem ===
pub fn list_files(caminho: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
	// Cria a variável de lista, vazia, para armazenar os "achados"
	let mut arquivos_achados: Vec<PathBuf> = Vec::new();

	// === Bloco para realizar a varredura do caminho recebido ===
	for resultado in WalkDir::new(caminho) {
		// Analisar o que tem em entrada
		match resultado {
			// Caso esteja tudo "certo"
			Ok(entrada) => {
				// Verifica se é arquivo ou pasta
				if entrada.file_type().is_file() {
					arquivos_achados.push(entrada.into_path());
				}
			},
			// Caso dê erro
			Err(erro) => {
				eprintln!("[ERRO]: {}", erro);
			}
		}
	}
	// === Retorna a lista ===
	Ok(arquivos_achados)
}
