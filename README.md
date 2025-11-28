// Read.me V.1.0

# Aether: Organização absoluta
> _"A desordem não é uma opção. Aether é o árbitro final do seu conteúdo."_

## Visão Geral

O **Aether** é um utilitário de sistema construído na linguagem **Rust**, nascido da paixão de um entusiasta por programação com o auxílio da inteligência artificial (IA).

### Missão e Objetivos

Nossa missão é clara: impor a **Organização Absoluta** sobre o seu armazenamento, resolvendo o problema da redundância digital.

O Aether foi desenhado para identificar e gerenciar arquivos duplicados com a máxima velocidade, utilizando o **mínimo de recurso computacional** possível. Acreditamos que a precisão (a certificação binária) deve ser alcançada de forma eficiente e sem sobrecarregar seu hardware.

## Passos de desenvolvimento
### MVP - Beta 0.1.0
	[X] Listagem de arquivos
	[X] Comparação de tamanhos
	[X] Comparação de hash (pegana parte 4kb)
	[X] Comparação de hash (Completo)
	[ ] Comparação bit-a-bit
	[ ] Reporte de arquivos duplicados
### MVP - Beta 1.0.0
	[ ] Mover um dos arquivos duplicados para outra pasta (_duplicatas)
	[ ] Arquivo de conficuração
		- Configurar o tamanho do chunk;
		- Configurar a ordem de processamento;
	[ ] Interface
		[ ] CLI (Terminal)
		[ ] GUI {Priorizar essa}
		- Receber a pasta de monitoramento de forma dinamica.
> _Versão 1.0 concluída e pronta para distribuição

### Beta 1.1.0
	[ ] Vizualização previa dos arquivos duplicados
	[ ] Processamento manual dos arquivos
		- Renomear
		- Excluir

### Beta 1.2.0
	[ ] Implementar o mult-threding
		- Implementar concorrencia e paralelismo
		- Implementar resiliencia a falha (By. Elixir)

### Alpha 1.0.0
	[ ] Otimização no processo de verificação
		- Garantir e tratar erros
		- Validar a resiliencia do programa
