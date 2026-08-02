import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open as pickDir, save } from "@tauri-apps/plugin-dialog";
import { openPath, revealItemInDir } from "@tauri-apps/plugin-opener";
import { join } from "@tauri-apps/api/path";

const $ = (id) => document.getElementById(id);

let inventory = null;
let currentExt = null;
let selectedPaths = new Set();
let sortBy = "name";
let lastUndo = [];
let lastUndoExt = null;
let scanning = false;

const els = {
  btnPick: $("btn-pick"),
  btnCancel: $("btn-cancel"),
  btnOpen: $("btn-open"),
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
  filePanel: $("file-panel"),
  emptyState: $("empty-state"),
  groupTitle: $("group-title"),
  groupStats: $("group-stats"),
  fileList: $("file-list"),
  checkAll: $("check-all"),
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
      els.checkAll.checked = false;
      renderGroups();
      renderFiles();
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
    row.setAttribute("aria-selected", String(selectedPaths.has(f.path)));

    const cbCell = document.createElement("span");
    const cb = document.createElement("input");
    cb.type = "checkbox";
    cb.checked = selectedPaths.has(f.path);
    cb.addEventListener("change", (e) => {
      toggleSelect(f.path, e.target.checked);
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
    row.addEventListener("click", (e) => {
      if (e.target === cb) return;
      const next = !selectedPaths.has(f.path);
      toggleSelect(f.path, next);
      updateButtons();
    });
    row.addEventListener("dblclick", () => openPath(f.path));
    els.fileList.appendChild(row);
  }

  els.btnMove.disabled = group.files.length === 0;
  updateButtons();
}

function toggleSelect(path, on) {
  if (on) selectedPaths.add(path);
  else selectedPaths.delete(path);
}

function updateButtons() {
  const any = selectedPaths.size > 0;
  els.btnOpen.disabled = !any;
  els.btnReveal.disabled = !any;
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

els.btnOpen.addEventListener("click", () => {
  const f = firstSelected();
  if (f) openPath(f.path);
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

renderAll();
