package jsonrpc

import (
	"encoding/json"
	"fmt"
	"os"
	"path"
	"time"
)

func log(msg string, args ...any) {
	// Append to a log file
	dest, _ := os.UserHomeDir()
	if dest == "" {
		dest = "/tmp"
	}
	f, err := os.OpenFile(path.Join(dest, "/touch-mcp-server.log"), os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		fmt.Println("failed to open log file:", err)
		return
	}
	defer f.Close()

	content := fmt.Sprintf(time.Now().Format(time.RFC3339)+" "+msg+"\n", args...)
	fmt.Print(content)
	if _, err := f.WriteString(content); err != nil {
		fmt.Println("failed to write to log file:", err)
		return
	}
}

func Log(msg string, args ...any) {
	log(msg, args...)
}

func LogRequest(req Request) {
	log(">> Request:")
	if v, err := json.MarshalIndent(req, "", "  "); err != nil {
		log("failed to marshal request: %v", err)
	} else {
		log(string(v))
	}
}

func LogResponse(resp *Response) {
	log(">> Response:")
	if v, err := json.MarshalIndent(resp, "", "  "); err != nil {
		log("failed to marshal response: %v", err)
	} else {
		log(string(v))
	}
}
