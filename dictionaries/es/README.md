# Wordlist es (filtro)

- **Arquivo canônico:** `dictionaries/es/words.txt` (85.649 formas únicas, ~0,8 MB)
- **Cópia servida ao frontend:** `public/dicts/es/words.txt` (idêntica; o frontend busca em runtime, fora do bundle JS)
- **Origem:** wordlist `spanish` (pacote `wspanish`, dados presentes em
  `/usr/share/dict/spanish` nos runners Debian/Ubuntu)
- **Processamento:** minúsculas + remoção de diacríticos via NFD + filtro `^[a-z]+$` + deduplicação.
  `Corazón → corazon`, `Ñ → n` (`niño → nino`). Nomes próprios mantidos de propósito.
- **Licença dos dados:** a lista é fato linguístico (palavras), sem descrições autorais;
  verbetes com descrição continuam no dicionário curado embutido no código.

Para regenerar: ver `dictionaries/README.md` (guia da comunidade).
