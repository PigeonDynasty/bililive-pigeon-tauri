declare global {
    function initGeetest(param: any, callback: (captchaObj: any) => void): void
    interface Object {
        [key: string | number]: any;
    }
}
export { }