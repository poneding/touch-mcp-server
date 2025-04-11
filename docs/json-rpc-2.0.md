# JSON-RPC 2.0

JSON-RPC是一个无状态且轻量级的远程过程调用(RPC)协议。 本规范主要定义了一些数据结构及其相关的处理规则。它允许运行在基于socket,http等诸多不同消息传输环境的同一进程中。

## 请求对象

客户端发送一个调用请求到服务端，请求对象是一个JSON对象，包含以下成员：

- `jsonrpc`：字符串，必须是"2.0"。
- `method`：字符串，必须是一个有效的方法名。
- `params`：调用方法所需要的基本类型或结构化类型的参数值，该成员参数可以被省略。
- `id`：已建立客户端的唯一标识id，值必须包含一个字符串、数值或NULL空值。如果该成员被省略，则该请求被视为一个通知请求。

示例：

```json
{
  "jsonrpc": "2.0",
  "method": "users/create",
  "params": {
    "name": "poneding",
    "age": 18
  },
  "id": 1
}
```

## 响应对象

对于 RPC 的调用请求，除通知外，服务端都必须回复响应。响应对象是一个 JSON 对象，包含以下成员：

- `jsonrpc`：字符串，必须是"2.0"。
- `result`：调用成功时返回的结果值，值可以是基本类型或结构化类型。调用成功时必须返回该成员，失败时必须省略该成员。
- `error`：调用失败时返回的错误对象，包含以下成员（调用失败时必须返回该成员，成功时必须省略该成员）：
  - `code`：数值，错误码。
  - `message`：字符串，错误信息。
  - `data`：可选，包含额外的错误信息。
- `id`：已建立客户端的唯一标识id，值必须包含一个字符串、数值或NULL空值，必须与请求中的id一致。

> 提炼：
>
> 响应对象中 `result` 和 `error` 只能存在且必须存在一个，不能同时存在。

`error.code` 枚举：

- `-32700`：Parse error，解析错误，JSON 解析错误。
- `-32600`：Invalid Request，无效请求，请求对象规范不符合。
- `-32601`：Method not found，方法不存在，请求的方法不存在。
- `-32602`：Invalid params，无效参数，`params` 校验错误。
- `-32603`：Internal error，内部错误，服务端发生错误。
- `-32000 ~ -32099`：Server error，服务端错误，预留自定义的服务端错误。

## 批量请求与响应

请求对象是一个数组，包含多个请求对象。响应对象是一个数组，包含多个响应对象，id对应的请求对象的响应。

请求示例：

```json
[
  {
    "jsonrpc": "2.0",
    "method": "users/create",
    "params": {
      "name": "poneding",
      "age": 18
    },
    "id": 1
  },
  {
    "jsonrpc": "2.0",
    "method": "users/get",
    "params": {
      "id": 1
    },
    "id": 2
  }
]
```

响应示例：

```json
[
  {
    "jsonrpc": "2.0",
    "result": {
      "id": 1,
      "name": "poneding",
      "age": 18
    },
    "id": 1
  },
  {
    "jsonrpc": "2.0",
    "error": {
      "code": -32601,
      "message": "Method not found"
    },
    "id": 2
  }
]
```

## 参考

- [JSON-RPC 2.0 规范](https://wiki.geekdream.com/Specification/json-rpc_2.0.html)
