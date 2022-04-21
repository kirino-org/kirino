package auth

import "golang.org/x/crypto/bcrypt"

type User struct {
	Username string
	Password string

	username     string
	passwordHash string
}

func NewUser(username, password string) User {
	pwHash, err := bcrypt.GenerateFromPassword(
		[]byte(password),
		BcryptCost,
	)
	if err != nil {
		panic(err)
	}

	return User{
		username:     username,
		passwordHash: string(pwHash),
	}
}

func CheckPassword(u User, password string) (matches bool) {
	switch err := bcrypt.CompareHashAndPassword(
		[]byte(u.passwordHash),
		[]byte(password),
	); err {
	case bcrypt.ErrMismatchedHashAndPassword:
		matches = false
		return
	case nil:
		matches = true
		return
	default:
		panic(err)
	}
}
