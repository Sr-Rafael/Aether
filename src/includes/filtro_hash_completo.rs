// aether/src/includes/filtro_hash_completo.rs

// === Inclusão das dependencias ===
use std::collections::HashMap; // Para agrupar os arquivos com o mesmo hash
use std::path::PathBuf; // Para lidar com os caminhos dos arquivos

// === Inclusão de arquivos ===
use crate::includes::hash_completo; // O arquivo com a função que calcula o hash

// === Declaração da função para agrupar
pub fn agrupar_por_hash_completo (grupos_hash_curto: HashMap<String, Vec<PathBuf>>) -> HashMap<String, Vec<PathBuf>> {
	// Cria a variável que armazenará os arquivos agrupados por hash
	let mut grupos_hash_final: HashMap<String, Vec<PathBuf>> = HashMap::new();

	// For para passar todos os arquivos do passo anteriro para o processamento completo do hash
	for (_hash_curto, lista) in grupos_hash_curto {

		// "if" para verificar se a lista de arquivo possui mais de um arquivo
		if lista.len() < 2 {
			continue;
		}

		// "for" para poder passar o caminho para o calculo de hash
		for caminho in lista {
			// Chama o processamento de hash completo
			match hash_completo::calcular(&caminho) {
				Ok(hash_final) => {
					// Se der tudo certo, guarda o resultado, a chave hash completa e o caminho
					grupos_hash_final
						.entry(hash_final)
						.or_insert(Vec::new())
						.push(caminho);
				},
				Err(erro) => {
					// Deu alguma falha, talvez ao abrir o arquivo
					eprintln!("[ERRO]: Falha ao ler o arquivo {:?}. Erro: {}", caminho, erro);
				}
			}
		}
	}
	// Retorna o agrupamento de arquivos por hash
	grupos_hash_final
}
