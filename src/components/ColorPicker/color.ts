/**
 * @description: 检查是否是一个合规的 hex/rgb/rgba 字符串
 * @param {string} str
 * @return {*}
 */
export function validColor(str: string): string {
  let reg, type
  if (/^rgb\(/.test(str)) {
    //如果是rgb开头，200-249，250-255，0-199
    type = 'rgb'
    reg =
      /^[rR][gG][Bb][(]([\s]*(2[0-4][0-9]|25[0-5]|[01]?[0-9][0-9]?)[\s]*,){2}[\s]*(2[0-4]\d|25[0-5]|[01]?\d\d?)[\s]*[)]{1}$/
  } else if (/^rgba\(/.test(str)) {
    //如果是rgba开头，判断0-255:200-249，250-255，0-199 判断0-1：0 1 1.0 0.0-0.9 .0-.9
    type = 'rgb'
    reg =
      /^[rR][gG][Bb][Aa][(]([\s]*(2[0-4][0-9]|25[0-5]|[01]?[0-9][0-9]?)[\s]*,){3}[\s]*(1|1.0|1.00|0|0.0|0.00|0.[0-9]?[0-9]|.[0-9]?[0-9])[\s]*[)]{1}$/
  } else if (/^#/.test(str)) {
    type = 'hex'
    //六位或者三位
    reg = /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|^[0-9a-fA-F]{8})$/
  } else {
    return ''
  }
  return reg.test(str) ? type : ''
}
export function formatColor(str: string, alpha: Boolean): string {
  const type = validColor(str)
  switch (type) {
    case 'rgb':
      return alpha
        ? rgb2rgbString(rgbString2rgb(str))
        : rgb2hex(rgbString2rgb(str))
    case 'hex':
      return alpha ? rgb2rgbString(hex2rgb(str)) : str
    default:
      return ''
  }
}
export function toHsv(str: string): Color.HSV {
  return rgb2hsv(toRgb(str))
}
export function toRgb(str): Color.RGB {
  const type = validColor(str)
  switch (type) {
    case 'rgb':
      return rgbString2rgb(str)
    case 'hex':
      return hex2rgb(str)
    default:
      return {
        r: 255,
        g: 255,
        b: 255,
        a: 1
      }
  }
}
export function hex2rgb(hex: string): Color.RGB {
  hex = hex.replace('#', '')
  const l = Math.floor(hex.length / 3) // 需要取几位
  let r = hex.slice(0 * l, l)
  let g = hex.slice(1 * l, 2 * l)
  let b = hex.slice(2 * l, 3 * l)
  r = l === 1 ? r + r : r
  g = l === 1 ? g + g : g
  b = l === 1 ? b + b : b
  return {
    r: parseInt(r, 16),
    g: parseInt(g, 16),
    b: parseInt(b, 16),
    a: hex.length === 8 ? parseInt(hex.slice(6), 16) / 255 : 1
  }
}
export function rgb2hex({ r, g, b }: Color.RGB): string {
  const res = [r.toString(16), g.toString(16), b.toString(16)].map(str =>
    str.length === 1 ? '0' + str : str
  )
  return `#${res[0]}${res[1]}${res[2]}`
}
export function rgbString2rgb(rgb: string): Color.RGB {
  let reg = /\((.+?)\)/g
  const keys: string[] = rgb.match(reg)
  let res = [0, 0, 0, 1]
  keys.forEach(key => {
    res = key
      .replace(/\(|\)/g, '')
      .split(',')
      .map(val => Number(val))
  })
  return { r: res[0], g: res[1], b: res[2], a: res[3] }
}
export function rgb2rgbString({ r, g, b, a }: Color.RGB): string {
  return `rgba(${r},${g},${b},${typeof a === 'number' ? a : 1})`
}

/**
 * @description: 参考 https://blog.csdn.net/weixin_38616850/article/details/107955393
 * @param {Color} r,g,b ;输入[0,255]
 * @return {*} h,s,v ;全都返回百分比[0,100]，包括hue
 */
export function rgb2hsv({ r, g, b }: Color.RGB): Color.HSV {
  r = r / 255
  g = g / 255
  b = b / 255
  const min = Math.min(r, g, b)
  const max = Math.max(r, g, b)
  const difference = max - min
  let h
  if (max === min) {
    h = 0
  } else {
    switch (max) {
      case r:
        h = (g - b) / difference + (g < b ? 6 : 0)
        break
      case g:
        h = (b - r) / difference + 2
        break
      case b:
        h = (r - g) / difference + 4
        break
    }
  }
  return {
    h: (h / 6) * 100,
    s: (max === 0 ? 0 : difference / max) * 100,
    v: max * 100
  }
}
/**
 * @description: rgb2hsv 反向
 * @param {Color}
 * @return {*}
 */
export function hsv2rgb({ h, s, v }: Color.HSV): Color.RGB {
  h = (h / 100) * 6
  s = s / 100
  v = v / 100
  const i = Math.floor(h)
  const f = h - i
  const p = v * (1 - s)
  const q = v * (1 - f * s)
  const t = v * (1 - (1 - f) * s)
  const mod = i % 6
  const r = [v, q, p, p, t, v][mod]
  const g = [t, v, v, q, p, p][mod]
  const b = [p, p, t, v, v, q][mod]
  return {
    r: Math.round(r * 255),
    g: Math.round(g * 255),
    b: Math.round(b * 255)
  }
}
