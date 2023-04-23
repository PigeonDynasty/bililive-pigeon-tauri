interface DbEmoji {
  _id: number
  emoji: string
  emoticon_id: number
  emoticon_unique: string
  url: string
  timestamp: number
}

interface DbSetting {
  _id: number
  room_id: number
  config: string
  timestamp: number
}
