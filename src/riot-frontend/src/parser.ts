// 你可以类比地自定义一些数据解析方式:)
export interface Line {
  x: string
  y: string
  title: string
}
export interface Map {
  latitude: string
  longitude: string
  order: string
  title: string
}
export interface Parser {
  name: string
  type: 'raw' | 'json'
  parser: Function | null
  lines?: Line[]
  maps?: Map[]
}
export const parsers: Parser[] = [
  { name: 'default', type: 'raw', parser: (raw: number[]) => ({ payload: raw.toString() }) },
  {
    name: 'BS_assginment',
    type: 'json',
    parser: null,
    lines: [{ title: 'x-y', x: 'x', y: 'y' }],
    maps: [{ title: 'x-y', latitude: 'x', longitude: 'y', order: 'timestamp' }]
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
function try2json(raw: number[]) {
  console.log(raw)
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
