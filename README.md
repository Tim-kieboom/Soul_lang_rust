

**Soul** is a **low-level programming language**, focusing on performance, memory safety, while remaining as simple as possible. Inspired by the power of **Rust's borrow checker** and the minimalism of **Go's syntax**, Soul offers a unique balance between control, safety and developer ergonomics.

At its core, **Soul** introduces a **borrow-checking** mechanism aimed at preventing common memory safety issues, such as memory leaks. While influenced by Rust, Soul adopts **looser and more flexible borrowing rules** by **decreasing race condition safety**, giving programmers more freedom while still catching many critical errors at compile time.

**Soul** also prioritizes **stack allocation** whenever possible, reducing the performance cost of the program by reducing heap allocations. With the help of the **borrow-checker** and **dynamic stack allocation**, **Soul** can reduce the vast majority of heap allocations.

A distinct feature of **Soul** is its **ruleset system**, which allows developers to define functions with specific compile-time or functional constraints:

- **Default Ruleset**: The default mode for functions and variables. **Default** allows unrestricted behavior, including mutation, borrowing, and runtime evaluation. It is the most flexible and imperative-friendly mode, suited for general-purpose programming and low-level system code.

- **Literal Rulesets**: Inspired by **C++'s `constexpr`**, **Literal Functions** in Soul are part of the **Literal Ruleset**. These functions must be fully resolved at compile time, enforced by Soul's **stricter static analysis** and **simplified syntax**. This guarantees that computations within **Literal Functions** will never occur at runtime, boosting efficiency and ensuring deterministic behavior.
    
- **Const Rulesets**: For developers requiring a purely functional approach, **Soul** offers **Const Functions** under the **Const Ruleset**. These functions enforce a **Haskell-like functional style**, forbidding side effects, mutation, and borrowing. **Const Functions** are ideal for deterministic logic, mathematical operations, and transformations that must remain free of external state or mutable data. Additionally, when a **Const Function** is called **exclusively with literal arguments**, it is **automatically promoted to compile-time evaluation**, behaving like a **Literal Function** to further enhance performance.

Soul introduces a feature called **Literal Retention**. This optimization allows variables initialized with literal values to maintain their literal status internally, even when declared as `var` or `const`. This enables the compiler to perform additional optimizations at compile-time. If a variable with literal status is later mutated, it automatically becomes a normal runtime variable.

With its minimalistic syntax, explicit memory model, compile-time computation capabilities, and functional programming constructs, Soul is an ideal choice for developers looking to write fast, efficient, and safe low-level code—without the steep learning curve or verbosity often found in other memory-safe low-level languages.

## **Core Concepts**
### 1. **Borrow Checker**

Soul uses a custom borrow-checking system inspired by Rust but with looser rules. It prevents common issues such as dangling pointers and double frees while giving developers more freedom in pointer ownership and aliasing.

### 2. **Stack-First Memory Model**

Soul prioritizes stack allocation whenever possible, automatically placing pointers and objects on the stack to reduce heap allocations. This improves cache locality and lowers the overhead typically associated with dynamic memory.

### 3. **Minimalistic Syntax**

Drawing from Go, Soul maintains a simple and concise syntax, reducing boilerplate and focusing on developer productivity, while still offering fine-grained control over low-level operations.

### 4. **Rulesets**

Rulesets define how functions behave in terms of mutability, evaluation time, and side-effects.

- **4.1. Default**  
    Standard behavior for functions and variables, allowing full control over mutation, borrowing, and runtime evaluation.
    
- **4.2. const**  
    Enforces a **functional programming style**, disallowing side effects, mutation, and borrowing within **Const Functions**. **Const Functions** can be promoted to compile-time if called exclusively with literal arguments.
    
- **4.3. Literal**  
    Functions under the **Literal Ruleset** are guaranteed to be fully evaluated at compile time. Variables assigned within these functions cannot be mutated and must resolve to literals before runtime.
    
### 5. **Literal Retention**
when a `var` or `const` variable is initialized with a literal value, it retains **literal status** under the hood. This allows for compile-time optimizations. The literal status is revoked automatically when the variable is later mutated, converting it into a regular runtime variable.
