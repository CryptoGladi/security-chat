<script>
    import "../TailwindCSS.css";
    import "bootstrap-icons/font/bootstrap-icons.css";
    import { open_that, registration } from "../Api.js";
    import { invoke } from "@tauri-apps/api/tauri";
    import FileDrop from "svelte-tauri-filedrop";
  
    let showRegistrationMenu = false;
    let nicknameIsTaken = false;
    let nicknameValue = "";
  
    function toggleModal() {
      showRegistrationMenu = !showRegistrationMenu;
    }
  
    function createNewAccount() {
      console.log("creating new account...");
      toggleModal();
    }
  
    function openAuthFile(paths) {
      console.log(paths);
    }
  
    function openRepo() {
      open_that("https://github.com/CryptoGladi/security-chat");
    }
  
    function checkNickname() {
      invoke("nickname_is_taken", { nickname: nicknameValue }).then((n) => {
        nicknameIsTaken = n;
      });
    }
  
    function registrationNewAccount() {
      invoke("nickname_is_taken", { nickname: nicknameValue }).then((n) => {
        if (!n) registration(nicknameValue);
      });
    }
  </script>
  
  {#if showRegistrationMenu}
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
          </div>
          <!--body-->
          <div class="relative p-6 flex-auto">
            <label
              for="nickname-text"
              class="block mb-2 font-s text-lb font-medium text-white"
              >Ваш ник</label
            >
            <input
              id="nickname-text"
              aria-describedby="nickname-text-explanation"
              class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              placeholder="CryptoGladi"
              bind:value={nicknameValue}
              on:input={checkNickname}
              on:keypress={(e) => {
                if (e.charCode == 32) e.preventDefault();
              }}
              maxlength="20"
            />
  
            {#if !nicknameIsTaken && nicknameValue !== ""}
              <p class="mt-2 text-sm text-green-600 dark:text-green-500">
                <strong>Ваш ник уникальный</strong>
              </p>
            {:else if nicknameIsTaken && nicknameValue !== ""}
              <p class="mt-2 text-sm text-red-600 dark:text-red-500">
                <strong>Ваш ник уже занятый</strong>
              </p>
            {:else}
              <p class="mt-2 text-sm text-white">
                <strong>Ваш ник должен быть уникальным</strong>
              </p>
            {/if}
          </div>
          <!--footer-->
          <div
            class="flex items-center justify-end p-6 border-t border-solid border-blueGray-200 rounded-b"
          >
            <button
              class="text-white bg-nord3 hover:bg-red-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
              on:click={toggleModal}
            >
              Закрыть
            </button>
            <button
              class="text-white bg-nord3 hover:bg-emerald-600 font-bold uppercase text-sm px-6 py-3 rounded shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150"
              on:click={registrationNewAccount}
            >
              Создать
            </button>
          </div>
        </div>
      </div>
    </div>
  {/if}
  
  <!--DROP ZONE FILE-->
  <FileDrop extensions={["txt"]} handleFiles={openAuthFile} let:files>
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
          <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <img
            class="fill-white w-9 h-9"
            src="github.svg"
            alt="Github репозиторий"
            on:click={openRepo}
          />
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
  