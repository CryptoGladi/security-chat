import App from "./App.svelte";
import Main from "./pages/Main.svelte";
import Welcom from "./pages/Welcom.svelte";

const app = new Main({
  target: document.getElementById("app"),
});

export default app;
