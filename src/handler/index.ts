import Handler from './handler'
import { message } from '@/pb/message'
import { sessionStore } from '@/store'
const useSessionStore = sessionStore()
const handler = new Handler()

// add handler
handler.addHandler(3, (msg: message.Message) => {
	const index = useSessionStore.sessionList.findIndex((item) => item.id === msg.src && item.session_type === 3)
	// new session.
	if (index === -1) {
		//TODO  get user info by database or store.
		useSessionStore.sessionList.push({
			id: msg.src,
			session_type: 3,
			last: msg.msg,
			last_time: msg.time,
			un_read: 1,
			avatar,
			name,
			remark,
		})
	}
})
