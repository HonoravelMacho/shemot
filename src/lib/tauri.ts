/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 *
 * Wrapper Tauri v2 (desktop + Android) com fallback para navegador.
 * Mantém a engine Rust como fonte de verdade para combinatória;
 * o motor TS em GematriaCalculator.tsx só é usado no browser.
 */

export interface RichResult {
  word: string;
  has_meaning: boolean;
  translation: string;
  description: string;
  original_word: string;
  /** ids das wordlists da comunidade que contêm a palavra (ex.: ["pt-BR"]). */
  in_wordlists?: string[];
}

export interface SearchProgress {
  tested: number;
  found: number;
}

export function isAndroid(): boolean {
  if (typeof navigator === "undefined") return false;
  const ua = navigator.userAgent || "";
  if (/android/i.test(ua)) return true;
  // WebViews antigas podem omitir "Android" do UA: heurística de fallback
  // (Linux + touchscreen + sem X11 típico de desktop).
  if (/linux/i.test(ua) && !/x11/i.test(ua)) {
    try {
      if (navigator.maxTouchPoints > 0 && window.screen.width < 1200) return true;
    } catch {
      /* noop */
    }
  }
  return false;
}

export interface SaveResult {
  ok: boolean;
  /** Caminho onde foi salvo (quando ok). */
  path: string;
  /** Motivo da falha / "cancelado pelo usuário" (quando !ok). */
  error: string;
}

/**
 * Salva um .txt via APIs nativas do Tauri.
 * - Desktop: diálogo "Salvar como" (plugin-dialog) + escrita (plugin-fs).
 * - Android: comando Rust `save_text_file_native` — tenta Downloads público
 *   e cai para a pasta privada do app (scoped storage do Android 10+ barra
 *   escrita direta em Downloads; a pasta do app é sempre gravável).
 *   A UI deve oferecer "Compartilhar" (shareTextAsFile) para levar o
 *   arquivo a Downloads/Drive/WhatsApp a partir da pasta do app.
 * Retorna ok:false para a UI oferecer o fallback de copiar o texto.
 */
export async function saveTextFileNative(
  filename: string,
  contents: string,
): Promise<SaveResult> {
  try {
    if (!isAndroid()) {
      const { save } = await import("@tauri-apps/plugin-dialog");
      const path = await save({
        defaultPath: filename,
        filters: [{ name: "Texto", extensions: ["txt"] }],
      });
      if (!path) return { ok: false, path: "", error: "cancelado pelo usuário" };
      const { writeTextFile } = await import("@tauri-apps/plugin-fs");
      await writeTextFile(path, contents);
      return { ok: true, path, error: "" };
    }

    // Android (Tauri): comando Rust com fallback interno
    // Downloads → pasta do app. Nunca usa o diálogo (inexistente no mobile).
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const path = await invoke<string>("save_text_file_native", {
        filename,
        contents,
      });
      return { ok: true, path, error: "" };
    } catch (e) {
      // Fallback direto via plugin-fs (pasta privada, sempre permitida
      // após a capability $APPDATA/**).
      const { writeTextFile, BaseDirectory } = await import(
        "@tauri-apps/plugin-fs"
      );
      await writeTextFile(filename, contents, {
        baseDir: BaseDirectory.AppData,
      });
      return { ok: true, path: `pasta do app/${filename}`, error: "" };
    }
  } catch (e) {
    return { ok: false, path: "", error: String(e) };
  }
}

/** O Web Share API com arquivos está disponível? (Android Chrome/WebView.) */
export function canShareFiles(): boolean {
  try {
    if (typeof navigator === "undefined" || !("share" in navigator)) return false;
    if (typeof File === "undefined") return false;
    const nav = navigator as Navigator & {
      canShare?: (data: { files: File[] }) => boolean;
    };
    if (typeof nav.canShare === "function") {
      return nav.canShare({
        files: [new File([""], "teste.txt", { type: "text/plain" })],
      });
    }
    return true;
  } catch {
    return false;
  }
}

/**
 * Abre o seletor nativo de compartilhamento (Downloads, Drive, WhatsApp…)
 * com o conteúdo como anexo .txt. Retorna true se o sheet abriu.
 */
export async function shareTextAsFile(
  filename: string,
  contents: string,
): Promise<boolean> {
  try {
    if (typeof navigator === "undefined" || !("share" in navigator)) return false;
    const file = new File([contents], filename, { type: "text/plain" });
    await (navigator as Navigator & { share: (d: unknown) => Promise<void> }).share({
      files: [file],
      title: filename,
    });
    return true;
  } catch (e) {
    // AbortError = usuário fechou o sheet: considera "ok, sem erro".
    if (e instanceof DOMException && e.name === "AbortError") return true;
    return false;
  }
}
export function isTauriRuntime(): boolean {
  if (typeof window === "undefined") return false;
  const w = window as unknown as Record<string, unknown>;
  return (
    w.__TAURI_INTERNALS__ !== undefined ||
    w.__TAURI__ !== undefined ||
    // @tauri-apps/api define __TAURI_EVENT_PLUGIN_INTERNALS__ no v2
    w.__TAURI_EVENT_PLUGIN_INTERNALS__ !== undefined
  );
}

export async function invokeSearch(
  command: "run_gematria_search",
  args: Record<string, unknown>,
): Promise<RichResult[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<RichResult[]>(command, args);
}

export async function invokeCancel(): Promise<void> {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("cancel_search");
  } catch (err) {
    console.error("cancel_search failed:", err);
  }
}

export async function listenProgress(
  cb: (p: SearchProgress) => void,
): Promise<() => void> {
  const { listen } = await import("@tauri-apps/api/event");
  const unlisten = await listen<SearchProgress>("search_progress", (event) => {
    cb(event.payload);
  });
  return unlisten;
}
