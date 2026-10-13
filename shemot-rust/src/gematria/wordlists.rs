/*
 * @license
 * SPDX-License-Identifier: Apache-2.0
 *
 * Wordlists da comunidade (ver dictionaries/README.md).
 * O build.rs embute cada wordlist do registry via include_str!;
 * este módulo normaliza (igual ao TS em dictLoader.ts) e expõe
 * consultas O(1) via HashSet com cache em OnceLock.
 */

// Gerado pelo build.rs a partir de dictionaries/registry.json.
include!(concat!(env!("OUT_DIR"), "/wordlists_registry.rs"));
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

static SETS: OnceLock<HashMap<&'static str, HashSet<String>>> = OnceLock::new();

fn strip_latin_precomposed(c: char) -> Option<char> {
    // Equivalente ao NFD + remoção de \u0300-\u036f do TS para o alfabeto latino:
    // decompõe manualmente as pré-compostas com diacrítico; o resto passa
    // por lowercase intacto (ex.: ß, æ, ø ficam como estão, igual no TS).
    let base = match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' | 'ǎ' | 'ȁ' | 'ȃ' => 'a',
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' | 'ȅ' | 'ȇ' => 'e',
        'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ǐ' | 'ȉ' | 'ȋ' => 'i',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' | 'ǒ' | 'ȍ' | 'ȏ'
        | 'ọ' | 'ỏ' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' | 'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'o',
        'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' | 'ǔ' | 'ȕ' | 'ȗ'
        | 'ụ' | 'ủ' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => 'u',
        'ý' | 'ÿ' | 'ŷ' => 'y',
        'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => 'c',
        'ď' | 'đ' => 'd',
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => 'g',
        'ĥ' | 'ħ' => 'h',
        'ĵ' => 'j',
        'ķ' => 'k',
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => 'l',
        'ñ' | 'ń' | 'ņ' | 'ň' => 'n',
        'ŕ' | 'ŗ' | 'ř' => 'r',
        'ś' | 'ŝ' | 'ş' | 'š' | 'ș' => 's',
        'ţ' | 'ť' | 'ŧ' | 'ț' => 't',
        'ŵ' => 'w',
        'ź' | 'ż' | 'ž' => 'z',
        _ => return None,
    };
    Some(base)
}

/// Normalização canônica de match: minúsculas + sem diacríticos + trim.
pub fn normalize_word(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    for c in word.trim().chars().flat_map(|c| c.to_lowercase()) {
        let u = c as u32;
        if (0x0300..=0x036f).contains(&u) {
            continue; // marca combinante (ex.: İ → i + ponto)
        }
        if let Some(b) = strip_latin_precomposed(c) {
            out.push(b);
            continue;
        }
        if let Some(b) = strip_greek_precomposed(c) {
            out.push(b);
            continue;
        }
        out.push(c);
    }
    out
}

/// Equivalente ao NFD + remoção de \u0300-\u036f do TS para o grego:
/// reduz pré-compostos monotônicos e o bloco Greek Extended (politônicos,
/// ex.: NT SBLGNT) à vogal base; ς (sigma final) vira σ para casar com
/// consultas e listas. Aplica-se após to_lowercase (maiúsculas já caíram
/// nas faixas minúsculas), mas as faixas cobrem maiúsculas por segurança.
fn strip_greek_precomposed(c: char) -> Option<char> {
    match c {
        'ά' | 'ὰ' | 'ᾶ' => Some('α'),
        'έ' | 'ὲ' => Some('ε'),
        'ή' | 'ὴ' | 'ῆ' => Some('η'),
        'ί' | 'ὶ' | 'ῖ' | 'ϊ' | 'ΐ' => Some('ι'),
        'ό' | 'ὸ' => Some('ο'),
        'ύ' | 'ὺ' | 'ῦ' | 'ϋ' | 'ΰ' => Some('υ'),
        'ώ' | 'ὼ' | 'ῶ' => Some('ω'),
        'ς' => Some('σ'),
        _ => {
            let u = c as u32;
            if (0x1F00..=0x1F0F).contains(&u) {
                Some('α')
            } else if (0x1F10..=0x1F1F).contains(&u) {
                Some('ε')
            } else if (0x1F20..=0x1F2F).contains(&u) {
                Some('η')
            } else if (0x1F30..=0x1F3F).contains(&u) {
                Some('ι')
            } else if (0x1F40..=0x1F4F).contains(&u) {
                Some('ο')
            } else if (0x1F50..=0x1F57).contains(&u) || (0x1F59..=0x1F5F).contains(&u) {
                Some('υ')
            } else if (0x1F60..=0x1F6F).contains(&u) {
                Some('ω')
            } else if (0x1F80..=0x1F8F).contains(&u) {
                Some('α') // + ypogegrammeni
            } else if (0x1F90..=0x1F9F).contains(&u) {
                Some('η') // + ypogegrammeni
            } else if (0x1FA0..=0x1FAF).contains(&u) {
                Some('ω') // + ypogegrammeni
            } else if (0x1FB0..=0x1FBB).contains(&u) || c == '\u{1FBC}' {
                Some('α')
            } else if (0x1FC2..=0x1FC7).contains(&u) || c == '\u{1FCC}' {
                Some('η')
            } else if (0x1FC8..=0x1FCB).contains(&u) {
                Some('ε')
            } else if (0x1FD0..=0x1FD3).contains(&u)
                || (0x1FD6..=0x1FDB).contains(&u)
            {
                Some('ι')
            } else if (0x1FE0..=0x1FE3).contains(&u)
                || (0x1FE6..=0x1FEB).contains(&u)
            {
                Some('υ')
            } else if (0x1FE4..=0x1FE5).contains(&u) || (0x1FEC..=0x1FEF).contains(&u) {
                Some('ρ') // rô com espírito (ex.: ῥήτωρ)
            } else if (0x1FF2..=0x1FF7).contains(&u) || c == '\u{1FFC}' {
                Some('ω')
            } else if (0x1FF8..=0x1FFB).contains(&u) {
                Some('ο')
            } else {
                None
            }
        }
    }
}

fn sets() -> &'static HashMap<&'static str, HashSet<String>> {
    SETS.get_or_init(|| {
        let mut map = HashMap::new();
        for meta in WORDLISTS {
            let mut set = HashSet::new();
            for line in meta.words.lines() {
                let key = normalize_word(line);
                if !key.is_empty() {
                    set.insert(key);
                }
            }
            map.insert(meta.id, set);
        }
        map
    })
}

/// ids das wordlists embutidas.
pub fn wordlist_ids() -> Vec<&'static str> {
    WORDLISTS.iter().map(|m| m.id).collect()
}

/// true se a palavra (ex.: "CORAÇÃO", "Logos (LOGOS)") está na wordlist `id`.
pub fn wordlist_contains(id: &str, raw_word: &str) -> bool {
    let base = raw_word.split(' ').next().unwrap_or(raw_word);
    let key = normalize_word(base);
    sets().get(id).map(|s| s.contains(&key)).unwrap_or(false)
}

/// ids de todas as wordlists que contêm a palavra.
pub fn matched_wordlists(raw_word: &str) -> Vec<String> {
    let base = raw_word.split(' ').next().unwrap_or(raw_word);
    let key = normalize_word(base);
    let map = sets();
    let mut out = Vec::new();
    for meta in WORDLISTS {
        if map.get(meta.id).map(|s| s.contains(&key)).unwrap_or(false) {
            out.push(meta.id.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_matches_typescript() {
        assert_eq!(normalize_word("Coração"), "coracao");
        assert_eq!(normalize_word("  CORACAO  "), "coracao");
        assert_eq!(normalize_word("Aarão"), "aarao");
        assert_eq!(normalize_word("naïve"), "naive");
        assert_eq!(normalize_word("Œuvre"), "œuvre");
        assert_eq!(normalize_word("Straße"), "straße");
    }

    #[test]
    fn normalization_greek_matches_typescript() {
        // Politônicos (NT) e monotônicos reduzem à base; ς final vira σ.
        assert_eq!(normalize_word("ἀγάπη"), "αγαπη");
        assert_eq!(normalize_word("ΛΌΓΟΣ"), "λογοσ");
        assert_eq!(normalize_word("Ὀδυσσεύς"), "οδυσσευσ");
        assert_eq!(normalize_word("καὶ"), "και");
        assert_eq!(normalize_word("ᾨ"), "ω");
        assert_eq!(normalize_word("Ῥώμη"), "ρωμη");
        assert_eq!(normalize_word("  ἘΝ ἈΡΧῇ  "), "εν αρχη");
    }

    #[test]
    fn registry_embeds_ptbr() {
        assert!(wordlist_ids().contains(&"pt-BR"));
    }

    #[test]
    fn registry_embeds_en_es() {
        assert!(wordlist_ids().contains(&"en"));
        assert!(wordlist_ids().contains(&"es"));
    }

    #[test]
    fn ptbr_contains_common_words() {
        assert!(wordlist_contains("pt-BR", "AMOR"));
        assert!(wordlist_contains("pt-BR", "coração"));
        assert!(wordlist_contains("pt-BR", "Alquimia (ALQUIMIA)"));
        assert!(!wordlist_contains("pt-BR", "XQZKWJQQ"));
        assert!(!wordlist_contains("pt-BR", ""));
    }

    #[test]
    fn matched_lists_ptbr() {
        let m = matched_wordlists("DEUS");
        assert!(m.contains(&"pt-BR".to_string()));
        assert!(matched_wordlists("XQZKWJQQ").is_empty());
    }

    #[test]
    fn en_es_contain_common_words() {
        assert!(wordlist_contains("en", "LOVE"));
        assert!(wordlist_contains("en", "god"));
        assert!(wordlist_contains("en", "Wisdom (WISDOM)"));
        assert!(wordlist_contains("es", "AMOR"));
        assert!(wordlist_contains("es", "corazón"));
        assert!(!wordlist_contains("en", "XQZKWJQQ"));
        assert!(!wordlist_contains("es", "XQZKWJQQ"));
    }

    #[test]
    fn el_koine_contain_common_words() {
        // Koiné (SBLGNT): Jo 1.1 — consulta politônica e monotônica casam.
        assert!(wordlist_contains("koine", "ΛΟΓΟΣ"));
        assert!(wordlist_contains("koine", "λόγος"));
        assert!(wordlist_contains("koine", "ΘΕΟΣ"));
        assert!(wordlist_contains("koine", "ἀγάπη"));
        // Moderno (hunspell-el, formas plenas).
        assert!(wordlist_contains("el", "ΑΓΑΠΗ"));
        assert!(wordlist_contains("el", "θεός"));
        assert!(!wordlist_contains("koine", "XQZKWJQQ"));
        assert!(!wordlist_contains("el", "XQZKWJQQ"));
    }
}
