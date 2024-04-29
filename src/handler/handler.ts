import { message } from '@/pb/message'
import type { Paylod } from '@/type'
type fn = (msg: message.Message) => any

export default class Handler {
	private _handlerMap: Map<number, fn>
	constructor() {
		this._handlerMap = new Map()
	}
	public addHandler(message_id: number, handler: fn) {
		this._handlerMap.set(message_id, handler)
	}
	public handle(paylod: Paylod) {
		// 这里可以进行一些处理
		const handler = this._handlerMap.get(paylod.message_id)
		if (handler) {
			return handler(message.Message.deserializeBinary(Uint8Array.from(paylod.data)))
		}
		return null
	}
}
