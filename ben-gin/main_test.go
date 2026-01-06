package main

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
)

func TestRootEndpoint(t *testing.T) {
	// Set Gin to test mode
	gin.SetMode(gin.TestMode)

	// Create a new router
	r := gin.New()
	r.GET("/", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "Hello, Gin Framework!",
			"status":  "success",
		})
	})

	// Create a request
	req, _ := http.NewRequest("GET", "/", nil)
	w := httptest.NewRecorder()

	// Perform the request
	r.ServeHTTP(w, req)

	// Assert the status code
	assert.Equal(t, http.StatusOK, w.Code)

	// Assert the response body
	expected := `{"message":"Hello, Gin Framework!","status":"success"}`
	assert.JSONEq(t, expected, w.Body.String())
}
