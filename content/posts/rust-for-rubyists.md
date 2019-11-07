---
draft: true
series: "rust-for-rubyists"
part: 1
date: 2019-09-09
title: "Rust For Rubyists"
tags: ["rust", "ruby"]
---
This is part one

<ul class="contents">
<li>Introduction</li>
<li>Gems -> Crates</li>
</ul>

Let's go through some of the things that make ruby so great:

- Concise, expressive code, can be almost like a natural language
- Dynamically typed
- Interpreted; write code, run it instantly
- Gets out of the way, let's you solve problems
- “The goal of Ruby is to make programmers happy.” ~ Yukihiro Matsumoto

Rust:

- More verbose, less expressive, a long long way from English
- Strongly, statically typed
- Compiled; write code, wait several seconds, run it
- Constantly in your face:
    "you haven't covered that case"
    "you've used that variable already"
    "what the hell even is that?"
- “Rust is a systems programming language focused on three goals: safety, speed, and concurrency.” ~ Rust documentation

So why, as a rubyist, learn rust at all? Let's look at the three goals of rust again: safety, speed, and concurrency. Rust is built on these three pillars, three things that for all its strengths, ruby is not renowned for.

def show_length(object)
  puts "It's #{object.length} long"
end

a = "Hello"
b = [1, 2, 3, 4]

show_length a
show_length b

c = b[0..1]

https://stevedonovan.github.io/rust-gentle-intro/readme.html



safe by default; all memory accesses are checked. It is not possible to corrupt memory by accident.

strictly enforcing safe borrowing of data
functions, methods and closures to operate on data
tuples, structs and enums to aggregate data
pattern matching to select and destructure data
traits to define behaviour on data
