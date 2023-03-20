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
}
