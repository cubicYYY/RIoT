// 你可以类比地自定义一些数据解析方式:)
export const parsers: Parser[] = [
  { name: 'default', type: 'raw', parser: (raw: number[]) => ({ payload: raw.toString() }) },
  {
    name: 'BS_assginment',
    type: 'json',
    parser: null,
    lines: [
      { title: 'value取值', x: 'timestamp', y: 'value' },
      { title: '警报？', x: 'timestamp', y: 'alert' }
    ],
    alert: 'alert',
    maps: [{ title: '轨迹与数据', latitude: 'lat', longitude: 'lng', order: 'timestamp' }]
  },
  {
    name: 'DHT22',
    type: 'raw',
    parser: (raw: number[]) => {
      console.log(raw)
      return { temperature: new DataView(Uint8Array.from(raw).buffer).getFloat64(0, true) }
    }
  }
]

export interface RiotLine {
  x: string
  y: string
  title: string
}
export interface RiotMap {
  latitude: string
  longitude: string
  order: string
  title: string
}
export interface Parser {
  name: string
  type: 'raw' | 'json'
  parser: Function | null
  alert?: string // 警报字段
  lines?: RiotLine[]
  maps?: RiotMap[] // 目前只支持一个
}

function try2json(raw: number[]) {
  try {
    const textDecoder = new TextDecoder('utf-8')
    const resultString = textDecoder.decode(Uint8Array.from(raw).buffer)
    return JSON.parse(resultString)
  } catch (e) {
    console.log(e)
  }
  return false
}
export function parse(raw: number[], parser_id: number): Object {
  const parser = parsers[parser_id]
  if (parser.type === 'json') return try2json(raw)
  if (parser.parser !== null) return parser.parser(raw)
  return {}
}
