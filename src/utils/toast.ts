const toast = (str: string, color = 'indigo', duration = 2000) => {
  const toastEl = document.createElement('div')
  toastEl.id = Symbol().toString()
  toastEl.className = `toast fixed top-3 left-1/2 -translate-x-1/2 text-${color}-400 bg-${color}-100 dark:bg-${color}-900 opacity-0 transition-opacity duration-1000 py-2 px-4 rounded-md shadow-md`
  toastEl.innerHTML = str
  document.body.appendChild(toastEl)
  setTimeout(() => {
    toastEl.classList.add('opacity-100')
    setTimeout(() => {
      toastEl.classList.remove('opacity-100')
      toastEl.classList.add('opacity-0')
      setTimeout(() => {
        document.body.removeChild(toastEl)
      }, 1000)
    }, duration)
  })
}
export default toast
