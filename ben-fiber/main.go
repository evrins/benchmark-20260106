package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	// Define the root endpoint that returns simple JSON
	app.Get("/", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"msg":  "OK",
			"data": map[string]string{},
			"code": 0,
		})
	})

	// Start the server on port 8080
	log.Fatal(app.Listen(":8080"))
}
