package jsonrpc

import (
	"os"
	"testing"
)

func TestHome(t *testing.T) {
	t.Log(os.UserHomeDir())
}
