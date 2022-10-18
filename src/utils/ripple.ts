export default function ripple() {
  const createRipple = event => {
    const e = event || window.event
    const button = e.srcElement || e.target
    if (button.tagName.toLowerCase() !== 'button') return
    const isExited = button.querySelector('.ripple')
    if (isExited) isExited.remove()

    const rippleSize = Math.max(button.clientWidth, button.clientHeight)
    const moved = rippleSize / 2
    const pos = button.getBoundingClientRect()
    const circle = document.createElement('span')
    circle.style.width = circle.style.height = `${rippleSize}px`
    circle.style.top = `${event.clientY - pos.top - moved}px`
    circle.style.left = `${event.clientX - pos.left - moved}px`
    circle.classList.add('ripple')
    button.append(circle)
    // 动画结束移除dom
    setTimeout(() => {
      circle.remove()
    }, 600)
  }
  document.body.addEventListener('mousedown', createRipple)
}
