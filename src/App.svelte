<script>
  import "./TailwindCSS.css";
  import "bootstrap-icons/font/bootstrap-icons.css";
  import FileDrop from "svelte-tauri-filedrop";

  function createNewAccount() {
    console.log("creating new account...");
  }

  function open(paths) {
    console.log(paths);
  }
</script>

<!--DROP ZONE FILE-->
<FileDrop extensions={["txt"]} handleFiles={open} let:files>
  <div class:bg-nord10={files.length > 0} class="unselectable">

    <div class="text-white flex flex-col">
      <h1 class="text-5xl text-center">
        Добро пожаловать в <strong>Security Chat<strong> </strong></strong>
      </h1>

      <i class="bi bi-github text-white text-[150px] text-center"></i>

      <div class="text-center px-3">
        <h2>
          <strong>Security Chat</strong> используется в качестве аутификации
          специальный файл. Если вы его утратили, то
          <strong>восстановление данные невозможно</strong>
        </h2>
        <h2>Вы можете перенести файл в окно программы для авторизации</h2>
      </div>

      <button
        class="my-9 hover:bg-nord10 self-center text-whit font-bold h-10 w-[15rem] rounded-full shadow-lg bg-nord2"
        on:click={createNewAccount}>Создать новый аккаунт</button
      >

      <div class="flex flex-row-reverse mx-2 content-end">
        <i class="bi bi-github text-white text-3xl"></i>
      </div>
    </div>
    
  </div>
</FileDrop>

<style>
  .unselectable {
    -webkit-touch-callout: none; /* iOS Safari */
    -webkit-user-select: none; /* Chrome/Safari/Opera */
    -khtml-user-select: none; /* Konqueror */
    -moz-user-select: none; /* Firefox */
    -ms-user-select: none; /* Internet Explorer/Edge */
    user-select: none; /* Non-prefixed version, currently
                                  not supported by any browser */
  }
</style>
