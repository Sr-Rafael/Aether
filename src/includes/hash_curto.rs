// aether/src/includes/hash_curto.rs

// === Dependências ===
use std::fs::File;
use std::io::{self, Read};
use sha2::{Sha512, Digest};

// === Cria a função para calcular o hash ===
pub fn calcular(caminho: &std::path::Path) -> Result<String, io::Error> {
	// Abre o arquivo
	let mut arquivo = File::open(caminho)?;

	// Cria o hasher, responsável por ver a identidade do arquivo
	let mut hasher =  Sha512::new();

	// Cria a variável que servirar de buffer para leitura do hash
	let mut buffer = [0;4096];

	// Ler uma parte do arquivo, definido na variável de buffer
	let bytes_lidos = arquivo.read(&mut buffer)?;

	// Passa para o hasher apenas o tamanho do buffer, ao invés do arquivo todo
	hasher.update(&buffer[..bytes_lidos]);

	// Finaliza o hasher e armazena o resultado
	let resultado = hasher.finalize();

	// Transforma o resultado em valor e transforma em texto hexadecimal
	Ok(hex::encode(resultado))
}
