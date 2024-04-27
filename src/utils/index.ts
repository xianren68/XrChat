// verify email address.
export const verifyEmailAddress = (email: string) => {
	const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
	return emailRegex.test(email)
}

// verify phone
export const verifyPhone = (phone: string) => {
	const phoneRegex = /^1[3-9]\d{9}$/
	return phoneRegex.test(phone)
}
