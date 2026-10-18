# Wordlist de (filtro)

- **Arquivo canônico:** `dictionaries/de/words.txt` (164.691 formas únicas, ~2,1 MB)
- **Cópia servida ao frontend:** `public/dicts/de/words.txt` (idêntica; o frontend busca em runtime, fora do bundle JS)
- **Origem:** hunspell `de_DE_frami.dic`, o dicionário alemão completo do projeto
  LibreOffice, variante frami (Version: 20161207+frami20170109), derivado do
  projeto igerman98 de Björn Jacke com add-ons de Franz Michael Baumann.
  Fonte: https://github.com/LibreOffice/dictionaries/blob/master/de/de_DE_frami.dic
  (258.200 lemas no .dic original, codificação ISO-8859-1).
- **Processamento:** ISO-8859-1 → UTF-8, minúsculas, remoção das flags hunspell
  (palavra/flags vira palavra), remoção de diacríticos via NFD
  (Müller → muller), deduplicação e ordenação determinística (LC_ALL=C sort).
  Nomes próprios mantidos de propósito (mesma decisão dos wordlists en/es).
  Entradas com hífen, apóstrofo ou números são descartadas (mesma conduta do en).
  - Chave canônica segue o padrão Latino: filtro a-z com ß convertido para ss
    (Straße → strasse).
  - **Exceção documentada:** palavras com ß geram duas entradas — a chave sem
    ß (Straße → strasse) e a forma original com ß (straße). Motivo: a
    normalização de match do TS (normalizeForMatch) e do Rust (normalize_word)
    preserva ß intacto por design, então manter as duas formas faz a busca
    casar digitando ß ou ss.
- **Licença dos dados:** igerman98/frami é GPLv3; a lista é fato linguístico
  (palavras), sem descrições autorais; verbetes com descrição continuam no
  dicionário curado embutido (src/data/dictionaries.ts e
  shemot-rust/dictionaries/*.json).

Para regenerar: ver dictionaries/README.md (guia da comunidade).
