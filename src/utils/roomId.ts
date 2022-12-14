import { writable } from 'svelte/store'
const roomIds = writable([])
const addRoomId = (roomId: string | number) => {
  roomIds.update(ids => {
    return roomId && !ids.includes(roomId) ? [...ids, roomId] : [...ids]
  })
}
const delRoomId = (roomId: string | number) => {
  roomIds.update(ids => ids.filter(id => id !== roomId))
}
export default roomIds
export { addRoomId, delRoomId }
