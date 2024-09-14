namespace Color {
  interface RGB {
    r: number
    g: number
    b: number
    a?: number
  }
  interface HSV {
    h: number
    s: number
    v: number
  }
  interface Context {
    rgb: Writable<RGB>
    hsv: Writable<HSV>
    a: Writable<number>
  }

  interface SideColorOption extends Object {
    msg: string
    bg: string
    roomid: string
    time: string
    username: string
    giftGold: string
    giftSilver: string
    guard: string
    sc: string
  }
}
