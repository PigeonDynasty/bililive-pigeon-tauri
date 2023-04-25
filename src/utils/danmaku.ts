export function formatInteractTypeName(type: number): string {
  switch (type) {
    case 1:
      return '进入'
    case 2:
      return '关注'
    case 3:
      return '分享'
    case 4:
      return '特别关注'
    case 5:
      return '互相关注'
  }
}

export function formatGuardName(level: number): string {
  switch (level) {
    case 3:
      return '舰长'
    case 2:
      return '提督'
    case 1:
      return '总督'
    default:
      return ''
  }
}
