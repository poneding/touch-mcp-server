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
	log(">> Request: [%v] %s", req.Id, req.Method)
	if req.Params != nil {
		if v, err := json.MarshalIndent(req.Params, "", "  "); err != nil {
			log("failed to marshal request params: %v\n", err)
		} else {
			log("params: \n%s\n", string(v))
		}
	}
}

func LogResponse(resp *Response) {
	if resp.Result != nil {
		log(">> Response: [%v] %s", resp.Id, "Succeed")
		if v, err := json.MarshalIndent(resp.Result, "", "  "); err != nil {
			log("failed to marshal request params: %v", err)
		} else {
			log("Result: \n%s", string(v))
		}
	} else {
		log("Response: [%v] %s", resp.Id, "Failed")
		if v, err := json.MarshalIndent(resp.Error, "", "  "); err != nil {
			log("failed to marshal request params: %v", err)
		} else {
			log("Error: \n%s", string(v))
		}
	}
}
