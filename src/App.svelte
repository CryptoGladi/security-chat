<script>
  import "./TailwindCSS.css";
  import "bootstrap-icons/font/bootstrap-icons.css";
  import FileDrop from "svelte-tauri-filedrop";

  let showModal = false;

  function toggleModal() {
    showModal = !showModal;
  }

  function createNewAccount() {
    console.log("creating new account...");
    toggleModal();
  }

  function open(paths) {
    console.log(paths);
  }
</script>

{#if showModal}
  <div
    class="overflow-x-hidden overflow-y-auto fixed inset-0 z-50 outline-none focus:outline-none justify-center items-center flex"
  >
    <div class="relative w-auto my-6 mx-auto max-w-sm">
      <!--content-->
      <div
        class="border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-nord1 outline-none focus:outline-none"
      >
        <!--header-->
        <div
          class="flex items-start justify-between p-5 border-b border-solid border-blueGray-200 rounded-t"
        >
          <h3 class="text-3xl font-semibold text-white">Меню регистрации</h3>
          <button
            class="p-1 ml-auto bg-transparent border-0 text-black opacity-5 float-right text-3xl leading-none font-semibold outline-none focus:outline-none"
            on:click={toggleModal}
          >
            <span
              class="bg-transparent text-black opacity-5 h-6 w-6 text-2xl block outline-none focus:outline-none"
            >
              ×
            </span>
          </button>
        </div>
        <!--body-->
        <div class="relative p-6 flex-auto">
          <label for="helper-text" class="block mb-2 font-s text-lb font-medium text-white">Ваш ник</label>
<input id="helper-text" aria-describedby="helper-text-explanation" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="CryptoGladi">
        </div>
        <!--footer-->
        <div
          class="flex items-center justify-end p-6 border-t border-solid border-blueGray-200 rounded-b"
        >
          <button
            class="text-red-500 bg-nord3 background-transparent font-bold uppercase px-6 py-2 text-sm outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
            type="button"
            on:click={toggleModal}
          >
            Закрыть
          </button>
          <button
            class="bg-nord3 text-white active:bg-emerald-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
            type="button"
            on:click={toggleModal}
          >
            Создать
          </button>
        </div>
      </div>
    </div>
  </div>
  <div class="opacity-25 fixed inset-0 z-40 bg-black"></div>
{/if}

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
