export default function ripple() {
  const createRipple = event => {
    const button = event.currentTarget
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
  document.querySelectorAll('button').forEach(btn => {
    btn.addEventListener('mousedown', createRipple)
  })
}
