window.customAlert = function (msg) {
  window.__TAURI__.dialog.message(msg, {
    title: "Compo",
    kind: "info"
  });
};