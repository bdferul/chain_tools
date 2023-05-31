Chain tools: The Way Rust Was Always Meant To Be
================================================

chain_tools is a library with the goal of turning every function into one long function chain. The inspiration comes from learning Haskell and finding myself frustated with the limitations of list comprehension.

# A Not-So Brief Rant About List Comprehension

For those who may be unaware, list comprehension is meant to be a quick way to 
define a new list based on some other list. In Python for example, you could 
write `[x*x for x in range(10) if x % 2 == 0]`. This would return the square 
of every even number under 10. In Haskell, the same would be written as 
`[x^2 | x <- [0..9], even x]`. At first, this syntax will seem confusing, 
especially in Haskell where the words are replaced with symbols, but it 
becomes trivial to read soon enough. Confusion is not the issue here.

A feature being confusing is not a problem. After all, I love Rust, a language considered by many to be one of the hardest and most confusing to learn. Those sentiments aren't wrong. When starting to learn Rust, every new feature you learn comes with ten new layers of complexity. This complexity might make the language harder for beginners, but it's there for a reason. Once you understand these complex systems, the power they give you is unparalled. Rust's type system is complex, but it is also the best feature of any language. 

With that established, let's come back to list comprehension. Rust doesn't have list comprehension. Rust doesn't need list comprehension. Rust has something better. When faced with the task of giving the developers an easy way to transform a list, Python and Haskell faced a problem. Their language was not designed around that kind of task. With Python, you can't even find the length of a list without wrapping it in some function (`len(list)`). Adding this kind of functionality would require a whole new syntax anyway. Haskell made the decision to write the entire language around list comprehension. I'll give them some credit, it does look very "mathy".

When Rust was faced with this very same problem, a lot of the leg work had been done already. The iterator trait was already capable of being extended with the `.filter()` and `.map()` methods. If you combine them together, you get the `.filter_map()` function which does the exact same thing as list comprehension. The functionality is identical. Here is some Rust to demonstrate: `(0..10).filter_map(|x| (x % 2 == 0).then(|| x * x))`. You could also just use the two methods separately with `(0..10).filter(|x| x % 2 == 0).map(|x| x * x)`.

What's the point then? I couild see many saying they prefer the look of list comprehension compared to rusts filtering and mapping. I can't even say that they're wrong, look is subjective after all, but there are some truths to keep in mind. 

**Truth #1:** *List comprehension breaks the sequence.*

Most programming languages operate with the following sequence: Top to bottom, left to right. List comprehension breaks this second sequence. You get the output first, then the definition, then the source, then the condition. ***This*** is ***This*** from ***This*** when ***This***. It might not seem like a big deal, but code can already be hard enough to read, and breaking the sequence will only make reading even harder.

**Truth #2:** *List comprehension is inflexible.*

List comprehension can only do 4 things. Filter, map, filter-map, or copy. What if you want to map and then filter? What if you want to chain another list? What if you want to fold the values too? You can do these things, sure, but you may need to chain multiple comprehension statements together, or leave them behind entirely. An entire feature, made to make your life better, is completely useless in many situations due to it's inflexablity.

This is where Rust shines. Being able to chain iterator function calls one after the other allows for some truly incredible flexibility. filter then map? `iter.filter().map()`. Map then filter? `iter.map().filter()`. Another filter and map? 
```rust
iter
  .filter()
  .map()
  .filter()
  .inspect()
  .cycle()
  .skip(5)
  .filter_map()
  ...
```
This can go on for as long as you need it to. That's the key here. Whenever you need it, this system will be flexible enough to do what you want, and stay readable while doing it. No matter how long the chain gets, the sequence stays the same. The logic follows clearly from function to function, never breaking.

- TODO: add more ranting.
