import Main from "./pages/Main.svelte";
import Welcom from "./pages/Welcom.svelte";
import { invoke } from "@tauri-apps/api/tauri";

async function get_form() {
  let have_account = await invoke("have_account");

  if (!have_account) {
    return new Welcom({
      target: document.getElementById("app"),
    });
  } else {
    return new Main({
      target: document.getElementById("app"),
    });
  }
}

export default get_form();
