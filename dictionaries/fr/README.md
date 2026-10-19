# Français (filtro de wordlist)

Wordlist de formas-base (lemas) do francês para filtrar combinações geradas no
alfabeto Latino. Sem descrições (verbetes curados continuam no dicionário
hermético embutido em `src/data/dictionaries.ts` / `shemot-rust/dictionaries/`).

## Fonte

Extraída de `fr_FR.dic` do **hunspell-fr** (Dicollecte), empacotado pelo
projeto LibreOffice. Somente entradas puramente alfabéticas (tags e flags
hunspell descartadas; entradas com apóstrofo/hífen/dígitos removidas),
deduplicadas e em minúsculas. A normalização em runtime (lowercase + NFD +
remoção de diacríticos) é aplicada no carregamento — é o mesmo pipeline do
filtro `pt-BR`.

## Licença

MPL-2.0 (Mozilla Public License 2.0), conforme hunspell-fr / LibreOffice
dictionaries.
