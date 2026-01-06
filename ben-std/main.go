package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

func main() {
	// Define the handler for the root endpoint
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		// Set the content type to JSON
		w.Header().Set("Content-Type", "application/json")

		// Create a response object
		response := map[string]any{
			"msg":  "OK",
			"code": 0,
			"data": map[string]string{},
		}

		// Encode the response as JSON and write it to the response writer
		if err := json.NewEncoder(w).Encode(response); err != nil {
			http.Error(w, "Failed to encode JSON response", http.StatusInternalServerError)
			return
		}
	})

	// Start the HTTP server on port 8080
	fmt.Println("Server starting on :8080")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		fmt.Printf("Server failed to start: %v\n", err)
	}
}
