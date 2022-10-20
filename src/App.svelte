<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import Test from './lib/Test.svelte'
  import Danmaku from './lib/Danmaku.svelte'
  import { Tabs, Tab } from './lib/Tabs'
  import Setting from './lib/Setting.svelte'
</script>

<main class="flex h-full ">
  <div class="flex-1 pl-2 pr-1">
    <Tabs class="h-full">
      <Tab header="Info" key="info">
        <h1>Welcome to Tauri!</h1>
        <div class="row">
          <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
          </a>
          <a href="https://svelte.dev" target="_blank">
            <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
          </a>
        </div>

        <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>
      </Tab>
      <Tab header="Greet" key="greet">
        <Greet />
      </Tab>
      <Tab header="Test" key="test">
        <Test />
      </Tab>
      <Tab header="用户" key="user">用户1</Tab>
      <Tab header="插件" key="plugin">插件1</Tab>
      <Tab header="设置" key="setting">
        <Setting />
      </Tab>
    </Tabs>
  </div>
  <div class="flex-1 pl-1 pr-2">
    <Danmaku roomId={5050} />
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    /* color: #0f0f0f;
    background-color: #f6f6f6; */

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    a:hover {
      color: #24c8db;
    }
  }

  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
