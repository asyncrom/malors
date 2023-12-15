# Malors
## <u>Ma</u>thematic <u>Lo</u>gic from <u>R</u>u<u>s</u>t
Malors is a toy language to run logic on maths expressions
The interpreter is made from scratch in Rust
The goal is to make scripting on calculators easy with fewer keywords and characters needed

# Steps:
-[x] Tokenizer
- [ ] Interpreter
- [ ] std
- [ ] multi-line fn / while / if

# Examples
```
a = 5                 # Declare a variable
a = 2a                # Modify the variable
c = b**(a/2) + 3      # Different operations 
f(x) = 3x**2 + 2x - 5  # Declare a function

while i <= 0          # While loop
:a+= 1                # Increment 
:i = f(a)             # Use the function

while i <= 0 :a++ :i=f(a) # On one line !

y = f.(i)             # Use the derivative function
```