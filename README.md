# Soul Language

**Soul** is a **low-level, memory-safe programming language** designed for **performance**, **developer control**, and **compile-time optimization**. 
Inspired by **Rust’s borrow-checker** and **Go’s simplicity**, Soul gives developers the best of both worlds — fine-grained memory control with minimal boilerplate.

---

## ❔ Why Soul?

- **Memory Safety** via a flexable custom **borrow checker** inspired by rust
- **Stack-First Allocation** model
- **Flexible Compile-Time Evaluation**
- **Minimal Syntax**, inspired by Go
- Built-in **Ruleset System** for deterministic or functional coding styles
- **Literal Retention** for zero-cost optimizations
---

## Core Concepts

### 🔄 Borrow Checker

Soul features a **custom borrow-checking system** inspired by Rust’s, designed to prevent:

- **Dangling pointers**  
- **Double frees**  
- **Use-after-free bugs**

Unlike Rust, **Soul removes restrictions that aren’t strictly necessary for memory safety**. This means:

- You can have **multiple mutable references** (`&`) and **mutable and const references** (`&` and `@`) to the same object **at the same time**  
- **Memory safety is still guaranteed**, thanks to **lifetime tracking**
- **Aliasing is allowed**, but always **tracked and bounded** by the compiler's borrow-checker

This relaxed model gives you **more flexibility than Rust**, without compromising safety, making Soul ideal for **low-level, high-performance systems** where **strict aliasing rules would otherwise limit design choices**.

> ⚖️ Soul keeps **lifetimes and ownership**, but **loosens aliasing**, offering the **safety of Rust** with the **more freedom**.

### 📦 Stack-First Memory Model

Using the borrow checker, objects are allocated on the **stack by default**. Heap allocation is minimized unless explicitly required, improving cache locality and runtime performance.

### ✍️ Minimal Syntax

Soul syntax is clean and direct, keeping the developer in control with less ceremony.

## 📜 Rulesets

Soul functions and variables follow specific **Rulesets**:

| Ruleset   | Description |
|-----------|-------------|
| `Default` | Normal behavior — allows mutation, borrowing, and runtime evaluation. |
| `Const`   | Enforces **pure functional** style. No mutation, borrowing, or side effects. Can be compile-time or run-time. |
| `Literal` | Must be **entirely resolved at compile-time**. No runtime evaluation allowed. Great for constexpr-like performance. |

## ⚡ Literal Retention

Variables initialized with literals retain their **literal status** for optimization, even if declared as `var` or `const`.

```soul
mut a = 1  // optimized at compile time
const b = 5    // retains literal status
Literal c = 10 // retains literal status

constFunc(a)   // 'a' is replaced with Literal '1' so function is run in compileTime
a = 20         // loses literal status
constFunc(a)   // 'a' lost literal retention so will not run in runTime
```

## Language Reference
### 🔹 Function Scoping & Rulesets
```soul
func() {}             // default, runtime allowed
const func() {}       // no mutation, can be runtime or compile-time
Literal func() {}     // no mutation, compile-time only

access() {}           // private function
Access() {}           // public function

parent() {
    child(int a) {}
    child(1)
}

//!!Error: 'child' not in scope!!
// child(1)
```
### 🔹 Parameters & Return Types
```soul
func(int a) {}                  // parameter is const int
func(str a) {}                  // function overloading is allowed  
func(mut int[] a) {}            // mutable array parameter

func(1)                         // calls func(int)
func("foo")                     // calls func(str)
func([1,2,3])                   // calls func(int[])

//!!function overloading by return type is not allowed!!
// func(str a) int {} 

funcNone() {                    // no return type
    return
}

funcInt() int {                 // returns int
    return 1
}
```
### 🔹 Static Method
```soul
u8 maxValue() u8 { // the first 'u8' is the 'this' type and the second 'u8' the return type
    return 255
}

max := u8.maxValue()
```
### 🔹 Consume Method
a methode that takes ownership of the variable when called
```soul
int[] consumeToEl(mut this, int a) int[] {
    this = [0]
    this[0] = a
    return this
}

arr := [1,2,3]
newArr := arr.consumeToEl(1)

//!!Cannot use 'arr' anymore after consume unless autoCopy is enabled(like in number types)!!
// el := arr[1]
```
### 🔹 Const Reference Method
```soul
int constRefSum(this@, int a) int {
    //!!not allowed!!
    // *this +=1
    return *this + a
}

a := 1
mut res := a.constRefSum(1)

int& aMutRef = &a
res = aMutRef.constRefSum(1)

int@ aConstRef = @a
res = aConstRef.constRefSum(1)

b := 1
res = b.constRefSum(1)
```
### 🔹 Mutable Reference Method
```soul
int mutRefSum(this&, int a) int {
    *this += 1
    return *this
}

a := 1
mut res := a.mutRefSum(1)

int& aMutRef = &a
res = aMutRef.mutRefSum(1)

int@ aConstRef = @a
//!!Cannot use mutRefSum on const ref!!
// res = aConstRef.mutRefSum(1)

const b = 1
//!!Cannot call mutRefMethode on const var!!
// res = b.mutRefSum(1)
```
### 🔹 Typed Variable Declarations
```soul
int a = 1
const uint b = 1
Literal i32 c = 1

int e
e = 1

const int f
f = 1

//!!Not allowed!!
// Literal int g
// g = 1
```
### 🔹 Type Inference
#### Strict Inference
```soul
mut x = 1            // becomes int
const b = 1          // becomes int
Literal c = 1        // becomes int

// type casting is done with contructors
mut f := i32(1)     // becomes i32
e := uint(1)        // becomes uint
Literal g := f32(1) // becomes f32
```
#### Lazy Inference
```soul
let a
//...
a = 1               // becomes int

const b
//...
b = "hello"         // becomes str

//!!Not allowed!!
// Literal c
// c = 1

mut list := List[]
//...
list = list.Push(1)  // becomes List<int>
```
## 📦 Build & Compile
### ⚠️ CLI and compiler tooling coming soon.

Soul is designed to perform compile-time checks and literal propagation during build. Its goal is zero-cost abstraction, maximum safety, and performance-first programming.

## 💬 Final Thoughts
Soul is for developers who want:
- The safety of Rust
- The simplicity of Go
- The performance of C
- The power of compile-time logic

> ⚡ Write fast. Run faster. Stay safe.




