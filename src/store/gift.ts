import { writable } from 'svelte/store'
const gifts = writable([])
const addGiftContent = (roomId: string | number) => {
  gifts.update(value => {
    return value.some(item => item.id === Number(roomId))
      ? value
      : [...value, { id: Number(roomId), data: [] }]
  })
}
const delGiftContent = (roomId: string | number) => {
  gifts.update(value => value.filter(item => item.id !== Number(roomId)))
}
const addGift = (
  roomId: string | number,
  uid: string,
  uname: string,
  giftName: string,
  num: string | number
) => {}
export default gifts
export { addGiftContent, delGiftContent }
