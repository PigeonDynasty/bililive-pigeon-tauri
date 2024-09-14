import './gt'
import { invoke } from "@tauri-apps/api/tauri";

export const initCaptcha = async (webLocation: string): Promise<any> => {
  const res: any = await invoke("get_captcha", {
    webLocation
  })
  const data = res.data
  return new Promise((resolve, reject) => {
    globalThis.initGeetest({
      challenge: data.geetest.challenge,
      gt: data.geetest.gt,
      new_captcha: 1,
      offline: false,
      product: 'bind',
      width: '100%'
    }, (captchaObj) => {
      captchaObj.appendTo('#app');
      captchaObj.onReady(() => {
        captchaObj.verify()
      })
      captchaObj.onSuccess(() => {
        resolve({
          data,
          captcha: captchaObj.getValidate()
        })
      })
      captchaObj.onError(() => {
        reject()
      })
    })
  })
}

export const getWebLocation = (): Promise<string> => {
  return new Promise((resolve, reject) => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          const { latitude, longitude } = position.coords
          const res = Math.round(latitude * 10) + '.' + Math.round(longitude * 10)
          console.log('coords', res)
          resolve(res)
        },
        (error) => {
          console.error("Error occurred. Error code: " + error.code)
          resolve('321.1234')
        }
      );
    } else {
      console.log("Geolocation is not supported by this browser.")
      resolve('321.1234')
    }
  })
}