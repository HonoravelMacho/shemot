# Wordlist el — Grego Moderno (filtro)

- **Arquivo canônico:** `dictionaries/el/words.txt` (329.375 formas únicas, ~5,9 MB)
- **Cópia servida ao frontend:** `public/dicts/el/words.txt` (idêntica; o frontend busca em runtime, fora do bundle JS)
- **Origem:** `hunspell-el` (dicionário Modern Greek do hunspell, pacote Debian;
  o `.dic` já vem com formas plenas flexionadas, sem flags — usado direto, sem `unmunch`)
- **Processamento:** minúsculas + remoção de diacríticos via NFD + `ς → σ` +
  filtro `^[α-ωϛϙϟϡ]+$` + deduplicação. `Θεός → θεος`, `Ελλάδα → ελλαδα`.
- **Corte de tamanho (sem mudar a filtragem):** só formas com **≤ 10 letras** —
  o motor limita `totalLength ≤ 10`, então formas maiores nunca casariam;
  o descarte reduz ~60% do volume original (812 mil → 329 mil).
- **Licença dos dados:** a lista é fato linguístico (palavras), sem descrições autorais;
  verbetes com descrição continuam no dicionário curado embutido no código.

Para regenerar: ver `dictionaries/README.md` (guia da comunidade).
