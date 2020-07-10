
+ Constants types must also always be annotated.
+ Return value：They are not the same. There are two solutions:
    1. Add a `return` ahead of `num * num;`
    2. remove `;`, make it to be `num * num`
+ "  " means &str while '  ' means character
+ slice： 
    ```rust
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice)
    }
    ```
+ String：
    - &str -> String
        ```rust
        let s = "hello world!"; // variable s: &str
        let s1 = String::from(s); // variable s1: String
        let s1 = s.to_string();
        let s1 = s.to_owned();
        ```      
    - String -> &str
        ```rust
        let s = String::from("hello world!"); // variable s: String
        let s1 = s.as_str(); // variable s1: &str
        let s1 = &s[..]; // 相当于：&s[0..s.len()];
        ```
    - 
        ```rust
        string_slice(&"blue");
        string("red".to_string());
        string(String::from("hi"));
        string("rust is fun!".to_owned());
        string("nice weather".into());
        string(format!("Interpolation {}", "Station"));
        string_slice(&String::from("abc")[0..1]);
        string_slice("  hello there ".trim());
        string("Happy Monday!".to_string().replace("Mon", "Tues"));
        string("mY sHiFt KeY iS sTiCkY".to_lowercase());
        ```
+ macro :宏的参数使用一个美元符号 $ 作为前缀，并使用一个指示符（designator）来 注明类型：
https://blog.csdn.net/teamlet/article/details/50989186

+ [Machine epsilon](https://en.wikipedia.org/wiki/Machine_epsilon)
```rust
    if (x - y).abs() < f64::EPSILON{ //https://en.wikipedia.org/wiki/Machine_epsilon
        println!("Success!");
    }
```

+  Add the AsRef trait appropriately as a trait bound
```rust
    fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
        arg.as_ref().as_bytes().len()
    }
    // Obtain the number of characters (not bytes) in the given argument
    // Add the AsRef trait appropriately as a trait bound
    fn char_counter<T:AsRef<str>>(arg: T) -> usize {
        arg.as_ref().chars().count()
    }
```
