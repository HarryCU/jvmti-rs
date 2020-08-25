
# Run Example

## 1. build example-rs

```
cd example-rs
cargo build
```

## 2. build HelloWord.class
```
javac HelloWorld.java
```

## 3. run it
```
java -agentpath:<jvmti-rs build library> HelloWorld
```