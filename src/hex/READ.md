# hex功能
hex库提供了一些便捷的方法用来编码十六进制数据

## decode和decode_to_slice
decode将16进制数据解码成字节并返回，比如下面就将"Hello world!"的16进制模式转化为"Hello world!"的字节表示

```rust
assert_eq!(
    hex::decode("48656c6c6f20776f726c6421"),
    Ok("Hello world!".to_owned().into_bytes())
);
```
decode_to_slice将16进制数据解码并赋值给一个buffer

```rust
let mut bytes = [0u8; 4];
assert_eq!(hex::decode_to_slice("6b697769", &mut bytes as &mut [u8]), Ok(()));
assert_eq!(&bytes, b"kiwi");
```

## encode, encode_to_slice和encode_upper
encode可以将输入数据编码返回一个16进制的字符串

`assert_eq!(hex::encode("Hello world!"), "48656c6c6f20776f726c6421");`\
encode_to_slice则把数据人数据编码并赋值给一个buffer

```rust
let mut bytes = [0u8; 4 * 2];
hex::encode_to_slice(b"kiwi", &mut bytes)?;
assert_eq!(&bytes, b"6b697769");
```
encode_upper和encode相同，只是返回的16进制字符串中的字母都变成了大写

`assert_eq!(hex::encode_upper("Hello world!"), "48656C6C6F20776F726C6421");`\

## serde支持
打开feature="serde"时，将对字节数据使用hex编码解码


```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    #[serde(with = "hex")]
    bar: Vec<u8>,
}
```