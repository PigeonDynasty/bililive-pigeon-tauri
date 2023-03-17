export default (node, to = 'body') => {
  const target = document.querySelector(to)
  target && target.appendChild(node)
  return {}
}
