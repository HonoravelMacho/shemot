# Wordlist en (filtro)

- **Arquivo canônico:** `dictionaries/en/words.txt` (75.144 formas únicas, ~0,7 MB)
- **Cópia servida ao frontend:** `public/dicts/en/words.txt` (idêntica; o frontend busca em runtime, fora do bundle JS)
- **Origem:** wordlists `american-english` + `british-english` (pacote `wamerican`/`wbritish`,
  dados presentes em `/usr/share/dict/american-english` e `/usr/share/dict/british-english`
  nos runners Debian/Ubuntu; mescladas com deduplicação)
- **Processamento:** minúsculas + remoção de diacríticos via NFD + filtro `^[a-z]+$` + deduplicação.
  `Love → love`, `naïve → naive`. Entradas com apóstrofo (`AA's`) são descartadas de propósito.
- **Licença dos dados:** a lista é fato linguístico (palavras), sem descrições autorais;
  verbetes com descrição continuam no dicionário curado embutido no código.

Para regenerar: ver `dictionaries/README.md` (guia da comunidade).
