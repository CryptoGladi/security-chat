import Main from "./pages/Main.svelte";
import Welcom from "./pages/Welcom.svelte";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, LogicalSize } from '@tauri-apps/api/window';

async function get_form() {
  let have_account = await invoke("have_account");

  if (!have_account) {
    return new Welcom({
      target: document.getElementById("app"),
    });
  }
  else {
    await appWindow.setResizable(true);
    await appWindow.setFullscreen(true);
    await appWindow.setSize(new LogicalSize(1000, 1000));

    return new Main({
      target: document.getElementById("app"),
    });
  }
}

export default get_form();
