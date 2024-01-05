let db
let startIndex = 0 // 记录起始索引
const createDb = async dbName => {
  return new Promise((resolve, reject) => {
    const delRequest = indexedDB.deleteDatabase(dbName)
    delRequest.onsuccess = () => {
      const database = indexedDB.open(dbName)
      database.onsuccess = e => {
        const res = (e.target as IDBOpenDBRequest).result
        res.onversionchange = () => {
          resolve(res)
        }
      }
      database.onupgradeneeded = (e: IDBVersionChangeEvent) => {
        const res = (e.target as IDBOpenDBRequest).result
        res.createObjectStore(dbName, {
          keyPath: 'index',
          autoIncrement: true
        })
      }
      database.onerror = reject
    }
  })
}
const addTableData = async (tableName, data) => {
  return new Promise((resolve, _reject) => {
    const table = db.transaction(tableName, 'readwrite').objectStore(tableName)
    table.add(data)
    return resolve(true)
  })
}
const getCount = async (tableName): Promise<number> => {
  return new Promise((resolve, _reject) => {
    const table = db.transaction(tableName, 'readonly').objectStore(tableName)
    table.count().onsuccess = ({ target: { result } }) => {
      resolve(result)
    }
  })
}
const getTableData = async ({
  tableName,
  start,
  end
}): Promise<{ list: any[]; innerHeight: number }> => {
  startIndex = start
  return new Promise(async (resolve, _reject) => {
    const table = db.transaction(tableName, 'readonly').objectStore(tableName)
    const range = IDBKeyRange.bound(
      isNaN(start) ? 0 : start,
      (isNaN(end) ? 0 : end) + 1
    )
    const list = []
    let innerHeight = 0
    table.openCursor(range).onsuccess = ({ target: { result } }) => {
      if (!result) {
        return resolve({ list, innerHeight })
      }
      innerHeight += result.value.height

      list.push(result.value)
      result.continue()
    }
  })
}
const updateTableData = async (tableName, data) => {
  if (data.length === 0) return
  const table = db.transaction(tableName, 'readwrite').objectStore(tableName)
  data.forEach(item => {
    table.put(item)
  })
}
const getHeight = async (
  tableName: number,
  start?: number
): Promise<{ height: number; paddingTop: number }> => {
  return new Promise((resolve, _reject) => {
    const table = db.transaction(tableName, 'readonly').objectStore(tableName)
    table.getAll().onsuccess = ({ target: { result } }) => {
      let paddingTop = 0
      const height = result.reduce((prev, next, i) => {
        if (i === start) {
          paddingTop = prev
        }
        return prev + (next.height || 0)
      }, 0)
      resolve({ height, paddingTop })
    }
  })
}
const getStartIndex = async (
  tableName,
  top,
  viewNum
): Promise<{
  start: number
  height: number
  paddingTop: number
  length: number
}> => {
  return new Promise((resolve, _reject) => {
    const table = db.transaction(tableName, 'readonly').objectStore(tableName)
    table.getAll().onsuccess = ({ target: { result } }) => {
      let index = -1
      let pt = 0
      for (let i = 0; i < result.length; i++) {
        pt += result[i].height
        if (pt >= top) {
          index = i
          break
        }
      }
      const start =
        index !== -1
          ? Math.max(index - Math.ceil(viewNum / 2), 0)
          : Math.max(Math.ceil(result.length - viewNum), 0)
      let paddingTop = 0
      const height = result.reduce((prev, next, i) => {
        if (i === start) paddingTop = prev
        return prev + (next.height || 0)
      }, 0)
      resolve({
        start,
        height,
        paddingTop,
        length: result.length
      })
    }
  })
}
addEventListener('message', async e => {
  const { type, tableName, data } = e.data
  if (!db && tableName) {
    db = await createDb(tableName)
  }
  switch (type) {
    case 'close': {
      await db?.close()
      postMessage({ type: 'closed' })
      break
    }
    case 'add': {
      await addTableData(tableName, data)
      const { height } = await getHeight(tableName)
      postMessage({ type: 'added', height })
      break
    }
    case 'getLast': {
      const { viewNum } = data
      const length = await getCount(tableName)
      let start = length - viewNum
      start < 0 && (start = 0)
      const { list, innerHeight } = await getTableData({
        tableName,
        start,
        end: length
      })
      const { height, paddingTop } = await getHeight(tableName, start)
      postMessage({
        type: 'get',
        data: list,
        innerHeight,
        height,
        paddingTop,
        length
      })
      break
    }
    case 'update': {
      updateTableData(tableName, data)
      break
    }
    case 'getByTop': {
      const { top, viewNum } = data
      const { start, height, paddingTop, length } = await getStartIndex(
        tableName,
        top,
        viewNum
      )
      if (start === startIndex) return
      startIndex = start
      let end = start + viewNum
      end >= length && (end = length)
      const { list, innerHeight } = await getTableData({
        tableName,
        start,
        end
      })
      postMessage({
        type: 'get',
        data: list,
        innerHeight,
        height,
        paddingTop,
        length
      })
      break
    }
  }
})
