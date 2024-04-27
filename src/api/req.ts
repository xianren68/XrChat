import { Message } from 'google-protobuf'

type interceptors = 'request' | 'response'

type Interceptor = <T extends RequestInit | Response>(config: T) => T
export default class Request {
	private readonly baseUrl: string
	private interceptors: {
		request?: Interceptor[]
		response?: Interceptor[]
	}

	public constructor(baseUrl: string) {
		this.baseUrl = baseUrl
		this.interceptors = {}
	}

	async Post<T extends Message>(url: string, data: T) {
		const config: RequestInit = {
			method: 'POST',
			headers: new Headers(),
			body: data.serializeBinary(),
		}
		// exec request interceptors.
		// async is not implement.
		if (this.interceptors['request']) {
			for (let fn of this.interceptors['request']) {
				fn(config)
			}
		}
		const res = await fetch(this.baseUrl + url, config)
		// exec response interceptors.
		// async is not implement.
		if (this.interceptors['response']) {
			for (let fn of this.interceptors['response']) {
				fn(res)
			}
		}
		return await res.arrayBuffer()
	}

	// group request.
	Group(url: string) {
		const req = new Request(this.baseUrl + url)
		// inherit interceptors.
		if (this.interceptors['request']) {
			req.setInterceptors('request', ...this.interceptors['request'])
		}
		if (this.interceptors['response']) {
			req.setInterceptors('response', ...this.interceptors['response'])
		}
		return req
	}

	// set interceptors.
	setInterceptors(type: interceptors, ...fns: Interceptor[]) {
		if (this.interceptors[type]) {
			this.interceptors[type]!.push(...fns)
		} else {
			this.interceptors[type] = [...fns]
		}
	}
}
