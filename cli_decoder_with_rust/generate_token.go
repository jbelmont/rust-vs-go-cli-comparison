package main

import (
	"fmt"

	"github.com/dgrijalva/jwt-go"
)

func main() {
	// Build JWT token.
	signingKey := "secret"
	claims := jwt.MapClaims{
		"sub": "Luke Cage",
		"iss": "John Rambo",
		"aud": "Badasses",
		"exp": 1300819380,
		"nbf": 1300819381,
		"iat": 1300819382,
		"jti": "oneRandomeString123456",
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenStr, _ := token.SignedString([]byte(signingKey))
	fmt.Print(tokenStr)
}
