export default function ripple() {
  const createRipple = event => {
    const e = event || window.event
    const button = e.srcElement || e.target
    if (button.tagName.toLowerCase() !== 'button') return
    const rippleSize = Math.max(button.clientWidth, button.clientHeight)
    const moved = rippleSize / 2
    const circle = document.createElement('span')
    const pos = button.getBoundingClientRect()
    circle.style.width = circle.style.height = `${rippleSize}px`
    circle.style.top = `${event.clientY - pos.top - moved}px`
    circle.style.left = `${event.clientX - pos.left - moved}px`
    circle.classList.add('ripple')
    const isExist = button.getElementsByClassName('ripple')[0]
    if (isExist) {
      isExist.remove()
    }
    button.append(circle)
  }
  document.body.addEventListener('mousedown', createRipple)
}
