# asciis
Rust-lang: ASCII base on RFC20. Just ord() and chr() 

基于 ASCII 规范文档 RFC20 编写的库。这个库非常简单易用，因为只有 ord() 和 chr() 这两个方法

# Usage 使用方法

**ord**

Given a string representing one Unicode character, return an integer representing the Unicode code point of that character. 

For example, ord('a') returns the integer Some(97) and ord("s") returns Some(115). 

This is the inverse of chr().


**chr**

Return the string representing a character whose Unicode code point is the integer i. 

For example, chr(97) returns the Some(), while chr(115) returns the String Some("s"). 

This is the inverse of ord().

  
## ord()
&str to Some(i32), "s" -> Some(115)

Example 示例:

```
use asciis::asc::Asciis;
let asc = Asciis{};
let r = asc.ord("s");
assert_eq!(r, Some(115));
```

## chr()
i32 to Some(String), 97 -> Some("a")
Example 示例:

```
use asciis::asc::Asciis;
let asc = Asciis{};
let v = asc.chr(97);
assert_eq!(v, Some(String::from("a")));
```

# FAQ
Q:Why the type of return value is Option, not i32 or String?

为什么返回值的类型是 Option ，而不是 i32 或者 String ？

```
A: The ascii value range is [0~127], which can cause panic! if exceeded. So Option better than i32 or String.

ASCII 码的范围是 [0~127]，超出范围就会引发 panic!。所以 Option 比 i32 或者 String 更合适。
```

Q: Rust std has AsciiExt,why write this crate?

Rust 标准库 std 中已经有了 AsciiExt，为什么你还要写这个库？

```
A: Python's simple style influenced me. ord() and chr() are very simple to use.

Python 简约的风格影响着我，我觉得 ord() 和 chr() 这种方式更好用。
```

Q: Where does the standard code come from?

这些标准代码从哪里来？

```
A: From the RFC20 https://tools.ietf.org/html/rfc20#section-2

详见 RFC20 文档的第 2 部分介绍 https://tools.ietf.org/html/rfc20#section-2
```

Q: How do I determine what value the ord () method should pass in?

我如何确定 ord() 方法应该传入什么值？

```
A：See https://tools.ietf.org/html/rfc20#section-2

跟上面一样，详见 RFC20 文档的第 2 部分介绍，这是 ASCII 规范中约定的值
```
