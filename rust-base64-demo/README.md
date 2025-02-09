# Rust-base64-demo

## Explain this code

这段代码使用了base64库来进行编码和解码操作。首先，代码定义了一个字符串变量 `original` ，值为"Hello?_d"。然后，使用 `general_purpose::STANDARD.encode(original)` 对该字符串进行Base64编码，并将结果保存在变量 `b64` 中。接下来，使用 `println!` 函数将编码结果打印出来。

然后，代码使用 `general_purpose::STANDARD.decode(b64)?` 对编码结果进行Base64解码，并将解码结果保存在变量 `decoded` 中。随后，使用 `String::from_utf8(decoded)?` 将解码结果转换为字符串，并将结果保存在变量 `decoded` 中。再次使用 `println!` 函数将解码结果打印出来。

接下来，代码使用 `general_purpose::URL_SAFE_NO_PAD.encode(original)` 对原始字符串进行Base64 URL编码，并将结果保存在变量 `b64u` 中。然后，使用 `println!` 函数将URL编码结果打印出来。

最后，代码使用 `general_purpose::URL_SAFE_NO_PAD.decode(b64u)?` 对URL编码结果进行解码，并将解码结果保存在变量 `decoded` 中。再次使用 `String::from_utf8(decoded)?` 将解码结果转换为字符串，并将结果保存在变量 `decoded` 中。最后一次使用 `println!` 函数将URL解码结果打印出来。

整体而言，这段代码展示了如何使用base64库进行Base64编码和解码操作，并分别展示了标准编码和URL编码的结果。

## 实操

```shell
cargo watch -q -c -x run
```
