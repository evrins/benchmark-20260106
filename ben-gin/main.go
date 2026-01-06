package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	gin.SetMode(gin.ReleaseMode)
	gin.DisableConsoleColor()

	// Create a Gin router without logger middleware
	r := gin.New()
	r.Use(gin.Recovery())

	// Define a GET endpoint at root path "/"
	r.GET("/", func(c *gin.Context) {
		// Return a simple JSON response
		c.JSON(http.StatusOK, gin.H{
			"data": map[string]string{},
			"msg":  "Ok",
			"code": 0,
		})
	})

	// Start the server on port 8080
	r.Run(":8080")
}
