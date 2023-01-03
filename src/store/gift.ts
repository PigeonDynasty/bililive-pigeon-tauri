import { writable } from 'svelte/store'
const gifts = writable([])
const addGiftContent = (roomId: string | number) => {
  gifts.update(value => {
    return value.some(item => item.id === Number(roomId))
      ? value
      : [...value, { id: Number(roomId), uname: '', data: [] }]
  })
}
const delGiftContent = (roomId: string | number) => {
  gifts.update(value => value.filter(item => item.id !== Number(roomId)))
}
const updateGiftContent = info => {
  gifts.update(value => {
    const obj = value.find(item => item.id === info.room_id)
    if (obj) obj.uname = info.uname
    return value
  })
}
const addGift = (
  roomId: string | number,
  uid: string,
  uname: string,
  giftName: string,
  num: number,
  coin_type: string
) => {
  gifts.update(value => {
    const obj = value.find(item => item.id === Number(roomId))
    if (obj) {
      const gift = obj.data.find(
        item =>
          item.uid === uid &&
          item.giftName === giftName &&
          item.coin_type === coin_type
      )
      if (gift) {
        gift.uname = uname
        gift.num += num
      } else {
        obj.data.push({
          uid: uid,
          uname: uname,
          giftName: giftName,
          num: num,
          coin_type: coin_type
        })
      }
    }
    return value
  })
}
export default gifts
export { addGiftContent, delGiftContent, updateGiftContent, addGift }
