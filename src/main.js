const globalApi = window.__TAURI__;
if (!globalApi || !globalApi.core || !globalApi.core.invoke) {
  const body = document.body;
  body.innerHTML =
    '<div style="padding:2rem;font-family:sans-serif;color:#fff">' +
    "<h3>Ошибка запуска</h3>" +
    "<p>Глобальный API Tauri (<code>window.__TAURI__</code>) недоступен. " +
    "Приложение должно запускаться через Tauri (tauri dev / tauri build).</p></div>";
  throw new Error("window.__TAURI__ is not available");
}

const { invoke, convertFileSrc } = globalApi.core;
const { listen } = globalApi.event;
const { open: pickDir, save } = globalApi.dialog;
const { revealItemInDir } = globalApi.opener;
const { join } = globalApi.path;

const $ = (id) => document.getElementById(id);

let inventory = null;
let currentExt = null;
let selectedPaths = new Set();
let focusedPath = null;
let sortBy = "name";
let lastUndo = [];
let lastUndoExt = null;
let scanning = false;

const els = {
  btnPick: $("btn-pick"),
  btnCancel: $("btn-cancel"),
  btnReveal: $("btn-reveal"),
  btnMove: $("btn-move"),
  btnUndo: $("btn-undo"),
  btnExport: $("btn-export"),
  rootPath: $("root-path"),
  progressWrap: $("scan-progress-wrap"),
  progressFill: $("scan-progress-fill"),
  progressCount: $("scan-progress-count"),
  tabTypes: $("tab-types"),
  tabEmpty: $("tab-empty"),
  panelTypes: $("panel-types"),
  panelEmpty: $("panel-empty"),
  extSearch: $("ext-search"),
  fileSearch: $("file-search"),
  sortSelect: $("sort-select"),
  groupList: $("group-list"),
  emptyList: $("empty-list"),
  emptyHint: $("empty-hint"),
  emptyBadge: $("empty-count-badge"),
  sidebar: $("sidebar"),
  resizerSide: $("resizer-side"),
  resizerPreview: $("resizer-preview"),
  filePanel: $("file-panel"),
  emptyState: $("empty-state"),
  groupTitle: $("group-title"),
  groupStats: $("group-stats"),
  fileList: $("file-list"),
  checkAll: $("check-all"),
  preview: $("preview"),
  previewTitle: $("preview-title"),
  previewBody: $("preview-body"),
  btnClosePreview: $("btn-close-preview"),
  statFiles: $("stat-files"),
  statSize: $("stat-size"),
  statDirs: $("stat-dirs"),
  statEmpty: $("stat-empty"),
  msg: $("msg"),
};

function fmtSize(bytes) {
  if (bytes < 1024) return `${bytes} Б`;
  const units = ["КБ", "МБ", "ГБ", "ТБ"];
  let v = bytes;
  let i = -1;
  do {
    v /= 1024;
    i += 1;
  } while (v >= 1024 && i < units.length - 1);
  return `${v.toFixed(1)} ${units[i]}`;
}

function fmtDate(ms) {
  if (!ms) return "—";
  return new Date(ms).toLocaleString("ru-RU", {
    dateStyle: "medium",
    timeStyle: "short",
  });
}

function extLabel(ext) {
  return ext === "" ? "(без расширения)" : `.${ext}`;
}

function setMsg(text, kind = "") {
  els.msg.textContent = text;
  els.msg.className = "msg" + (kind ? ` ${kind}` : "");
}

async function scanFolder(path) {
  scanning = true;
  els.progressWrap.classList.remove("hidden");
  els.progressFill.classList.add("indeterminate");
  els.progressFill.style.width = "0%";
  els.progressCount.textContent = "0";
  els.btnCancel.disabled = false;
  setMsg(`Сканирование: ${path}`, "");

  try {
    inventory = await invoke("scan_folder", { path });
    if (inventory.cancelled) {
      setMsg("Сканирование отменено.", "error");
    } else {
      setMsg(`Готово: ${inventory.total_files} файлов.`);
    }
  } catch (e) {
    setMsg(`Ошибка сканирования: ${e}`, "error");
  } finally {
    scanning = false;
    els.progressWrap.classList.add("hidden");
    els.btnCancel.disabled = true;
  }

  renderAll();
}

async function rescan() {
  if (inventory && inventory.root) {
    await scanFolder(inventory.root);
  }
}

function renderAll() {
  renderGroups();
  renderEmpty();
  renderStats();
  renderFiles();
  updatePreview();
}

function renderStats() {
  if (!inventory) {
    els.statFiles.textContent = "";
    els.statSize.textContent = "";
    els.statDirs.textContent = "";
    els.statEmpty.textContent = "";
    els.btnExport.disabled = true;
    return;
  }
  els.statFiles.textContent = `Файлов: ${inventory.total_files}`;
  els.statSize.textContent = `Объём: ${fmtSize(inventory.total_size)}`;
  els.statDirs.textContent = `Папок: ${inventory.total_dirs}`;
  els.statEmpty.textContent = `Пустых папок: ${inventory.empty_dirs.length}`;
  els.btnExport.disabled = false;
}

function renderGroups() {
  if (!inventory) {
    els.groupList.innerHTML = "";
    return;
  }
  const q = els.extSearch.value.trim().toLowerCase();
  const groups = inventory.groups.filter((g) =>
    q ? g.extension.toLowerCase().includes(q) : true,
  );

  els.groupList.innerHTML = "";
  for (const g of groups) {
    const li = document.createElement("li");
    if (g.extension === currentExt) li.classList.add("selected");

    const badge = document.createElement("span");
    badge.className = "ext-badge" + (g.extension === "" ? " ext-noext" : "");
    badge.textContent = g.extension === "" ? "∅" : g.extension;

    const meta = document.createElement("span");
    meta.className = "group-meta";
    const name = document.createElement("span");
    name.className = "name";
    name.textContent = extLabel(g.extension);
    const count = document.createElement("span");
    count.className = "count";
    count.textContent = `${g.count} файл${plural(g.count)}`;
    meta.append(name, count);

    const size = document.createElement("span");
    size.className = "ext-size";
    size.textContent = fmtSize(g.total_size);

    li.append(badge, meta, size);
    li.addEventListener("click", () => {
      currentExt = g.extension;
      selectedPaths.clear();
      focusedPath = null;
      els.checkAll.checked = false;
      renderGroups();
      renderFiles();
      updatePreview();
    });
    els.groupList.appendChild(li);
  }
}

function plural(n) {
  const m10 = n % 10;
  const m100 = n % 100;
  if (m10 === 1 && m100 !== 11) return "";
  if (m10 >= 2 && m10 <= 4 && (m100 < 10 || m100 >= 20)) return "а";
  return "ов";
}

function renderEmpty() {
  if (!inventory) {
    els.emptyList.innerHTML = "";
    els.emptyBadge.classList.add("hidden");
    return;
  }
  const n = inventory.empty_dirs.length;
  els.emptyBadge.textContent = n;
  els.emptyBadge.classList.toggle("hidden", n === 0);
  els.emptyHint.textContent = n
    ? `Папок без файлов: ${n}. Клик — открыть в проводнике.`
    : "Пустых папок нет.";

  els.emptyList.innerHTML = "";
  for (const rel of inventory.empty_dirs) {
    const li = document.createElement("li");
    const full = join(inventory.root, rel);
    li.textContent = rel;
    li.title = "Открыть в проводнике";
    li.addEventListener("click", () => revealItemInDir(full));
    els.emptyList.appendChild(li);
  }
}

function currentFiles() {
  if (!inventory || currentExt === null) return [];
  const g = inventory.groups.find((x) => x.extension === currentExt);
  return g ? g.files : [];
}

function renderFiles() {
  if (!inventory || currentExt === null) {
    els.emptyState.classList.remove("hidden");
    els.filePanel.classList.add("hidden");
    return;
  }
  els.emptyState.classList.add("hidden");
  els.filePanel.classList.remove("hidden");

  const group = inventory.groups.find((x) => x.extension === currentExt);
  if (!group) return;

  els.groupTitle.textContent = extLabel(group.extension);
  els.groupStats.textContent = `${group.count} файл${plural(group.count)}, ${fmtSize(group.total_size)}`;

  const q = els.fileSearch.value.trim().toLowerCase();
  let files = group.files.filter((f) =>
    q ? f.name.toLowerCase().includes(q) : true,
  );

  const sorters = {
    name: (a, b) => a.name.localeCompare(b.name, "ru"),
    size: (a, b) => b.size - a.size,
    date: (a, b) => b.modified_ms - a.modified_ms,
    path: (a, b) => a.rel_path.localeCompare(b.rel_path),
  };
  files = [...files].sort(sorters[sortBy] || sorters.name);

  els.fileList.innerHTML = "";
  for (const f of files) {
    const row = document.createElement("div");
    row.className = "file-row";
    row.setAttribute("role", "option");
    row.classList.toggle("selected", selectedPaths.has(f.path));
    row.classList.toggle("focused", f.path === focusedPath);
    row.setAttribute("aria-selected", String(selectedPaths.has(f.path)));
    row.__path = f.path;

    const cbCell = document.createElement("span");
    const cb = document.createElement("input");
    cb.type = "checkbox";
    cb.checked = selectedPaths.has(f.path);
    cb.addEventListener("change", (e) => {
      toggleSelect(f.path, e.target.checked);
      row.classList.toggle("selected", e.target.checked);
      row.setAttribute("aria-selected", String(e.target.checked));
      updateButtons();
    });
    cbCell.appendChild(cb);

    const name = document.createElement("span");
    name.className = "col-name";
    name.textContent = f.name;
    name.title = f.name;

    const size = document.createElement("span");
    size.className = "col-size";
    size.textContent = fmtSize(f.size);

    const date = document.createElement("span");
    date.className = "col-date";
    date.textContent = fmtDate(f.modified_ms);

    const path = document.createElement("span");
    path.className = "col-path";
    path.textContent = f.rel_path;
    path.title = f.rel_path;

    row.append(cbCell, name, size, date, path);
    row.addEventListener("click", () => {
      setFocus(f.path);
      updatePreview();
    });
    els.fileList.appendChild(row);
  }

  els.btnMove.disabled = group.files.length === 0;
  updateButtons();
}

function setFocus(path) {
  focusedPath = path;
  for (const row of els.fileList.children) {
    row.classList.toggle("focused", row.__path === path);
  }
}

function toggleSelect(path, on) {
  if (on) selectedPaths.add(path);
  else selectedPaths.delete(path);
}

function updateButtons() {
  const any = selectedPaths.size > 0;
  els.btnReveal.disabled = !any;
}

let previewRequest = 0;
let lastPreviewPath = null;

function setPreviewVisible(visible) {
  els.preview.classList.toggle("hidden", !visible);
  els.resizerPreview.classList.toggle("hidden", !visible);
}

async function updatePreview() {
  if (!inventory || !focusedPath) {
    lastPreviewPath = null;
    setPreviewVisible(false);
    return;
  }
  const f = currentFiles().find((x) => x.path === focusedPath);
  if (!f) {
    lastPreviewPath = null;
    setPreviewVisible(false);
    return;
  }
  if (f.path === lastPreviewPath && !els.preview.classList.contains("hidden")) return;
  lastPreviewPath = f.path;

  setPreviewVisible(true);
  els.previewTitle.textContent = `${f.name} · ${fmtSize(f.size)}`;
  els.previewBody.innerHTML = "";
  const loading = document.createElement("p");
  loading.className = "preview-note";
  loading.textContent = "Загрузка предпросмотра…";
  els.previewBody.appendChild(loading);

  const req = ++previewRequest;
  const note = (text) => {
    const p = document.createElement("p");
    p.className = "preview-note";
    p.textContent = text;
    return p;
  };

  try {
    const res = await invoke("preview_file", { path: f.path });
    if (req !== previewRequest) return;
    els.previewBody.innerHTML = "";
    if (res.kind === "image") {
      const img = document.createElement("img");
      img.src = `data:${res.mime};base64,${res.data}`;
      img.alt = f.name;
      els.previewBody.appendChild(img);
    } else if (res.kind === "text") {
      const pre = document.createElement("pre");
      pre.textContent = res.truncated
        ? `(показаны первые 256 КБ)\n\n${res.data}`
        : res.data;
      els.previewBody.appendChild(pre);
    } else if (res.kind === "video") {
      const video = document.createElement("video");
      video.controls = true;
      video.preload = "metadata";
      if (video.canPlayType(res.mime)) {
        video.src = convertFileSrc(f.path);
        els.previewBody.appendChild(video);
      } else {
        els.previewBody.appendChild(
          note(`Встроенный предпросмотр не поддерживает этот видеокодек (${extLabel(f.extension)}).`),
        );
      }
    } else {
      els.previewBody.appendChild(note(res.note));
    }
  } catch (e) {
    if (req !== previewRequest) return;
    els.previewBody.innerHTML = "";
    els.previewBody.appendChild(note(`Ошибка предпросмотра: ${e}`));
  }
}

function switchTab(which) {
  const isTypes = which === "types";
  els.tabTypes.classList.toggle("active", isTypes);
  els.tabEmpty.classList.toggle("active", !isTypes);
  els.panelTypes.classList.toggle("hidden", !isTypes);
  els.panelEmpty.classList.toggle("hidden", isTypes);
}

els.btnPick.addEventListener("click", async () => {
  const dir = await pickDir({ directory: true, title: "Выберите папку для сканирования" });
  if (!dir) return;
  els.rootPath.textContent = dir;
  await scanFolder(dir);
});

els.btnCancel.addEventListener("click", () => {
  invoke("cancel_scan");
});

els.btnReveal.addEventListener("click", () => {
  const f = firstSelected();
  if (f) revealItemInDir(f.path);
});

function firstSelected() {
  const files = currentFiles();
  return files.find((f) => selectedPaths.has(f.path)) || null;
}

els.btnMove.addEventListener("click", async () => {
  const files = currentFiles();
  if (!files.length) return;

  let targets = files.filter((f) => selectedPaths.has(f.path));
  if (!targets.length) targets = files;

  const destDir = await pickDir({
    directory: true,
    title: "Куда переместить файлы?",
  });
  if (!destDir) return;

  const ok = confirm(
    `Переместить ${targets.length} файл${plural(targets.length)} типа «${extLabel(currentExt)}»\nв папку: ${destDir}?\n\nПерезапись не производится — дубли получат суффикс «(1)».`,
  );
  if (!ok) return;

  const items = targets.map((f) => ({ src: f.path, dest_dir: destDir }));
  const report = await invoke("move_files", { items });
  lastUndo = report.moved;
  lastUndoExt = currentExt;
  els.btnUndo.disabled = lastUndo.length === 0;

  if (report.errors.length) {
    setMsg(`Перемещено ${report.moved.length}, ошибок: ${report.errors.length}`, "error");
    console.error("move errors", report.errors);
  } else {
    setMsg(`Перемещено файлов: ${report.moved.length}.`, "ok");
  }
  selectedPaths.clear();
  focusedPath = null;
  await rescan();
});

els.btnUndo.addEventListener("click", async () => {
  if (!lastUndo.length) return;
  const ok = confirm(
    `Отменить последнее перемещение (${lastUndo.length} файл${plural(lastUndo.length)})?`,
  );
  if (!ok) return;
  const report = await invoke("undo_move", { items: lastUndo });
  if (report.errors.length) {
    setMsg(`Откат: ${report.moved.length} возвращено, ошибок: ${report.errors.length}`, "error");
  } else {
    setMsg(`Откат выполнен: ${report.moved.length} файл${plural(report.moved.length)}.`, "ok");
  }
  lastUndo = [];
  els.btnUndo.disabled = true;
  selectedPaths.clear();
  focusedPath = null;
  await rescan();
});

els.btnExport.addEventListener("click", async () => {
  if (!inventory) return;
  const fmt = await save({
    title: "Экспорт инвентаря",
    defaultPath: "inventory.csv",
    filters: [
      { name: "CSV", extensions: ["csv"] },
      { name: "JSON", extensions: ["json"] },
    ],
  });
  if (!fmt) return;

  const ext = fmt.toLowerCase().endsWith(".json") ? "json" : "csv";
  const rows = [];
  for (const g of inventory.groups) {
    for (const f of g.files) {
      rows.push({
        rel_path: f.rel_path,
        name: f.name,
        extension: f.extension,
        size: f.size,
        modified_ms: f.modified_ms,
      });
    }
  }

  try {
    await invoke("export_inventory", {
      rows,
      format: ext,
      dest: fmt,
      delimiter: ";",
    });
    setMsg(`Экспорт сохранён: ${fmt}`, "ok");
  } catch (e) {
    setMsg(`Ошибка экспорта: ${e}`, "error");
  }
});

els.checkAll.addEventListener("change", (e) => {
  const on = e.target.checked;
  for (const f of currentFiles()) {
    if (on) selectedPaths.add(f.path);
    else selectedPaths.delete(f.path);
  }
  renderFiles();
  updatePreview();
});

els.btnClosePreview.addEventListener("click", () => {
  setPreviewVisible(false);
});

els.extSearch.addEventListener("input", renderGroups);
els.fileSearch.addEventListener("input", renderFiles);
els.sortSelect.addEventListener("change", (e) => {
  sortBy = e.target.value;
  renderFiles();
});
els.tabTypes.addEventListener("click", () => switchTab("types"));
els.tabEmpty.addEventListener("click", () => switchTab("empty"));

listen("scan-progress", (event) => {
  const n = event.payload;
  els.progressCount.textContent = String(n);
  els.progressFill.classList.remove("indeterminate");
  const pct = Math.min(100, Math.max(4, (n % 1000) / 10));
  els.progressFill.style.width = `${pct}%`;
});

function makeResizer(handle, target, key, min, max) {
  const saved = Number(localStorage.getItem(key));
  if (saved && saved >= min && saved <= max) {
    target.style.width = `${saved}px`;
  }
  handle.addEventListener("mousedown", (e) => {
    e.preventDefault();
    handle.classList.add("active");
    document.body.classList.add("resizing");
    const startSize = target.offsetWidth;
    const startX = e.clientX;
    const onMove = (ev) => {
      const size = Math.min(max, Math.max(min, startSize + (ev.clientX - startX)));
      target.style.width = `${size}px`;
    };
    const onUp = () => {
      handle.classList.remove("active");
      document.body.classList.remove("resizing");
      document.removeEventListener("mousemove", onMove);
      document.removeEventListener("mouseup", onUp);
      localStorage.setItem(key, String(target.offsetWidth));
    };
    document.addEventListener("mousemove", onMove);
    document.addEventListener("mouseup", onUp);
  });
}

makeResizer(els.resizerSide, els.sidebar, "fl.sidebar.w", 200, 600);
makeResizer(els.resizerPreview, els.preview, "fl.preview.w", 240, 900);

renderAll();
