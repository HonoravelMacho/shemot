# Wordlist tanakh — Hebraico Bíblico / Tanakh (filtro)

- **Arquivo canônico:** `dictionaries/tanakh/words.txt` (45.224 formas únicas, ~0,5 MB)
- **Cópia servida ao frontend:** `public/dicts/tanakh/words.txt` (idêntica; o frontend busca em runtime, fora do bundle JS)
- **Origem:** [Open Scriptures Hebrew Bible](https://github.com/openscriptures/morphhb)
  (Westminster Leningrad Codex, 40 arquivos XML). Texto WLC em domínio público;
  morfologia OSHB sob [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/).
  Extraídas as formas de superfície de cada `<w>` em **duas** variantes:
  segmentos morfêmicos (`ב` + `ראשית`) **e** a forma plena junta (`בראשית`),
  para casar tanto com prefixos isolados quanto com palavras completas geradas.
  (Os `lemma` são números de Strong, não formas hebraicas — ignorados.)
- **Processamento:** remoção de niqqud/cantilação (U+0591–U+05C7) + filtro
  `^[א-ת]+$` (sofit preservada) + deduplicação. `בְּרֵאשִׁית → בראשית`.
  Corpus fechado: sem corte de tamanho.
- **Licença dos dados:** a lista é fato linguístico (palavras), sem descrições autorais;
  verbetes com descrição continuam no dicionário curado embutido no código.
  Ao redistribuir, mantenha a atribuição ao OSHB/WLC acima.

Para regenerar: ver `dictionaries/README.md` (guia da comunidade).
