export function html2text(html_str: string): string {
  return _htmlReplaceImg(html_str).replace(/<[^<>]+>/g, '')
}
function _htmlReplaceImg(html_str: string): string {
  const img_reg = /<img(.+?)\/>/g
  const imgs = html_str.match(img_reg)
  if (!imgs) return html_str
  imgs.forEach(img => {
    const alt = img.match(/alt=(.+?)"/g)
    if (!alt) return
    html_str = html_str.replace(
      img,
      alt[0].replace('alt="', '').replace('"', '')
    )
  })
  return html_str
}

export function setCookieToCookieStr(str: string, cookieMap?: Map<string, string>): string {
  if (!cookieMap) cookieMap = new Map()
  const cookies = str.split('&&')
  cookies.forEach((item: string) => {
    const parts = item.split(';')
    const [name, value] = parts[0].split('=')
    cookieMap.set(name, value)
  })
  return Array.from(cookieMap.entries()).map(([key, val]) => `${key}=${val}`).join(';')
}