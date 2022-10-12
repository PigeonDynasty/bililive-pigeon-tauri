<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  let name = ''
  let greetMsg = ''

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke('greet', { name })
  }
</script>

<div class="row">
  <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
  <button on:click={greet}> Greet </button>
</div>
<p>{greetMsg}</p>

<style>
  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }
  @media (prefers-color-scheme: dark) {
    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
  }
</style>
