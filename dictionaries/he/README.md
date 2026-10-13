# Wordlist he — Hebraico Moderno (filtro)

- **Arquivo canônico:** `dictionaries/he/words.txt` (447.303 formas únicas, ~6,3 MB)
- **Cópia servida ao frontend:** `public/dicts/he/words.txt` (idêntica; o frontend busca em runtime, fora do bundle JS)
- **Origem:** `hunspell-he` (dicionário hebraico do hunspell, pacote Debian).
  A expansão total via `unmunch` gera 62 milhões de formas (inviável embarcar);
  usamos as formas-base do `.dic` (sem flags) — mesmo compromisso das outras listas.
- **Processamento:** remoção de niqqud/cantilação (U+0591–U+05C7) + filtro
  `^[א-ת]+$` (sofit ךםןףץ preservada, como o motor gera) + deduplicação.
  `שָׁלוֹם → שלום`.
- **Corte de tamanho (sem mudar a filtragem):** só formas com **≤ 10 letras** —
  o motor limita `totalLength ≤ 10`, então formas maiores nunca casariam.
- **Licença dos dados:** a lista é fato linguístico (palavras), sem descrições autorais;
  verbetes com descrição continuam no dicionário curado embutido no código.

Para regenerar: ver `dictionaries/README.md` (guia da comunidade).
