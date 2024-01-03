interface TimedData<T> {
  startTime: number
  expires: number
  data: T
}
export function setLocal(
  name: string,
  data: any,
  pExpires = 1000 * 60 * 60 * 24 * 7 // 7 days
): void {
  const timed = {
    startTime: Date.now(),
    expires: pExpires,
    data
  }
  localStorage.setItem(name, JSON.stringify(timed))
}
export function getLocal<T>(name: string): TimedData<T> | null {
  const item = localStorage.getItem(name)
  if (!item) return null
  return JSON.parse(item)
}

export async function useLocal<T>(name: string): Promise<T> {
  return new Promise((resolve, reject) => {
    const local = getLocal<T>(name)
    if (local === null) reject(`Not set`)
    else if (local.startTime + local.expires < Date.now()) reject(`${name}已超过有效期`)
    else resolve(local.data)
  })
}
