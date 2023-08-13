<script>
    import "../TailwindCSS.css";
    import 'material-icons/iconfont/material-icons.css';

    let isSearchOpen = false
</script>

<div class="avatar placeholder flex py-1 px-1">
    <div class="bg-nord10 text-white rounded-full w-20">
      <span class="text-3xl">G</span>
    </div>
    <p class="text-white self-end py-3 px-2">Вы: Моё после сообщение</p>
</div> 

<div class="bg-nord1 absolute inset-x-0 bottom-0 h-6 flex">
  <span on:click={()=>isSearchOpen = true} class="material-icons text-black bg-nord10 absolute">search</span>
</div>

<input type="checkbox" id="my-modal" class="modal-toggle" bind:checked={isSearchOpen}>

<div class="modal shadow-lg bg-transparent" on:click|self={()=>isSearchOpen = false}>
  <div class="modal-box">
    <label class="label">
      <span class="label-text">What is your name?</span>
    </label>
    <input type="text" placeholder="Type here" class="input input-bordered w-full max-w-xs" />
  </div>
</div>