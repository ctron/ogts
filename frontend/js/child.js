const invoke = window.__TAURI__.invoke;

export async function childCreate(child) {
  return await invoke("child", {name: name});
}
export async function childList(opts) {
  return await invoke("hello", {name: name});
}

