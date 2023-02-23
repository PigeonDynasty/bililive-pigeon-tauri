<script lang="ts">
  import Popover from '@/components/Popover.svelte'
  import Input from '@/components/Input.svelte'
  let value
  let className = ''
  let placeholder = '请选择'
  let filter: Function = undefined
  let valueKey = 'value'
  let labelKey = 'label'
  let autocomplete = false
  let data = []
  let dataMethod: Function = undefined

  let visible = false
  let inputValue = ''
  let inputPlaceholder = placeholder
  let ulEl
  $: labelText = i => ulEl.querySelectorAll('li')[i].textContent
  $: showData = () =>
    inputValue
      ? filter
        ? filter(inputValue, data)
        : data.filter((item, i) =>
            String(typeof item === 'object' ? labelText(i) : item)
              .toLowerCase()
              .includes(inputValue.toLowerCase())
          )
      : data
  const toggle = async bol => {
    if (!bol) return
    data = dataMethod ? await dataMethod() : data
    if (autocomplete) inputValue = value
    else {
      const i = data.findIndex(
        item => (typeof item === 'object' ? item[valueKey] : item) === value
      )
      if (i > -1) {
        inputValue = typeof data[i] === 'object' ? labelText(i) : data[i]
      }
    }
    if (showData().length === 0) visible = false
  }
  const click = (item, i) => {
    value = String(typeof item === 'object' ? item[valueKey] : item)
    inputValue = autocomplete ? value : labelText(i)
    visible = false
  }
  const input = () => {
    autocomplete && (value = inputValue)
    visible = showData().length > 0
  }
  const inputBlur = () => {
    if (autocomplete) return
    if (inputPlaceholder === placeholder) inputValue = ''
    else if (inputPlaceholder !== inputValue) inputValue = inputPlaceholder
  }
  const inputFocus = () => {
    inputPlaceholder = autocomplete ? placeholder : inputValue || placeholder
    inputValue = autocomplete ? inputValue : ''
    visible = showData().length > 0
  }
  const clear = () => (value = '')
  export {
    value,
    className as class,
    placeholder,
    filter,
    valueKey,
    labelKey,
    autocomplete,
    data,
    dataMethod
  }
</script>

<Popover
  bind:visible
  trigger="manual"
  class={className}
  popoverClass="min-w-[13rem]"
  on:toggle={e => toggle(e.detail)}
>
  <Input
    slot="trigger"
    bind:value={inputValue}
    placeholder={inputPlaceholder}
    on:input={() => input()}
    on:focus={() => inputFocus()}
    on:blur={() => inputBlur()}
    on:clear={() => clear()}
  />
  <ul class="max-h-36 text-sm -mx-2" bind:this={ulEl}>
    {#each showData() as item, i}
      <li
        class="px-2 py-1 cursor-pointer hover:bg-sky-50 hover:dark:bg-sky-900 hover:dark:bg-opacity-50 whitespace-nowrap"
        on:click={() => click(item, i)}
      >
        <slot {item}>
          {typeof item === 'object' ? item[labelKey] : item}
        </slot>
      </li>
    {/each}
  </ul>
</Popover>
