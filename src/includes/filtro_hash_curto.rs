// aether/src/includes/filtro_hash_curto.rs

// === Dependências ===
use std::collections::HashMap;
use std::path::PathBuf;

// === Importação de arquivos ===
use crate::includes::hash_curto;

// === Cria a função para organizar os hashers ===
pub fn agrupar_por_hash(grupos_tamanho: HashMap<u64, Vec<PathBuf>>) -> HashMap<String, Vec<PathBuf>> {
	// Cria a variável que vai armazenar os grupos
	let mut grupos_hash: HashMap<String, Vec<PathBuf>> = HashMap::new();

	// Passa por cada grupo
	for (_tamanho, lista_arquivos) in grupos_tamanho {
		// Verificação para poder pular grupo com um arquivo
		if lista_arquivos.len() < 2 {
			continue;
		}

		// Se passou da verificação calcula o hasher
		for caminho in lista_arquivos {
			// Chama a função para calcular o hash
			match hash_curto::calcular(&caminho) {
				Ok(hash) => {
					// Em caso de sucesso guarada os dados
					grupos_hash
					.entry(hash)
					.or_insert(Vec::new())
					.push(caminho.to_path_buf());
					},
					Err(erro) =>{
					// Se der algum erro, retorna a falha
					eprintln!("[ERRO]: Falha ao calcular o hash de {:?}. Erro: {}", caminho, erro);
				}
			}
		}
	}
	// Retorna o novo grupo de conteúdo
	grupos_hash
}
