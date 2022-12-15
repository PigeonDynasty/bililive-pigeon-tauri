import { writable } from 'svelte/store'

const rooms = writable([])
const addRoomId = (roomId: string | number) => {
  rooms.update(value => {
    return value.some(item => item.room_id === Number(roomId))
      ? value
      : [...value, { room_id: Number(roomId) }]
  })
}
const updateRoomInfo = info => {
  rooms.update(value => {
    const obj = value.find(item => item.room_id === info.room_id)
    if (obj) Object.assign(obj, info)
    return value
  })
}
const delByRoomId = (roomId: string | number) => {
  rooms.update(value => value.filter(item => item.room_id !== Number(roomId)))
}
export default rooms
export { addRoomId, updateRoomInfo, delByRoomId }
