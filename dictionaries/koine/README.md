# Wordlist koine — Grego Koiné / Novo Testamento (filtro)

- **Arquivo canônico:** `dictionaries/koine/words.txt` (20.517 formas únicas, ~0,4 MB)
- **Cópia servida ao frontend:** `public/dicts/koine/words.txt` (idêntica; o frontend busca em runtime, fora do bundle JS)
- **Origem:** [MorphGNT: SBLGNT Edition](https://github.com/morphgnt/sblgnt)
  (Tauber, 2017; 27 livros, colunas *word* + *lemma*). Texto SBLGNT sob
  [SBLGNT EULA](http://sblgnt.com/license/); análise morfológica sob
  [CC-BY-SA 3.0](http://creativecommons.org/licenses/by-sa/3.0/).
  Extraídas formas de superfície **e** lemas (ex.: `λόγος` e `λέγω`),
  para casar tanto com o gerador quanto com consultas de dicionário.
- **Processamento:** minúsculas + remoção de diacríticos politônicos via NFD +
  `ς → σ` + filtro `^[α-ωϛϙϟϡ]+$` + deduplicação.
  `ἀγάπη → αγαπη`, `ΛΌΓΟΣ → λογος`. Corpus fechado: sem corte de tamanho.
- **Licença dos dados:** a lista é fato linguístico (palavras), sem descrições autorais;
  verbetes com descrição continuam no dicionário curado embutido no código.
  Ao redistribuir, mantenha a atribuição ao MorphGNT/SBLGNT acima.

Para regenerar: ver `dictionaries/README.md` (guia da comunidade).
