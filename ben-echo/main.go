package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func main() {
	e := echo.New()

	// Simple JSON endpoint
	e.GET("/", func(c echo.Context) error {
		return c.JSON(http.StatusOK, map[string]any{
			"msg":  "OK",
			"code": 0,
			"data": map[string]string{},
		})
	})

	// Start the server
	e.Logger.Fatal(e.Start(":8080"))
}
