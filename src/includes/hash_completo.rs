// aether/src/includes/hash_completo.rs

// === Inclusão de depêndencias ===
use std::fs::File;
use std::io::{self, Read};
use sha2::{Sha512, Digest};

// === Declaração da função para cálcular o hash ===
pub fn calcular (caminho: &std::path::Path) -> Result<String, io::Error> {

	// Abre o arquivo
	let mut arquivo = File::open(caminho)?;

	// Cria o hasher
	let mut hasher = Sha512::new();

	// Cria o buffer para leitura do arquivo
	let mut buffer = [0; 4096];

	// Lógica para ler todo o arquivo e calcular o hasher
	loop {
		// Tenta armazenar o arquivo dentro do buffer
		let count = arquivo.read(&mut buffer)?;

		// "if" para verificar se leu todo o arquivo
		if count == 0 {
			break;
		}

		// Envia para o hasher para poder realizar o calculo
		hasher.update(&buffer[..count]);
	}

	// Finaliza o hasher e armazena o resultado em uma variável
	let resultado = hasher.finalize();

	// Converte o resulado do hasher em hexadecimal e retorna a converção
	Ok(hex::encode(resultado))
}
