package jsonrpc

func NewResultResponse(id any, result any) *Response {
	return &Response{
		JSONRPC: "2.0",
		Id:      id,
		Result:  result,
	}
}

func NewErrorResponse(id any, code int, message string) *Response {
	return &Response{
		JSONRPC: "2.0",
		Id:      id,
		Error: &Error{
			Code:    code,
			Message: message,
		},
	}
}
