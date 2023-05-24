<?btxt+rust filename='main.rs' ?>
<?btxt+rust tag='examples'  pre=||| fn main() {
||| post=|||
}||| mode='overwrite' cmd='rustc main.rs && ./main' ?>
<?btxt+go filename='main.go' ?>
<?btxt+go tag='go' pre=||| package main
func main() {||| post=|||}||| cmd='go run main.go' mode='overwrite' ?>
<?btxt+javascript mode='overwrite' filename='index.js' cmd='node index.js' ?>

Rust is definitely weird. It's also hard. But one thing to keep in mind is that almost all of its difficulty isn't because rust just decided to be hard -- these difficult topics are important to understand in other languages, those languages just let you compile your code without understanding them. So as you learn what rust is trying to teach you, you'll become a better developer even in other langauges.

This module will introduce the things that are particularly weird, especially when compared to Go. Since everyone here is familiar with Go, we'll use that as our baseline frequently.

If you need convincing, here is Google's Chat AI Bard, telling it like it is:
![[bard_rust.png]]
> Bard is terrible, so uh... take that with a spoonful of salt

# Key Topics

The purpose of this module is to present some critical topics in Rust that are novel (at least coming from Go) and essential for decoding Rust source. However, it is definitely _not_ expected that after this module everything in it makes perfect sense. You _should_ have a rudimentary understanding of what is being described but it's fine if things are still vague. 

You will almost certainly encounter these topics while progressing through the training, and this module can serve as a reference, in addition to the Rust documentation and community, to help you decode and understand what you're writing and reading.

## Ownership

Rust doesn't have a garbage colletor. Like C and C++, rust forces the developer to manage their own memory. Unlike those languages, though, Rust won't let your code compile if you've mismanaged your memory.  This is all handled through Rust's concept of ownership.

The best part is, you've definitely seen ownership before -- scopes in all languages own the variables that are defined within them. If you drop the scope, you drop the variable. See the go example below.

```go own1
foo := "foo"
{
	bar := "bar"
}
foo = bar
```

This example won't compile because bar is already gone by the time foo tries to read its value. No remaining scope owns the variable `bar` so the compiler won't let you access the value. In rust, having a scope (or the possibility of one) that ends before being used is referred to as not _living long enough_.

In languages that clearly delineate stack vs heap types, like C or C++, stack memory has always been automatically garbage collected -- because it uses ownership. The compiler knows to clear stack memory when the stack is dropped. You only have to worry about managing your own memory, in those languages, when you allocated something on the heap using `malloc` or similar. (C++ also has an ownership system like rust's, but it is _optional_ since C++ is a superset of C)

The problem with C is that if you pass a pointer to a value defined on the stack, and then that stack is dropped, the pointer no longer (necessarily) points to that variable. Rust won't allow this. So rust gives you all the advantages of C/C++ (bare metal, zero runtime or garbage collector) and none of the risks. It also fixes problems with most other languages, as we'll go over periodically.

```c
#include <stdio.h>

int main()
{
    int *p = thing();
    printf("p is %d", *p);
    
    return 0;
}

int * thing() 
{
    int *i [4] = {1,2,3,4};
    return i;
}
```
>`*i` is allocated on the stack, so its dropped after thing() returns, so what does the pointer point to? This is called a dangling pointer. Rust doesn't allow this.

So, ownership in rust takes these concepts that are already clear to most developers and integrates its memory management to heap types as well -- still using the same rules. When the scope that defined a variable is dropped, so is the variable, and rust ensures _at compile time_ that no one can still possibly be referencing that variable after the owning scope has been dropped.

Because ownership is a critical concept in rust, this changes the output of certain error messages and introduces some new lingo that may be confusing at first (actually, it'll _almost certainly_ be confusing at first). Consider the following rust program:

<?btxt+rust filename='fail.rs' tag='ownership' ?>

```rust own2
fn main() {
	let foo;
	{
		let bar = 5;
		foo = &bar;
	}
	println!("{}", foo);
}
```

You get an error during compilation, and the error says this:
```
error[E0597]: `bar` does not live long enough
 --> /tmp/fail.rs:6:9
  |
6 |         foo = &bar;
  |               ^^^^ borrowed value does not live long enough
7 |     }
  |     - `bar` dropped here while still borrowed
8 |     println!("{}", foo);
  |                    --- borrow later used here
```
Whenever you see "does not live long enough" what its really saying is that the owner's scope has dropped. Take a look at our code. `bar` is defined in a nested scope block, so `foo`
has a "broader" scope. A way to look at lifetimes (not perfect, but good for a start) is that a broader scope means a longer lifetime. 

So, if we translate the error into "scopes", we get something like, "`bar` is declared in a smaller scope, so we can't reference its value in `foo`, which is declared in a broader scope". Who knows what the memory that was originally at `*&bar` now points to? We can't know, for sure and in every situation, that its safe and so rust simply won't let us do it.

### Terminology 
Since Rust completely relies on "ownership" to manage its memory, this leads to a few terms you've likely never heard of before. We've already seen one mentioned, and thats `lifetime`.

#### Lifetime
Lifetime refers to the scope of the variable, or when the variable is first defined, and when its scope is dropped. This period of "time" is the lifetime. "Living long enough" refers to the scope being broad enough.

#### Borrowing
References in rust are called `borrows` (but also sometimes references, its not ideal), but the act of referencing a variable is called `borrowing`. This means that only one variable (and one scope) can ever _own_ a value, but many places can borrow it. The component of the rust compiler that makes sure that references do not live longer than their owners is called the "borrow checker". 

#### Move
A move occurs by default when you have a variable, say `foo`, and either return it from a function or assign it to another value. Move implies that the previous variable (or scope, or both) no longer owns it -- ownership has been moved. However, it also implies a lot more. 

Take a look at the following golang snippet:

```go 
type Foo struct {
	Foo: string,
	Bar: []string
}

func main() {
	foo := Foo{}
	doTheThing(foo)
}

func doTheThing(foo Foo) {
    // To pass Foo as value COPIES the Foo
}
```

While we can't always be sure if golang will copy or move (move semantics are hardly Rust's invention), it very frequently _will_ copy. And since we can't be sure, that means we basically have to pass by reference for efficiency (this actually can lead to inefficiencies because the garbage collector has to do more work with more references, although in most cases passing by reference is better to avoid a copy).

Rust doesn't have this problem. Nothing (except very small scalars like `isize`) is ever copied without you explicitly telling rust to copy. Everything is _moved_. Moving means the underlying values aren't copied, only a pointer or, in some cases, a "fat" pointer, are copied to a new stack frame. The data is not. Even in select few cases with stack types where the whole thing has to move to a new stack address, its still more efficient than a copy since it doesn't have to leave the old stack address in memory. 

<?btxt+rust filename='main.rs' ?>

```rust move2 
struct Foo {
	foo: String,
	bar: Vec<String>
}

fn main() {
	let foo = Foo{
		foo: "foo".to_owned(), // don't worry about this, yet
		bar: Vec::new()
	};
	do_the_thing(foo);
	// uncommenting this line will break compilation
	// println!("{}", foo.foo)
}

fn do_the_thing(foo: Foo) {
	// This function now _owns_ foo
	// main() can no longer interact with foo at all
}
```

An important thing to remember is if you use a variable, aside from declaration, without borrowing (without  `&`) then it is being moved. 

### Stack vs Heap

Every value you create in Rust is created on the stack. This is what enables ownership to work -- when the stack frame that created the object is dropped, then its easy to clean up the memory at that point. 

But while all values you create are on the stack, that doesn't mean everything is on the stack. When you create a `Vector`, the data _in_ the vector is allocated in the heap. What you create in the stack is a smart pointer. The smart pointer knows how long the vector is, where it starts, its current capacity for new elements, etc. When the stack frame that owns the smart pointer is dropped, the smart pointer will then free the heap memory it consumed.

Because all types you define are created on the stack, like structs, this means that the size of all fields _must_ be known at compile time. This is usually fine, but occasionally you'll encounter a situation where you need to store an object with unfixed size (a common example is dynamic trait objects, which we'll address in the `Error` trait section). 

In these situations, rust provides a type that lets you indicate that a value should be in the heap, `Box`. Box works the same way as vector -- when the box smart pointer is dropped from the stack, then it frees the memory it allocated. 

### Copy Types

Because taking a reference or moving ownership requires creating and moving a pointer around, some types are actually no more efficient to move than they are to just copy. These types, which are primarily simple scalars like integers and floats, implement `Copy`. Because there is no performance advantage to moving them (and in fact it _may be slower_), they are silently copied instead of moved.

### Reference Counters and Clone
<?btxt+rust mode='overwrite' pre='' tag='rc' post='' filename='rc.rs' cmd='rustc rc.rs && rc' ?>
While ownership is all well and good, you'll eventually reach a point where you need multiple "owners" of a value. Rust categorically doesn't support this, but you can emulate it using the `Rc` type. Rc stands for Reference Counter, and it works similarly to how a reference counting garbage collector (a la Python) works. 

Essentially, when you create a reference counter, the counter itself takes ownership of the value, moves it to the heap, and allows you to pass that value without copying it or referencing it. You do this by `clone`ing the Rc.

Here's is the (broken version) of the move example rewritten to use a `Rc`:
```rust rc1
use std::rc::Rc;

struct Foo {
	foo: String,
	bar: Vec<String>
}

fn main() {
	let foo = Foo{
		foo: "foo".to_owned(), // don't worry about this, yet
		bar: Vec::new()
	};
	let foo = Rc::new(foo); // foo the Rc now owns the Foo instance
	do_the_thing(&foo.clone()); // we aren't duplicating a Foo, but just an Rc
	println!("{}", foo.foo);
}

fn do_the_thing(foo: &Foo) {
}
```
> This isn't exactly a _good_ example since we should just borrow foo here. But it serves its purpose. We'll have more practical examples of Rc and similar types in later sections.

The two key take aways here are that:
1) `do_the_thing()` does _not_ take an `Rc`. It takes `&Foo`. But Rc can behave exactly like a &Foo. We'll go over how in the `Traits` section. 
2) Cloning foo does not copy the whole struct -- that data is in the heap. We are just copying a simple pointer to that data, as well as incrementing a counter.

#### Understanding Ownership Via Go's Garbage Collector

Let's look at our example of C's "ownership" we had above, where we created a dangling pointer problem, and rewrite it in Go.
<?btxt+go pre='package main
' post='' tag='gc' cmd='go run main.go' ?>

```go gc1
func main() {
	thing := thing()
	println((*thing)[0])
}

func thing() *[]int {
	slice := []int{1, 2, 3, 4}
	return &slice
}
```

Unlike the C example where passing a pointer exposed us to a potential dangling pointer problem, Go has a garbage collector. But how does this work? 

Here's what Go's own [FAQ](https://go.dev/doc/faq#stack_or_heap) says about when it allocates things on the heap:
```
When possible, the Go compilers will allocate variables that are local to a function in that function's stack frame. However, if the compiler cannot prove that the variable is not referenced after the function returns, then the compiler must allocate the variable on the garbage-collected heap to avoid dangling pointer errors. Also, if a local variable is very large, it might make more sense to store it on the heap rather than the stack.

In the current compilers, if a variable has its address taken, that variable is a candidate for allocation on the heap
```

So just the very act of taking a reference (and passing it from stack frame to frame) will allocate that value on the heap instead of the stack... thus preventing danging pointers. But it doesn't prevent memory leaks. For this, go has a separate thread, part of its runtime, that inspects things in the heap to see if anything is refering to them. If not, it deletes them. Go _used_ to use a reference counting garbage collector.

A reference counting garbage collector essentially adds a counter to things on the heap. The counter keeps track every time a reference is created (increment) and when that variable's function returns (stack frame is dropped), which decrements the counter. When the counter reaches 0, the memory is freed. 

So we can rewrite exactly what Go used to do at runtime... except Rust's version doesn't require a garbage collector or a runtime. 
<?btxt+rust mode='overwrite'  tag='gc' ?>

```rust gc2
use std::rc::Rc;

fn main() {
	let thing = thing();
	println!("{}", thing[0]);
}

fn thing() -> Rc<Vec<usize>> {
	let v = vec![1, 2, 3, 4];
	let rc = Rc::new(v);
	rc.clone()
}
```

So, to reiterate, ownership exists in all modern languages. But because of the limitations on compilers, or the reliance on garbage collectors and runtimes, those languages either sacrificed runtime performance for safety, or sacrificed safety for performance. By adding more compiler checks, some additional syntax and rules, rust eliminates the need for a runtime or a garbage collector. Rust sacrifices nothing for safety and nothing for speed -- except learning curve. 

We'll address specifically how all this magic works in the `Drop` trait section.

### Lifetimes

So, the one thing that everyone eventually struggles to understand about rust (yours truly very much included) is _lifetimes_. They are both very simple, and very different and strange. Once you understand them, you won't struggle (much) with them but at first they are possibly the most incomprehensible sources of frustration in rust. 

To explain lifetimes, we're going to use some functions interacting with a slice of data as a training ground.

#### Slices

In go, we know that under the hood of a slice is an actual array. When we try to expand the slice beyond the capacity of the underlying arrary, go will automatically create a new array with double the size, copy all of our stuff from the old array into the new, and then that new array becomes the basis for our slice instead. No one can "own" that underlying array in go -- you can never define it(without unsafe code). Effectively, the garbage collector owns it. 

In rust, slices are not growable -- because they are just a view into data controlled (and owned) elsewhere. 

Lets take an example of some sort of file parser. We read a (potentially) large amount of data into memory, parse out interesting bits, and send those bits to other places in the program. (Yes, ideally you wouldn't parse the whole thing at once, but that's a problem for another day)

Here is our "large file":
<?btxt+txt filename='input.txt' tag='slice' mode='overwrite' ?>
```txt
Lorem ipsum dolor sit amet, 
consectetur adipiscing elit, 
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
Ut enim ad minim veniam, 
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. 
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

If we want to read all this text into memory in Rust, we would do something like this:
<?btxt+rust filename='main.rs' tag='slice' cmd='rustc main.rs && ./main' pre='' post='' ?>
```rust slice1
use std::fs::File;
use std::str::from_utf8;
use std::io::Read;
use std::io::BufReader;

fn main() {
	let file = File::open("input.txt").expect("error opening file");
	let mut buffer: Vec<u8> = Vec::new();
	let mut reader = BufReader::new(file);
	reader.read_to_end(&mut buffer).expect("error reading contents into memory");
	println!("{}", from_utf8(&buffer).expect("unable to parse input as utf8"));
}
```
> Ignore all the `expect` stuff for now -- we'll go over that when we get to error handling.

So, at this point we have a vector of bytes `buffer` containing all the contents of the file. What if we wanted to relocate this code into a function that returns the contents of the file, given a filename? This should be just a few minor changes:

```rust
use std::fs::File;
use std::str::from_utf8;
use std::io::Read;
use std::io::BufReader;

fn read_file(name: &str) -> Vec<u8> {
	let file = File::open(name).expect("error opening file");
	let mut buffer: Vec<u8> = Vec::new();
	let mut reader = BufReader::new(file);
	reader.read_to_end(&mut buffer).expect("error reading contents into memory");
	buffer
}
```
> Note, this is not a great example of proper error handling. We're ignoring that for now

Note the return signature of this function: `Vec<u8>`. It is returning by Value (in go terms) and not a Reference (which is what you usually return in go to prevent copy). If you create an object in a function in rust, you very rarely would want to return a reference to that (and it would be difficult to do so). Why do you think that is?

We can easily write a function to borrow our file's vector, in order to do some processing. We just pass it a reference (or borrow):
<?btxt+rust mode='append' ?>

```rust
fn count_lines(data: &[u8]) -> usize {
	data.iter().filter(|byte| **byte as char == '\n').count()
}
```
> Don't worry about the `filter` `iter` and things. We'll address this in more detail in `Traits`

Of note, `'\n'` differs from `"\n"`, because single quotes can only hold one character -- they are `char` literals, not `&str` literals. Characters in rust are not ASCII, but are [unicode characters](https://doc.rust-lang.org/std/primitive.char.html).

So, we can easily pass borrows, basically the same as references in go (except mutability, which we'll get to later). But what if, instead of counting the lines, we wanted to actually return the lines?

```rust
fn split_lines(data: &[u8]) -> Vec<&[u8]> {
	let mut start = 0;
	let mut result = Vec::new();
	for (idx, byte) in data.iter().enumerate() {
		if *byte as char == '\n' {
			result.push(&data[start..idx]);
			start = idx + 1;
		}
	}
	result
}
```

Well, that was a little longer. The real issue here is that we're working with bytes. Strings have built in `split` methods that would turn this into a similar one liner like the previous function, but this way we get to show a bit more logic -- creating a vector, and pushing new elements into it.

```rust slicefull
fn main() {
	let data = read_file("input.txt");
	let count = count_lines(&data);
	let lines = split_lines(&data);
	println!("{}", count);
}
```

So, let's discuss the lifetime of our vector of data. We read all the data in `read_file`, which passes the value back to `main`. The variable `data`  in `main` , and not `read_file()`, now owns our data. Since `data` exists until the end of `main`, we can safely borrow its data. Rust can easily ensure that the Vec exists at least as long as the references passed to `count` and `lines`. 

Note that `lines` contains a vector of references. It does _not_ contain a duplicate of any data, as we never `clone` anything. Essentially what it contains is a list of start and end indexes, and it needs to look at the original Vec to actually get the values. 

Let's break down a brief example. The first two lines of our input are this:
```txt
Lorem ipsum dolor sit amet,
consectetur adipiscing elit,
```

So `lines` has something that looks like this:
```json
[
{"start": 0, "end": 27},
{"start": 28, "end": 58}
]
```
Because a slice is basically a pointer to the start and end of a sequence of data. So what would happen if we passed `data` elsewhere, and then printed `lines`? What if we mutated the contents of `data`? If `data` is cleared from memory, and then we print `lines` what would happen? 

In order for Rust to do its job and have zero sacrifice performance and safety, it must be able to know the complex relationships between where the _actual_ data is, when it was defined, and when that memory will be inaccessible (when the stack frame drops). Because a view into a sequence of bytes cannot live longer than the bytes themselves -- otherwise we have undefined behavior and potential SEGFAULTs. This is what lifetimes are all about. 

#### Lifetime Annotations

Lifetimes are the rust's compiler's way of tracking the scope (or lifetime) of a reference. We've already used them -- in our `split_lines` function above, we return an _owned_ `Vec` of _borrowed_ values. So what's so confusing about that? Well, when there is only 1 borrowed input parameter, and 1 borrowed return type, Rust knows that those two values are paired -- the lifetime of the ouput must be the same as the input.

> Think about that last statement for a while. Why is it true that, if there is one reference input and one reference output, they must have the same origin -- the output must be referencing the input, 100% of the time?

When the compiler lets you omit the lifetime annotations from a function signature, this is called _lifetime ellision_. 

So, when there is only one input and one output reference, no lifetime annotations are needed. When you have multiple reference input parameters, the story changes. First, lets establish what lifetime annotations are. We'll rewrite our `split_lines` function to use lifetimes:

<?btxt+rust tag='lifetimes'  mode='overwrite'  filename='main.rs' pre='' post='' ?>
```rust
fn split_lines<'a>(data: &'a [u8]) -> Vec<&'a [u8]> {
	let mut start = 0;
	let mut result = Vec::new();
	for (idx, byte) in data.iter().enumerate() {
		if *byte as char == '\n' {
			result.push(&data[start..idx]);
			start = idx + 1;
		}
	}
	result
}
```

So, we effectively add a generic parameter (we'll go over generics a bit later) that starts with a `'` (a single quote). The name doesn't matter. Instead of `'a` we could have named it `'only`. Simply by convention, lifetime names start with `'a` and go up the alphabet. However, especially while learning, feel free to name them whatever helps you understand what you're reading. 

These are some of the strangest bits of rust's syntax, and when digging through source code they can get overwhelming. Fortunately, _most_ of the time, especially in the beginning, you can pretend they aren't there... except when you write your own of course.

So, when do you need to use them? Well, the super simple answer is, when the compiler tells you. But the slightly longer truth is simply, whenever its not possible to infer from the function signature which input is refenced in which output. In order for rust to be able to ensure that you don't have references that live longer than their values, it must be able to tell which output comes from which input. 

We're not going to dive super deep into a bunch of examples right now. This is one of those concepts that needs to be used, discussed, and thought about for a while before its likely to make a lot of sense. But we will have one example of a struct that has two, separate unowned string fields.

```rust ltstruct
struct PatchName<'a> {
	old: &'a str,
	new: &'a str
}

fn main() {
	let old = "Bob Marley"; // string literals are type &str
	let new = "Rob Marley"; // we'll go over why later
	let patch = PatchName{
		old,
		new, // javascript-like shorthand for setting like-named struct fields
	};
	println!("{} -> {}", patch.old, patch.new);
}
```

So in our example, we only specify one lifetime in our struct. This means that our two different references _must_ have the same lifetime (or the same scope). In our `main` function, this is not an issue, as both `old` and `new` are defined in the same scope. But consider the potential use for this -- maybe for showing changes to an API resource by comparing the existing value in the database to the new one. If that's the case, it would be likely that these two come from different places. In that case, we should probably define two different lifetimes:

```rust
struct PatchName<'old, 'new> {
	old: &'old str,
	new: &'new str
}
```

#### Static Lifetimes

There is one name you can't use for your lifetimes, except under specific situations. This is `'static`. Static is a special lifetime, one that implies that the value will exist for the lifetime of the running program. This means they have no constraints on usage, because the application never has to worry about that memory being changed (`'static` values are also immutable), moved or deleted. 

The most common type of `'static` type is that of string literals. Since literals exist in the source code, they exist for the entire program (and are typically in the `strings` table of the compiled ELF). 

```rust static1
const FOO: &'static str = "foo"; // immutable, always available 
// constants must ALWAYS have their type defined

fn main() {
	print(FOO);
}

fn print(s: &'static str) { // we can _only_ pass this a string literal
	println!("{}", s);
}
```

## Safe, Correct, and Efficient Defaults
<?btxt+rust tag='safe' ?>
The design philosophy of rust is to be safe and correct by default. Since its a systems programming language, it also strives to be efficient by default. This has a few direct effects that distinguish it from other languages.

### Immutable By Default

For one, variables are _immutable_ unless you explicitly declare them as mutable. (In javascript terms, all `let` statements in Rust produce `const`s in javascript).

This means that variables declared with `let` and parameters in functions are immutable, regardless of whether they are references or by-value (move or copy).

```rust
let foo = 1; //immutable
let mut bar = 2;
bar += 2;
```

This has consequences for the borrow checker, too. In order to be _safe_, you can never have the same variable _mutably_ borrowed while it is also borrowed (mutably or not) in another place. This prevents race conditions at compile time. It also means you can safely pass around references without it being unclear whether the intent is to mutate it or not -- if they need to mutate it, it'll be marked `mut`. Align your semantics with your syntax!

### No Zeroing of Types

One of Go's "features" is that any type, when declared, automatically becomes the Zero value for that type if it isn't set to some value. This isn't a thing in Rust. Doing so has dangerous side effects even in Go -- [http servers with default](https://github.com/golovers/effective-go#dont-use-default-httpclient--httpserver-in-production), zero configs have timeout values that can leave the door wide open for DOS attacks. The header read timeout defaults to the max value (if its zero, it's considered basically max), and that means, by default, all go servers are trivial to attack -- just write a script that opens a connection and never sends headers. 

This "easy vs correct" is a key differentiator between Go and Rust, and you can find some pretty fun flame wars on the interwebs on the subject, specifically between Go and Rust. 

Of course, it also helps that types in Rust can have actual constructors so each end user of your types doesn't have to set each value each time to sane defaults. 

### Fearless Concurrency

Go is famous for having built in, really "easy" concurrency. And, to an extent, that's very true. But its also _really_ easy to shoot yourself in the foot with Go's concurrency. While it provides a _simple_ interface to spawning concurrent tasks, and nice channel primitives, anyone who attempts to write anything even remotely sophisticated in Go _will_ run into deadlocks, data races where multiple workers modify the same value simultaneously, etc. This is because, while go _provides_ the types for safe concurrent access (e.g. `sync.Mutex`) absolutely nothing prevents you from doing it wrong... and then you only find out at runtime, and possibly cause a production outage. No bueno. 

Rust solves this, too, in its type system. Rust knows what types can be safely sent across threads and what can't. It won't allow mutability of of _any type_ for _any reason_ if its _ever_ sent across a thread. And Rust's `Futures` are immune to deadlocks by nature. So, while it might be a bit more complicated to get parallel compute working in rust, you can be completely assured that if your code compiles, you won't have any nasty surprises waiting around the bend.

### Explicit Copy Operations

As already mentioned, Rust never copies heap objects unless you explicitly `clone` them. This means that you are always, painfully aware of every potential inefficiency. Don't let this deter you, especially in the beginning, from `clone`ing values, though. While it very well might be an indicator that your code is inefficient, its also something that you've been doing in other languages and just never were made aware. 

This general philosophy, of always exposing to the developer the cost of an operation, is continued into many facets of Rust. 

### Static Dispatch By Default

Another thing where rust takes the approach of being explicit about potential inefficiencies is with dynamic versus static dispatch. Rust supports both, but most things are usually Static Dispatch. Static Dispatch is usually faster than dynamic, but static dispatch requires generating much more code from generics and macros, so it leads to slower compilation, and sometimes, much larger amounts of work offloaded to the developer.

But because dynamic dispatch uses more _memory_ and is slower at runtime, rust disallows causally mixing the two. If you must use a dynamic dispatch type (where the concrete type isn't known, only that it implements some trait), then you must also use the `dyn` keyword. We'll see an example of this in the `Error` trait section.

## Macros

Rust is a pretty verbose language. Typing everything out to accomplish everything would be a _lot_ of work (although, lets be honest, it'd probably still be less work than error handling in go, with an if statement on every second line).

Additionally, since rust doesn't have a runtime, rust needs some way to metaprogram. This is what [macros](https://doc.rust-lang.org/book/ch19-06-macros.html) are for. The function you've seen so far that end with a bang (`!`) (e.g. `println!()`) are macros. Macros run _at compile time_ to expand code in some procedural way. 

You'll see many examples of macros throughout this module, and the rest of the training, but we won't be implementing our own for some time. For now, all you need to know is that macros are _zero cost_ since they happen at compile time (although they do make compilation slower), that any failures happen at compile time (and thus are far more friendly to build than complicated runtime reflection in Go), and that there are two types:

1) [Declarative macros](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming). These are functions ending in !. The most common ones are things like `println!()` which expand to more complicated code to handle formatting and parsing of output. The syntax for writing them looks very similar to a regular expression, and its basically just matching patterns and building generated strings of code. 
2) [Procedural Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes) exist primarily for the `Derive` macro, which enables you to implement a Trait for a type based only on the composition of the struct, and some other annotations the developer can provide. Writing a procedural macro is essentially writing rust code that executes at compile time. You can do really anything rust can do -- including fetch web pages, execute local system calls, etc. This is _Extremely_ powerful, but does mean that its possible for code compilation to do bad things to your machine.

We won't be spending a lot of time on them -- I don't think its worthwhile. Its an intermediate topic to me, and simply knowing what they are is enough for now. Feel free to browser the links above if you want more information. You can also look at the relatively simple example of the derive macro for my JSONAPI crate [here](https://github.com/qmuloadmin/jsonapi/blob/main/jsonapi_resource_derive/src/lib.rs).

## Types, Types, and more Types
Most people who end up developing for a significant period of time gain an appreciation for types. Compared to dynamic languages, they help ensure that code is more likely to be doing what you want, and eliminate whole categories of tests. 

Rust takes types to the next level. It has types for things you've probably never thought of needing types for. Unlike go, which heavily relies on `interface{}` and runtime reflection to get much of anything real done, Rust does everything at compile time. Rust doesn't have any support for runtime reflection -- because rust _has no runtime_. Rust only has the runtime you program into it. There's no garbage collector, no asynchronous executor, nothing running except the code you wrote (or imported). 

If you wanted your mind to be blown, Rust's type system -- by itself -- is turing complete. There is quite literally nothing you can't do with it (if you're willing to sacrifice your sanity and compile times).

### The Unit Type
It might seem a silly place to start, but its actually quite significant -- Every single function in rust returns _exactly_ one value. Not zero, Not two. One. This means that method chains can quite literally be endless, leading to one of rust's greatest strengths -- expressiveness. 

Obviously, some functions never need to _really_ return anything. For instance, a setter for a field on a struct. In these cases, those functions automatically return the unit type, which is an empty tuple: `()`.

In addition, every statement in Rust is also an expression, because rust is an expression-based language. This means `if/else` is not a statement, its an expression. So, no ternary operator needed. 

Here is an example:

```rust unit1
// don't worry about these types just yet
let input: Vec<&str> = "foo bar".split_whitespace().collect();
// functions ending in a ! are macros. We'll go over those later
println!("You said {} word{}", input.len(), if input.len() > 1 {"s"} else {""});
```

Important observation: the `if` block does not include `return`, but it automatically returns the last statement's value, in this case `"s"`. This is a very common pattern that at first will seem very strange, but over time you'll come to appreciate it. (Note: technically any expression without a `;` returns, however this should be the last line)

In functions, the last line of a function is returned, even if `return` is not specified (use `return` only for returning _early_). Consider the following:

```rust unit2
let some_closure = |x| {x;};
let other_closure = |x| {x};
println!("some closure: {:?}", some_closure(5));
println!("other closure: {:?}", other_closure(5));
```

Here we are introduced to closures in rust. A closure is strongly typed, although it might not seem like it, since we don't put any types on our variables. Rust has type inference, and can determine the type from usage. This is _true_ type inference, unlike go, which just has default types for all literals. 

Also shown here is the formatting syntax for `println!()`. This is common among all string formatting macros, whether thats `println!` or `print!` or `format!`. It is very similar to python, but the very short version is that `{}` prints the `Display` implementation for that type (similar to go's `Stringer`) and the `{:?}` prints the `Debug` implementation. This is similar to `repr` in Python, or `spew.Dump` in go... but its all compile-time in Rust and not runtime reflection. 

But the really important part here is that both of these functions ostensibly do the same thing -- return the input. But one has a semicolon and one doesn't. A semicolon on the last line basically truncates the value of the expression and the function instead returns the unit type, `()`. 

### Enums

<?btxt+rust tag='enums' pre='' post='' ?>

Many people for a long time considered the lack of generics to be the biggest weakness with Golang (include Rob Pike). And while I do agree that generics would fix a lot of problems (yes, they added them, but not _really_), the biggest issue with go for me is the lack of enums.

By including enums, Rust single handedly solves two of the biggest sources of bugs in go -- unhandled errors and null (or nil) pointer exceptions.

#### No Null Pointers Ever Again

One of the biggest, most obvious example of better types to anyone who's ever written Java, or Golang, or Javascript is that rust has no concept of a null reference. It's impossible. Every variable must be initialized to be used. Null pointers are a bad idea, and by this time, everybody knows it (Rust is hardly the first language to remove support, Kotlin and Swift both are immune to null pointers as well and even Python is immune and it predates Java).

However, at some point, you will _need_ to support a value being "empty" and not being forced to set it to some value just for the sake of satisfying some crusade against null pointers. For this, Rust uses a standard enum, `Option<T>`.

A hugely important distinction is that this is a different type. You cannot compare a `T` to an `Option<T>`. In go, there is no capacity in the type system to know if a pointer can be null. This means you have to use sophisticated linting to determine that possibility via tracing of code paths... But a simple type fixes all of that. Of course, to enable this type, you need generics and enums. Here is what Option looks like:

```rust
enum Option<T> {
	Some(T),
	None
}
```

This is Rust's enum type. Individual "fields" are called variants. A variant can be defined in many ways, but here are shown two (and these two are by far the most common, so we won't go over the others right now).

##### Match

In general, rust has a replacement for `switch/case` (and to a large extent if/else) that is far more ergonomic to use. When you need to test a value that can be mutliple variants of an enum, match is what you _typically_ use. 

```rust enummatch1
struct Foo<T> {
	bar: Option<T>
}

fn main() {
	let foo = Foo{
		bar: Some("hello")
	};
	let bar = Foo{
		bar: None
	};
	print_foo(&foo);
	print_foo(&bar);
}

fn print_foo(foo: &Foo<&str>) { // there are better ways to do this, for example only
	match &foo.bar {
		Some(x) => println!("{}", x),
		None => println!("Nadda")
	}
}
```
> Note, usually you have to use the fully qualified enum variant, e.g. `Option::None`, but a few core enums are automatically in the namespace, for convenience.

There's a lot going on in this short example. For one, we show how to create new generic structs containing generic fields. We'll discuss generics in more detail a little later. 

Two, even though `Foo` the struct is generic, the `print_foo()` function is not -- it will only work with Foo structs where `T` is `&str`. We'll talk more about why this is in the generics section.

But most importantly for this section is how `match` works. Match uses expressions to unpack enums on the left side (it works for more than just enums, but is most often used for that). So, when the enum variant contains a value, you use the syntax that would be used to _create_ that value on the left side with variables to store the value. You can use match to destructure and unpack nested types, too. 

```rust
enum Foo {
	Bar((String, isize, bool))
	Baz
}

fn main() {
	foo = Foo::Baz;
	match foo {
		Foo::Bar((s, i, b)) => {},
		Foo::Baz => {}
	}
}
```

##### Option Methods and If Let

One of rust's strengths is its expressiveness. This is, as previously mentioned, largely due to each function returning exactly one value. This means that you can do many things without needing to resort to match. 

One very useful, but slightly weird pattern is `if let`. If let is useful if you want to take a certain action if the value of an `Option` is Some, but want to do nothing if it is `None`. Consider the below example. Imagine that this is part of a REST API GET method, for instance a set of filters to filter the results of a resource. Obviously, this is not fully functional code and we're hiding a lot, here. We'll get to do a real implementation of this in the next module.

```rust
struct Filters{
	group: uint,
	name: Option<String>,
	email: Option<String>
}

fn get_users(users: &UserRepo, filters: &Filters) -> Vec<User> {
	let mut query = users.getUsersQuery();
	query.filterByGroup(&filters.group);
	if let Some(name) = &filters.name {
		query.filterByName(&name);
	}
	if let Some(email) = &filters.email {
		query.filterByEmail(&email);
	}
}
```

So, `if let` uses pattern matching to check `if` a condition is matched, and then `let` a variable equal a value. In this case, `if let` is saying, `if filters.name.is_some() { let name = filters.name.unwrap() }`.

Which already presents some of the methods implemented on `Option`. 

`is_some()` and `is_none()` are self evident -- they return true or false based on whether the variant is some or none. 

`unwrap()` is a common method on `Options` and on `Results` (from the next section). This assumes the variant is `Some` and returns the value contained. If it is _not_ `Some` but is in fact `None` then the thread will panic and unwind. Avoid using this except in PoC code and immediately after checking `is_some()`. It is not a good idea to get in the habit of using `unwrap()`.

`expect()` is slightly more palatable than `unwrap()`. Expect takes a single string parameter and, if the variant is `None` then it panics with that string as the error body. This is still not a great habit to get into.

There are [dozens more methods](https://doc.rust-lang.org/std/option/enum.Option.html), less commonly used, defined on Option. 

#### Ergonomic Errors

For me, the thing that drives me the most crazy about golang is error handling. Not only is it needlessly verbose, its also prone to errors (pun intended). When you repeatedly type something along the lines of:

```go
if err != nil {
	return err
}
```

Over and over in a function, with different errors, scopes, etc. you increase the chance drastically of checking the wrong variable, [not checking an error at all](https://github.com/OrderMyGear/payment-service/pull/1267/files#diff-6140337e81750801315429325d7581a2b71d1d9c822a788bfebc86497bbe757aR40), accidentally returning nil instead of the error, etc. Many linters will catch those, but not always, and the compiler itself will _not_ catch it. 

Rust, again, solves this problem. The `Result<T, E>` type exists for when a function may return an error. Like `Option<T>` it is generic, but this time it has two generic parameters. One is the type to be returned on success, and the other is the type to be returned on error.

Note that, while the _convention_ is to return an Error as the value in `Err`, there is no actual constraint that requires `E` to implement the `Error` interface (but it almost always should). 

Let's take another look at the code snippet from the last section. It probably doesn't make sense for our "get users" function to just return a Vec of users. What if it fails? It should probably look something like this:

```rust
fn get_users(users: &UserRepo, filters: &Filters) -> Result<Vec<User>, MyError> {
	// ...
}
```

Now, typing this over and over again for every method in my module that fail would get very old. It is common practice for each module to define its own error type, and then alias the `Result` type for that package to no longer be generic over `E`:

```rust
struct MyError {
	message: String
}

impl MyError {
	fn new(message: String) -> Self {
		Self{
			message,
		}
	}
}

type Result<T> = std::result::Result<T, MyError>;

// modified this so its a complete example and easier to execute
fn get_users(good: bool) -> Result<Vec<usize>> {
	if good {
		Ok(Vec::new())
	} else {
		Err(MyError::new("oops!".to_string()))
	}
}
```

So, we have shadowed the std Result type for this module, to have only 1 generic parameter.  If for some reason you wanted to still use the standard, double-generic `Result` type, you could use the fully qualified path of `std::result::Result` (but this would be unusual and you shoud avoid it).

We also showed how to implement methods on a type. You use `impl` (implement) to start a block. You can then define any number of methods for that type. Rust uses the special names `self` and `Self` while inside `impl` blocks to provide access to the object being affected -- unlike go, you _can't_ pick the name (thank god). 

We'll go over more about `impl` when we go over traits. For now, this is how you create a constructor.

Like `Option` (and every other enum in rust) you can use match to take a different action based on the Result variant:
<?btxt+rust mode='append' ?>
```rust result1
fn main() {
	match get_users(false) {
		Ok(users) => {},
		Err(err) => println!("{}", err.message)
	};
}
```

Similar to `Option`, Result has `unwrap()` and `expect()` and they behave the same way as they do in `Option`. It also has `is_ok()` and `is_err()` in place of `is_some()` and `is_none()`. [There are many methods](https://doc.rust-lang.org/std/result/#method-overview) on Result that we'll have a chance to use throughout the training, but we won't specifically mention them all here (that's what documentation is for).

##### Passing Errors Up The Stack

So, we have a result type, but how does that actually help us? So far, we're not much better off than go. 

Well, allow me to introduce everyone's favorite operator. 

Drum roll, please...

`?`

That's right, a question mark. The question mark takes all that annoying `if err then return err` nonsense and just automates it. If the function returns a `Result` with an `Err` variant of the same type (or a type that can be converted into that type) as the function being called, the compiler just expands that for you. 

Here's an example, building on our code so far this section:
<?btxt+rust mode='overwrite' ?>
```rust result2
struct MyError {
	message: String
}

impl MyError {
	fn new(message: String) -> Self {
		Self{
			message,
		}
	}
}

type Result<T> = std::result::Result<T, MyError>;

// modified this so its a complete example and easier to execute
fn get_users(good: bool) -> Result<Vec<usize>> {
	if good {
		Ok(Vec::new())
	} else {
		Err(MyError::new("oops!".to_string()))
	}
}

fn get_users_count() -> Result<usize> {
	Ok(get_users(true)?.len())
}

fn main() {
	match get_users_count() {
		Ok(count) => println!("{}", count),
		Err(err) => println!("{}", err.message)
	};
}
```
> Note: unlike Go, Rust actually uses unsigned integers correctly (`len()` in go is not unsigned, nor are slice indexes)

Isn't having a real type system nice? We're only getting started. The `?` operator also works for Options (which is similar to TypeScript), and you can convert a Result to an Option and vice-versa. Since everything returns one value, you can build long chains of method calls that pass up any errors without having to break out of the method chain. 

#### Choosing Result or Option

It's fairly straightforward to choose Option when failure is not possible -- no real error state, just empty. It's also straightforward to choose Result when you need to return either a meaningful value or an error. But what if a function _can_ error, but has no meaningful return value? In Go, this would just be something like this:

```go
func DoTheThing() error {
    // ...
}
```

So, is this an `Option<Error>` or a `Result<(), Error>`? When you encounter this situation, just remember the old adage: "Failure is not an Option". There is a reason for this: Rust's compiler will generate a warning (which you can configure lint to fail on) if a `Result` is unchecked. It has no such enforcement for `Option`. 

### Strings

So far, you've seen a couple different string types: `String` and `&str`. All the reasons for this are numerous (and I don't fully understand it all) but it's mostly for two reasons:

1) Rust values correctness, even in the extreme edge case, and having two string types makes programs more correct
2) Ownership

The short version is, a `String` is an owned, heap allocated sequence of Unicode charaters. It can be extended. A `&str` is a slice of u8 bytes. It _may_ be on the heap but also _may_ be on the stack. 

A `String` can be converted to a `&str` simply by borrowing it (similar to how a Vec can be converted into a slice... a `String` is basically a `Vec<u8>` except that all the bytes in a `String` are absolutely guaranteed to be valid UTF-8 code points). Converting a `String` to a `&str` is absolutely free.

A `&str` can be converted into a `String` in 3 ways:

- `"".to_owned()`
- `"".to_string()`
- `"".into()`

Converting a `&str` into a `String` is _not_ free and _always_ copies data in the heap.

Why are there 3 ways to do it? Well, that's because there happen to be 3 traits implemented on `&str` that all accomplish the same thing, but for different reasons.

`to_string()` is the `ToString` trait, which is automatically implemented for all types that implement `Format` (you should really _never_ implement this yourself)

`to_owned()` is the `std::borrow::ToOwned` trait, which is used for types that have specific logic to take when they are converted from a reference to an owned value. You will likely never need to implement this, either, but that is simply because its esoteric.

`into` is part of the `From/Into` trait, which is _very_ common and you will implement this all the time.

So, what are traits? Well, I'm glad you asked.

### Traits

A slightly inaccurate definition, but an acceptible starting place, is that traits are interfaces. On the surface, they serve the same purpose: they provide a list of functions that a type may implement in order to be passed around and used via polymorphism. 

However, traits are more powerful than interfaces typically are. Traits are somewhat like an Abstract Base Class and an interface in one... combined with generics and a dash of super powers. 

Lets look at an example:
<?btxt+rust tag='traits' pre='' post=''?>
```rust 
trait FooBuilder<E> where Self: Sized, E: std::fmt::Debug {
	fn new() -> Self;
	fn add_value(&mut self, value: String);
	fn build(self) -> Result<Foo, E>;
	/// must_build panics if build fails
	fn must_build(self) -> Foo {
		self.build().unwrap()
	}
}
struct Foo {
	values: Vec<String>
}
```
> Don't worry overly about the `where` part -- we'll go over that in Generics

So, silliness of this trait aside, there is a lot on display here. As mentioned earlier, in `impl` blocks (and trait definitions), `self` and `Self` are reserved words.

`Self` is the placeholder for the type. A constructor (`new()`) should always return the type, so the method `new()` returns a new `Self`. `self` refers to the _instance_. In javascript, this would be `this`. In python its also called `self`. 

A function in an `impl` block that does _not_ have `self` as the first parameter is _static_. This means that function is called in static context (`Type::method()`) and doesn't have access to the instance variable. It's identical to static methods in Java. Go does not allow static methods because its silly.

A function that does take `self` may do so in 3 forms:
`self` is owned (no `&`) so this function will _consume_ `self`. It will no longer exist in the scope in which it was called, or outer scopes.
`&self` is borrowed, but cannot be mutated (changed)
`&mut self` is mutably borrowed, and can be mutated. We'll go over the details of this in a later section.

The last thing of note in this code block is that we actually provide an implementation for `must_build`. This is certainly not something Go allows. Many times, there is a sensible default implementation that can be leveraged to accomplish some task, so it doesn't make sense to require the implementor to implement it every time. But, this also allows the implementer to implement their own version of it, if they can do it more efficiently. In this way, traits are like ABCs.

So, let's actually implement our trait.
<?btxt+rust mode='append' ?>
```rust traits1
struct MyBuilder {
	values: Vec<String>
}

#[derive(Debug)]
struct MyError {} //unused, just for show

impl FooBuilder<MyError> for MyBuilder {
	fn new() -> MyBuilder {
		Self{ // Self and MyBuilder are interchangeable. Self means MyBuilder
			values: Vec::new()
		}
	}
	fn add_value(&mut self, value: String) {
		self.values.push(value);
	}
	fn build(self) -> Result<Foo, MyError> {
		Ok(Foo{values: self.values})
	}
}

fn main() {
	let mut builder: MyBuilder = FooBuilder::new();
	builder.add_value("test".into());
	println!("{}", builder.must_build().values.get(0).expect("should have one element"));
}
```
> Unlike go, where interfaces are implemented implicitly, traits must be explicitly implemented in Rust. This allows you to implement multiple traits on the same type that have the same funtion name... something which is impossible in Go. It also makes code intelligence easier and searching code easier... basically its better in every way.

Unlike Go, and like most languages with interfaces (python, java, C++, ruby), Rust's standard library has _many_ interfaces that are critical to writing sane, idiomatic rust. The next several setions will go over what I think are the most common and most usable traits.

The last important thing to remember is that rust has _zero_ magic. Everything standard types are able to do is codified through a Trait, which means your types can do it, too. Everything. The `+` operator is just a trait, so it the `=` operator. `Rc` being able to serve as the type its wrapping is also just a trait (`AsRef` or `Deref`). 

This last part is, in my opinion, the reason why Python became the most popular language in science and data domains. Scientists want to write normal `+` and `-` and `/`. They don't want to use confusing, inside-out methods like in Java for `.Add()` and `.Sub()`. That becomes extremely hard to read extremely quickly. Python allowed people used to math to create their own complex types and continue using basic math operators on them... Java did not. Rust did, though. So Rust will likely take over this space since it can offer the same benefits... but be _much_ faster and safer than Python.

#### Iterator

Iterator is, in my opinion, another thing Go desparately needed standardized. The fact that you can't make a custom type that you can use `range` with in Go means so many things are harder to use than they really need to be. That lack of an iterator type, in conjunction with the lack of generics, makes `map`, `filter` and `reduce` patterns really annoying to implement in Go.

Before we begin, let's take a look at the [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait in Rust. First, note how many methods it has! How could we ever implement so many? Oh wait, most of these have default implementations. Only `next` has to be implemented.

![[Pasted image 20230408093019.png]]

We also have something new - an `associated type`. This is another new thing to traits. In this case, since the Iterator could be iterating over absolutely anything, we have to define what the type of each item in the iterator is. 

[Here](https://doc.rust-lang.org/std/iter/index.html#implementing-iterator) is an example of how to implement an iterator, from the official rust documentation. It also goes over `IntoIterator`. Into and From methods (and Traits) are very common in rust.

##### Working with Iterators
<?btxt+rust mode='overwrite' pre='fn main() {
let people = vec!["Sergio", "Sergio", "Daniel", "Khalil", "Rene", "Mora", "Justin", "Dino"];
' post='}' ?>
So, once you have an iterator, what do you do with it? Well, the iterator trait provides dozens of methods for working with iterators. 

All of the below examples will use this vector of `people`:
<?btxt+rust ignore=true?>
```rust 
let people = vec!["Sergio", "Sergio", "Daniel", "Khalil", "Rene", "Mora", "Justin", "Dino"];
```
<?btxt+rust ignore=false?>
Maybe obviously, you can iterate over them in a for loop:
```rust iter1
for person in people.iter() {
	println!("{}", person);
}
```

You can also enumerate any iterator:
```rust iter2
for (idx, person) in people.iter().enumerate() {
	println!("{}: {}", idx, person);
}
```

You can filter any iterator:
```rust iter3
for (idx, person) in people
					.iter()
					.filter(|person| person.starts_with("S"))
					.enumerate() {
	println!("{}: {}", idx, person);
}
```

You can map:
```rust iter4
for phrase in people.iter().map(|person| format!("{} is awesome", person)) {
	println!("{}", phrase);
}
```

And you can reduce, but Rust has a slightly different version of reduce, called [fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold).
```rust iter5
let everyone = people
			   .iter()
			   .fold("all the students are: ".to_owned(), |acc, person| format!("{}, {}", acc, person));
println!("{}", everyone);
```
There is also an ordinary `reduce` which is similar, but fold is superior (because you can provide the initial value). And yes, in this particular case, it would be better to just use `join` on `Vec<&str>`:

```rust iter6
let everyone = people.join(",");
println!("{}", everyone);
```

There are dozens of other methods and I won't go over them all, here. But the last common one you will use a lot is `collect`. Collect takes an iterator and converts it back into a collection of some sort -- a concrete type. E.g. another Vector. It is commonly used after `map` to convert the values of a collection into something permanent. 

```rust iter7
let greetings: Vec<String> = people.iter().map(|person| format!("Hello, {}!", person)).collect();
println!("{}", greetings[0]);
```
> What do you think would happen if we got rid of the `Vec<String>` type annotation here?

#### Debug and Display
<?btxt+rust mode='overwrite'?>

Debug and Display are two different stringification traits. We've been using Display all along -- whenever you use a string format macro like `println!` and you use a format placeholder, like the basic `"{}"` then you are saying to use that type's `Display` trait. This of course means that you can only print, or format, types that implement `Display`. 

```rust display
struct Foo {
	value: String
}

impl std::fmt::Display for Foo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.value)
	}
}

fn main() {
	let foo = Foo{value: "Test".into()};
	println!("{}", foo);
}
```

A note on implementing traits: because Rust explicitly implements traits, once you write the `impl` block, your editor will usually give you an option to generate the methods for you:

![[Pasted image 20230416065811.png]]
> Clicking on the light bulb will give me the option to populate missing methods

`Debug` is similar, except that is generally isn't intended for _users_ to read, but developers. It is similar to Python's `repr` or Go's `spew` output. It usually has the type and value of the variable being debugged. Most of the time, you don't really need to implement this yourself, but you tell the compiler to implement it for you:

```rust debug
#[derive(Debug)]
struct Foo {
	value: String
}

fn main() {
	let foo = Foo{value: "Test".into()};
	println!("{:?}", foo);
}
```
> Note how we invoke debug output in a format string: `{:?}`

We'll go over more of how `derive` works in the Macro section.

#### Error
<?btxt+rust mode='overwrite' tag='error' pre='' post='' ?>

Errors in rust are very similar to go. In go, the error interface basically just has one function -- one that returns a `string`. They have expanded on this and now, by convention, most errors should also implement a chaining mechanism that allows errors to keep track of what caused them. In go, this is `Unwrap`. 

For rust, the only thing you _must_ implement for a type to be a valid implementation of `std::error::Error` is `std::fmt::Display` and `Debug`. So in short, an error must be string-able.

However, because interfaces must be explicitly defined in Rust, you must also "implement" the error explicitly, as shown. 

```rust error1
#[derive(Debug)]
struct MyError {
	message: String,
	cause: Option<Box<dyn std::error::Error>>
}

impl std::fmt::Display for MyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl std::error::Error for MyError {}
```
> Note the use of Box here, and the `dyn` keyword. This is because we're not sure exactly what the underlying error is (although sometimes, we could be -- we're being lazy here). So we need to use a smart pointer because we don't know the size of a dynamic trait object. All we know is its something that implements Error

So even though we aren't implementing any of the methods defined on `Error` we still need to create the empty impl block. (There are actually traits in Rust that _have no methods_ and you just need to say your type implements them, explicitly).

Since we have our `cause` field, we should also implement the `source` method, so a better implementation would be:

```rust error2
#[derive(Debug)]
struct MyError {
	message: String,
	cause: Option<Box<dyn std::error::Error>>
}

impl std::fmt::Display for MyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl std::error::Error for MyError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
	        Some(cause) => Some(Box::as_ref(cause)),
	        None => None
        }
    }
}

fn main() {}
```
> Don't worry about the confusing `'static` lifetime on the return signature. For now, just know that it's an implementation detail in Rust, and it has to be there.

Implementing `source` is a great idea whenever you're writing a library -- it can help the caller of your function understand the reason for the failure. It's usually _not_ necessary when writing an executable binary, though. Because anyone executing the binary probably _shouldn't_ see the reason for the failure in detail and it'll just be extra bloat in the program.

##### Trait Objects and Dispatch

Notice the `dyn` keyword in the above example. This is needed when dynamic types are used. Dynamic types are types that exist only when the program is running (in effect, they aren't really types at all, since Rust has no runtime), and the _real_ (called 'concrete') type isn't known at compile time. Essentially, in this case, its saying "Some type that implements `std::error::Error`". This is functionally the same as setting the type of a parameter in a go function to an interface. In other words, all interface-based dispatch in Go is _dynamic_.

`dyn` is generally not a great idea, but in this case, it is an ok idea. Consider the potential significance, here. Let's say `MyError` represents the error a method in an REST client api we are writing. If we wanted to account for all the possible underlying causes of an error in our client, what would that include?

Some possibilities:
1) DNS failure
2) HTTP connect failure
3) HTTPS handshake failure
4) Connection interrupted
5) Timeout
6) Unsuccessful status code
7) Bad response (not JSON for instance)
8) Missing or invalid header in response

And honestly, the list could go on and on. It _would_ be possible for us to enumerate them all, and in a robust, mature and critical library its actually a very good idea. That makes it very clear to the developer, in the type system itself, what went wrong. But it would be very time consuming. Using trait objects lets us forget about that headache. Errors _shouldn't_ be very frequent, either, so the performance penalty for dynamic dispatch should be negligable.  

##### Source Static Dispatch Through Enums

What would we do if we wanted to use static dispatch instead of dynamic dispatch for our `source`, though? There are two approaches. One is to use Generics, which we'll go over in the section on Generics. The other is to use an Enum. 

```rust error3
#[derive(Debug)]
struct MyError {
	message: String,
	cause: Option<MyErrorCause>
}

enum MyErrorCause {
	DNSError(net.ResolveError), // These types are made up
	Timeout(tcp.Timeout),
	// ...
}

impl std::error::Error for MyError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
	        Some(cause) => match &cause {
		        MyErrorCause::DNSError(err) => Some(err),
		        MyErrorCause::Timeout(err) => Some(err),
		        // ...
	        ,
	        None => None
        }
    }
}
```

So, its definitely a lot more typing, but it's also really clear, isn't it? We could then glance at the types of this libraries errors and know all the different things that could go wrong, and then we could decide which of those errors indicate we should just try again, and which indicate we should give up, or other methods of handling the error.

#### Serde
<?btxt+toml filename='serde_example/Cargo.toml' mode='overwrite' tag='serde' ?>
<?btxt+rust filename='serde_example/src/main.rs' mode='overwrite' tag='serde' cmd='cargo run --manifest-path serde_example/Cargo.toml' ?>

Serde is the only Trait (well, technically its several traits) mentioned here that is actually not part of the Rust standard library. However, serde is so universally leveraged that it might as well be. Serde is short for Serialize Deserialize, and its a library for taking any given rust type and converting into a standard format that can be then serialized into other formats -- JSON, XML, YAML, etc. 

Serde is infinitely better than Go's runtime JSON marshalling. It is, quite frankly, a reason to use rust for web apis by itself. It's must faster, easier to use, harder to screw up, and easier to extend, and it supports more encodings than go.

Since its not part of the standard library, first we need to add a few crates:

```bash
cargo add serde
cargo add serde-derive
cargo add serde-json
```

After that, our project's `cargo.toml` will look like this:
```toml
[package]
name = "serde_example"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
serde = "1.0.160"
serde_derive = "1.0.160"
serde_json = "1.0.96"
```

Then we need to import the traits in order to use them.

```rust
use serde_derive::{Deserialize, Serialize};
use serde_json::json; // for the json! macro
```

Now we can define some struct that we'll marshal and unmarshal (using go jargon) into and from JSON.
<?btxt+rust mode='append' ?>

```rust
#[derive(Serialize, Deserialize)]
enum Gender {
	#[serde(rename="female")]
	Female,
	#[serde(rename="male")]
	Male
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Person {
	first_name: String,
	last_name: String,
	age: usize,
	gender: Gender
}
```

A type can have Serialize and Deserialize _derived_ as long as all fields implement Serialize and Deserialize. Serde implemented both for all standard types, so we only have to implement it for our custom ones, which we're also just deriving. 

Note how we rename the enum fields -- this is to prevent issues with serialization into "Female" instead of "female".

```rust serde1
fn main() {
	let person = Person{
		first_name: "Zach".into(),
		last_name: "Bullough".into(),
		age: 34,
		gender: Gender::Male
	};
	println!("{}", serde_json::to_string(&person).expect("failed to parse json"));
	// sample of output:{"first_name":"Zach","last_name":"Bullough","age":34,"gender":"male"}
	// And we can convert from json into a struct. 
	// let's use one of rust's and serde's super powers -- the json! macro
	let gender = Gender::Female;
	let person: Person = serde_json::from_value(json!({
		"first_name": "Bob",
		"last_name": "Marley",
		"age": 5,
		"gender": &gender// we're gender bending this, apparently
	})).expect("failed parsing json as Person");
	// We can't pass in invalid types
	if let Err(err) = serde_json::from_value::<Person>(json!({
		"first_name": "Bob",
		"last_name": "Marley",
		"age": -4,
		"gender": "male", // trailing commas allowed
	})) {
		println!("Got error as expected: {}", err);
	};
	// We can't have extra fields
	if let Err(err) = serde_json::from_value::<Person>(json!({
		"first_name": "Bob",
		"last_name": "Marley",
		"age": 60,
		"gender": "male",
		"something": true
	})) {
		println!("Got error as expected: {}", err);
	};
}
```

Lots to unload here. First, we dont have to define the logic for unmarshalling or marshalling our types, similar to go. Unlike go, this is all done at compile time. That means if something won't work, we'll know before we launch to production (or even compile it). 

For instance, if we had a typo in one of our "annotations" in go, it'd compile just fine... and only fail at runtime. If we did the same thing here, for instance, if we had `#[serde(renam="test")]` we'd get an error at compile time saying `error: unknown serde variant attribute 'renam'

Two, the `json!` macro allows you to embed json literals, not just strings, in your Rust code. Again, this is all done at _compile_ time, and so if we have invalid JSON, we'll get a compile error, and not something when we fail to parse a plain string at runtime in go. It's also just way more convenient than dealing with multiline strings and no syntax highlighting. You can also pass in variables into the `json!`, as we pass in `&gender` here. So no need for string interpolation. 

While not shown here, because Serde is so ubiquitous, every web server framework in rust is going to be integrated with it, and that means that if a web request fails to unmarshal into the type you set as the content body, it'll _automatically_ return a meaningful 400 error for you. Nice.

Lastly, none of these types are public. None of the fields are public. In go, you have to have public fields to reflect over them, so you must export all your json types. Yuck.

Starting to see the light, yet?

#### Clone

Clone is a very simple trait. Similar to `Serialize` from `serde`, it is almost always simply derived for a type:

```rust
#[derive(Clone)]
struct Foo {}
```

Clone implements a way of copying a value. This is _expensive_ (or at least more so than a move) and so Rust does _not_ copy values for you automatically. You must tell rust to copy it by using `clone()`. 

While you learn rust, you'll find yourself cloning things left and right. You will likely wonder if that's normal. The answer is yes, but also no. It is normal -- your source in Go that didn't pass by reference was also copying. And some level of cloning ends up being inevitable. However, as you get more experience you'll discover ways to restructure your code to eliminate these clones.

Don't lose sleep over clone, though. Use it when it solves your problem. There are bigger fish to fry.

#### Drop

We've mentioned "dropping" scopes and dropping lifetimes many times throughout this module so far. This is intentional -- when a scope returns, or ends, all the values it owned are _dropped_. This is the mechanism Rust uses for "compile time" garbage collection -- codified in a trait so you can implement it for your types. 

In a certain light, `drop` is similar to Go's `defer`, which executes a function when the function returns. Drop is also a function (the trait only has one method), and when a function returns, it is also called. However, drop is called on more than just function returns, and `defer` in go is very limiting.

To study drop, lets implement a fake, very simplified sql transaction in code -- you start a transaction, pass it along through functions, and when its done, it either commits or rollsback. 

If your transaction in go only ever spans one function call, its easy enough to just start a transaction and then immediately defer its commit or rollback, like this:

```go
func (r repo) updateUsers(users []User) (err error) {
	txn, err := r.BeginTxn()
	if err != nil {
		return err
	}
	defer func() {
		if err != nil {
			_ = txn.Rollback()
		} else {
			return txn.Commit()
		}
	}()
	// execute various sql statements below
}
```
This is pretty elegant, as far as it goes. However, there is a major weakness: I can't integrate this behavior into a type. I must remember to implement this defer each time I begin a transaction. If I forget, then my behavior will not be as desired. This makes it difficult to integrate into a library, since a library can't define `defer` for its callers.

So, let's try implementing `Drop` in a rust type that'll represent our transaction. 
<?btxt+rust mode='overwrite' ?>

```rust 
struct Txn {
    committed: bool,
}

struct Error();

impl Drop for Txn {
    fn drop(&mut self) {
        if !&self.committed {
            // rollback the txn when its dropped
            println!("Rollback now");
        } else {
	        println!("committed");
        }
    }
}

impl Txn {
	fn new() -> Self {
		Txn{
			committed: false
		}
	}
    fn commit(&mut self) -> Result<(), Error> {
        self.committed = true;
        Ok(())
    }
    fn exec(&self, statement: String) -> Result<(), Error> {
        Ok(())
    }
}
```

So, here we define our type, `Txn` and implement the `Drop` trait, which just has one method. When whatever scope owns our instance of `Txn` is dropped, `drop` will be called. This means that anything can create an instance of `Txn` , pass it around, and the txn will be rolled back if we don't commit it.

Of note, Go has a unique advantage here, in its functional patterns, to be able to easily also _commit_ automatically... however we must remember to implement `defer` in each case. Rust can't easily replicate this behavior, but we gain other advantages (the ability to define `defer` like behavior in a *type* that can be passed around).

In this sense, `Drop` is very similar to a deconstructor in OO languages. However, note that in most langauges with a deconstructor, you could not use a deconstructor to commit or rollback the transaction because the deconstructor is called when its garbage collected -- and that is not a definitive moment. Rust's drop is called _immediately_ when the object is about to be dropped from scope and removed from memory.

So, how would we go about using our new type? Here's an example use case. 
<?btxt+rust mode='append' ?>

```rust drop1
fn main() {
	let mut txn = Txn::new();
	update_users(txn)
}

fn update_users(mut txn: Txn) {
	get_users(txn);
	println!("end: update_users");
}

fn get_users(mut txn: Txn) {
	txn.commit();
	println!("end: get_users");
}
```
> Question for the audience: Why do we need `mut` here, in `let mut txn`?

Where do you think `committed` is printed?

#### Default

Since Rust doesn't set values to their zero values implicitly like Go, this means you must explicitly set variables to a value _all the time_. Generally, this is not a problem, as you can just set them to sensible defaults explicitly in the constructor. But, if you are working with types that do not have a constructor, or if you want to reduce the burden of typing all the zero values out in your constructor code, Rust has the `Default` trait.

Like many standard traits, `Default` can be derived for a type. 

```rust
#[derive(Default)]
struct Foo {
	bar: String, // will default to ""
	baz: usize // will default to 0
}

impl Foo {
	fn new() -> Self {
		..Default::default()
	}
}
```

#### From/Into

> “Bad programmers worry about the code. Good programmers worry about data structures and their relationships.”
> - Linus Torvalds

A perfectly defined program is nothing but converting one type into another type, with side effects (e.g. writing to disk, sending over a wire). Thus, the most important thing you can program is your types, and the second most important thing is how to convert one type into another.

A lot of what makes rust so readable, despite a rather heavy syntax, is [From](https://doc.rust-lang.org/rust-by-example/conversion/from_into.html). When you use the `?` operator, Rust will convert an `Err<E>` of any expression into the resturn type of the function, `Err<R>`, based on implementations of the `From` trait. 

Any type that implements `From` automatically gets an inverse relationship defined for `Into`. Let's look at an example:
<?btxt+rust mode='overwrite' ?>

```rust from1
#[derive(Debug)]
struct ErrorA {
	message: String
}

#[derive(Debug)]
struct ErrorB {
	message: String
}

impl From<ErrorA> for ErrorB {
	fn from(err: ErrorA) -> ErrorB {
		ErrorB{
			message: err.message
		}
	}
}

fn do_a() -> Result<(), ErrorA> {
	Err(ErrorA{
		message: "a".into()
	})
}

fn do_b() -> Result<(), ErrorB> {
	do_a()?;
	Err(ErrorB{
		message: "b".into()
	})
}

fn main() {
	println!("{:?}", do_b().unwrap_err());
	let err_a: ErrorB = ErrorA{message: "a".into()}.into();
}
```

So, we have a function `do_b()` which returns a result of `ErrorB` type. And that function calls `do_a()` which returns a different error.  But because we defined how to create an `ErrorB` from an `ErrorA` using `From`, we can use `?` and we don't have to manually check the error and create a new `Err(ErrorB)`. 

In our main function, we cann `do_b()` which returns an error with message `a` and _not_ `b` because the first line of `db_b()` is going to return the error from `do_a()`. In the next line we show that for any type that implements `From` we get an automatic `Into` implemented -- we never defined `ErrorA.into()` but we are able to use it.

Using `from` and `into` methods is _very_ common in Rust. Both explicitly and implicitly (using `?` and other special cases). Not how we convert our `"a".into()` a String as well. 

Lastly, note that `From` and `Into` both require _ownership_ of the type they are converting. They consume the input, so you can't use the source after conversion without explicit duplication. For this reason, `From` is usually very efficient as it moves data around instead of duplicating it. 
 
#### The Orphan Rule

Old school developers may be wary of Traits -- one of the worst things about interfaces allowing you to define how operators work, or in general, is that you can change code behavior in unexpected ways. You can suddenly make `+` do something _completely_ unexpected by importing a library or something similar. 

Rust avoids this problem entirely by enforcing the orphan rule. The orphan rule says that you can only implement a Trait for a Type, if you defined either the Trait or the Type _in the crate_ where the implementation is defined. This means that I can't just implement the `+` operator for `Vec` because I defined neither `Vec` nor `Add`. There are also [coherence problems](https://github.com/Ixrec/rust-orphan-rules) without the orphan rule.

The orphan rule, combined with a lack of _trait specialization_ will, at some point, prevent you from being able to elegantly solve a problem in Rust, at the intermediate level. It is unfortunate (although trait specialization is being worked on and may be added at some point), but it is necessary to keep behavior predictable. It is an inconvenient but necessary trade off.

### Generics 
<?btxt+rust mode='overwrite' tag='generics' pre='' post='' ?>
<?btxt+go ignore=true ?>

Generics are, simply put, a way to write code that is DRY but needs to be created, verbatim, for many different types, in exactly the same way. It is essentially a template using syntax. The basics of it are quite simple. Consider the following go code:

```go
func addUint(a, b uint) uint {
	return a + b
}

func addInt(a, b int) int {
	return a + b
}

func main() {
	println(addUint(0, 3))
	println(addInt(1, 3))
}
```

If we wanted to create a function that can add two of _any_ type of numbers, we'd have to manually write all these functions out. This isn't dry, and it drastically increases the probability of a programmer making a stupid mistake as the number of functions increases, and it rapidly becomes unreadable. 

Generics simply allow us to define a function that can take any combinations of anything and the compiler builds out the code for us, for each type we use. Consider the following pseudocode:

```go
func addNumber<T>(a, b T) T {
	a + b
}

func main() {
	println(addNumber(0, 4))
	println(addNumber(1.0, 4.5))
}
```

This shows a snippet of how generics _might_ work in go (but it isn't, actually). Basically, we define a template, where the function, at compile time, is built for each type `T` that calls it. So, the compiler here will generate two different functions:

```go
func addNumber(a, b int) int 
func addNumber(a, b float32) float32
```

This is, at its core, generics. A way of letting the compiler take care of writing all the different signatures you'd need to handle all the types you pass.

But, Rust is a statically, strictly typed language. Let's say I define a function like above in rust:

```rust
fn addNumber<T>(a: T, b: T) -> T {
	a + b
}
```

This seems like it should work, but it won't. Because `T` could be _absolutely anything_. Not every possible type can be added. What is, for instance, `() + ()`? Unbounded generics are _completely useless_ because you can't do anything with it.

For an analogy in go, consider the following:

```go
func addAnything(a, b interface{}) {
	a + b
}
```

This is essentially the same thing. Go won't let us compile this because you can't add `interface{}` since it could be absolutely anything, including `nil`, and you can't add `nil + nil`. 

So, for generics to be actually _useful_ you need to be able to give them _bounds_. Things, in life and in programming, are best defined by their limitations. In order to be able to use the `+` operator, we must only accept types that implement the [Add](https://doc.rust-lang.org/std/ops/trait.Add.html) trait.

```rust add1
use std::ops::Add;

fn addAnything<T>(a: T, b: T) -> T where T: Add<Output = T> {
	Add::add(a, b)
}

fn main() {
	println!("{}", addAnything(0, 5))
}
```

There are two ways to define generic trait bounds, I prefer the `where` syntax shown here, because I find it easier to read. What we're saying he is this function is generic over T, where T must implement Add, and the Output (type) of Add::add must also be T. 

`Output` is a type on the trait. You can see the definition [here](https://doc.rust-lang.org/std/ops/trait.Add.html).

Another useful element of generics is the ability to have more compile time checks than you can with go's runtime reflection. Consider the following common pattern in Go:

```go
func fatal(message string, args ...interface{}) {
	fmt.Fprintf(os.Stderr, message, args...)
	os.Exit(1)
}
```

Here, we pass a format string to a print statement and then the things being formatting are just... anything. Literally anything. Go can't tell in this case what's going to happen if you pass something with a type that can't be printed, until runtime. If I pass an empty struct, what will happen? Who knows?

We can solve this problem easily with rust (except rust doesn't support variadic parameters, which is why these things are macros in rust):

```rust 
fn fatal<T>(message: String, arg: T) where T: std::fmt::Display {
	//
}
```

### Futures and async

<?btxt+rust mode='overwrite' filename='tokio-example/src/main.rs' cmd='cargo run --manifest-path tokio-example/Cargo.toml' tag='async' pre='' post='' ?>
<?btxt+javascript tag='async' ?>
[Let's talk about Javascript](https://www.destroyallsoftware.com/talks/wat). Nowadays, writing async code in javascript consists of `asnyc/await` but it wasn't always that way. Before the syntactic sugar of those two keywords became standard, functions that wanted to support asynchronous behavior returned _promises_, which were then polled by the javascript runtime to complete.

Rust has something similar to promises, called a `Future`. Note how above I mentioned that javascript promises are polled by the JS runtime. Rust has no runtime. So how can it poll it? That's actually exactly why they are called futures and not promises. A future _may_ come true, but only if you do something about it. A promise (in JS anyway) will complete, even if you immediately forget about it. 

We aren't going to spend a lot of time implementing `Future` types -- just know that they're generic types, like `Result` built into the standard library. They are in fact a generic trait, since they require you to implement a method (poll). I consider implementing your own futures to be an intermediate topic, and is generally a waste of brain power at this stage. 

The reason we don't need to really go over futures is that Rust also has `async/await` syntactic sugar, just like Javascript. The primary difference is, because Rust _doesn't_ have a runtime, you have to provide your own. Most of the time, you'll be using `tokio`. Usually, as in later modules of this training, you will indirectly use tokio via async code supported by a web framework or database library. There are other async runtimes available, but for our purposes, there is no need to investigate them -- tokio is the de facto, general purpose standard. 

#### Tokio and Async Await

Let's take a look at a sample project to spawn some green threads (similar to goroutines) and collect their output, using tokio and async/await.

```rust tokio1
#[tokio::main]
async fn main() {
	let handle = tokio::spawn(async {
		"done"
	});
	let result = handle.await.unwrap();
	println!("RESULT: {}", result);
}
```

Any block or function can be marked as async, where `async` is actually a macro that expands the block into a block or function that returns a Future. This means the return type of this block is not `&str` but is actually a `Future`. This means you _cannot_ pass around the result as a `&str` because `Future` is a different type. You must `await` it, and then deal with the Result (execution could fail). Then you are presented with the result of the actual execution -- in this case a `&str`.

#### Fearless Concurrency

We briefly went over how rust prevents shared memory issues or race conditions when writing parallel code, but lets actually look at an example. Consider a situation where you need to share a map across threads. In go, you shouldn't just use a `map`, but nothing will actually stop you. Here's what happens in rust:

```rust asyncbad1
use std::collections::HashMap;
#[tokio::main]
async fn main() {
	let mut map: HashMap<String, String> = HashMap::new();
	let handle = tokio::spawn(async {
		map.insert("test".into(), "value".into());
	});
	handle.await;
}
```

Trying to compile this code gives a long, but very helpful error:

```
error[E0373]: async block may outlive the current function, but it borrows `map`, which is owned by the current function
 --> src/main.rs:5:28
  |
5 |       let handle = tokio::spawn(async {
  |  _______________________________^
6 | |         map.insert("test".into(), "value".into());
  | |         --- `map` is borrowed here
7 | |     });
  | |_____^ may outlive borrowed value `map`
  |
  = note: async blocks are not executed immediately and must either take a reference or ownership of outside variables they use
help: to force the async block to take ownership of `map` (and any other referenced variables), use the `move` keyword
```

`map` is dropped at the end of `main()`, but our async code isn't always going to drop with it (ownership of the block belongs to `tokio::spawn` and we don't know what that will do with it). Since we don't have control over the ownership mechanics of `tokio::spawn` we effectively must always `move` ownership into the block. We can accomplish this easily enough:

```rust asyncbad2
use std::collections::HashMap;
#[tokio::main]
async fn main() {
	let mut map: HashMap<String, String> = HashMap::new();
	let handle = tokio::spawn(async {
		map.insert("test".into(), "value".into());
	});
	handle.await;
}
```

So, this code will compile! Everything is perfectly safe -- since `map` is moved into the thread, we can't possibly have any conflicts. However, that means our host thread _also_ can't do anything with it, since the value was moved out of the main scope. 

We need to be able to split ownership between two different scopes. One in `main` and the other in the `async` block. Let's try a `Rc`!

```rust asyncbad3
use std::collections::HashMap;
use std::rc::Rc;

#[tokio::main]
async fn main() {
	let map: HashMap<String, String> = HashMap::new();
	let map = Rc::new(map); // shadow map
	let handle = tokio::spawn(async {
		map.insert("test".into(), "value".into());
	});
	handle.await;
	println!("{}", map.get("test").unwrap());
}
```

Now we get another long, but very clear error message:

```
error[E0277]: `Rc<HashMap<String, String>>` cannot be shared between threads safely
   --> src/main.rs:9:28
    |
9   |       let handle = tokio::spawn(async {
    |  __________________------------_^
    | |                  |
    | |                  required by a bound introduced by this call
10  | |         map.insert("test".into(), "value".into());
11  | |     });
    | |_____^ `Rc<HashMap<String, String>>` cannot be shared between threads safely
    |
    = help: the trait `Sync` is not implemented for `Rc<HashMap<String, String>>`
    = note: required for `&Rc<HashMap<String, String>>` to implement `Send`
note: required because it's used within this `async` block
   --> src/main.rs:9:28
    |
9   |       let handle = tokio::spawn(async {
    |  _______________________________^
10  | |         map.insert("test".into(), "value".into());
11  | |     });
    | |_____^
note: required by a bound in `tokio::spawn`
   --> /home/zach/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.28.0/src/task/spawn.rs:163:21
    |
163 |         T: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
```
> Get used to long, clear error messages. If you take the time to carefully and fully read them, they can teach you how to write rust. They might be the best error messages in any compiler ever.

Basically, the key part of this error is `Rc<...> cannot be shared between threads safely`. There are two traits, here, that must be implemented on a type for it to work here. The first is `Send`, which means that a type can be sent across threads safely, and also `Sync` which means it can be _used_ across threads safely. Rust won't let us use unsafe types!

> Send and Sync are both _auto traits_ which means you can't implement them. The compiler automatically implements them when it can tell its appropriate.

Great, but what do we do? Well, in Go we would use a mutex. A mutex will allow us to mutate our value, but it won't allow us to share a value across multiple owners. For that we still need a reference counter -- but Rc isn't threadsafe! So, we'll need an `Atomic Reference Counter`, or `Arc`. Arcs function the same as Rc except that each reference count operation is atomic, and thus you can't have a situation where a race condition occurs on the increment or decrement of the count operation.

```rust async1
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
	let map: HashMap<String, String> = HashMap::new();
	let map = Arc::new(Mutex::new(map)); // shadow map
	let handle_map = map.clone();
	tokio::spawn(async move {
		handle_map.lock().unwrap().insert("test".into(), "value".into());
	}).await.expect("expected future to complete successfully");
	println!("{}", map.lock().unwrap().get("test").unwrap());
}
```

Okay, so now we have a completely functional and thread safe program! It is, admittedly, a little cluttered compared to a similar go program, but we don't have to worry _at all_ about issues due to forgetting to lock a mutex, or unlock one, or race conditions or undefined behavior. I'd say that's a fair trade, personally. 

For reference, here is a similar node script to accomplish the same thing:
```javascript promise1
const main = async () => {
	const map = {};
	await (async () => {
		map["test"] = "value";
	})();
	console.log(map["test"]);
}
Promise.resolve(main());
```

Note, however, that nodejs is locked to a single thread, so we don't have to worry about race conditions, here, or shared memory.

## Batteries Not Included

Aside from the syntax and potential new concepts, the biggest surprise for a developer coming from go (or most languages, really) is that rust's standard library is surprisingly, perhaps even shockingly, lean. 

Go has, in contrast, possibly the most impressive standard lib outside of Java. 

Of note, Rust's standard library doesn't have:

- Random number generation support
- Async runtime
- DB drivers or framework
- Serialization or parsing libraries

The standard library actually avoids functionality altogether -- it provides types and critical interactions with the system. Logic, even elementary logic, has to come from an external crate.

The reasons for this, according to the rust developers, is that in older languages, the standard library becomes a graveyard. Old patterns, types, and packages that no one uses anymore, but must exist for backwards compatibility reasons. And certainly, if you look at Python, Java or C++, this problem is abundant. Even Go has parts of this, and its not even that old. 

So, for better or worse, you will end up importing external libraries for essentially _everything_. 

## Superpowers

So, at this point you are probably feeling overwhelmed. Trust me when I say, no one has ever claimed rust is _easy_. Rust is crazy hard. It's just also worth it. We've mentioned a few things along the way that you get from all of this complexity, that you don't get from anything else.

This section is just a small showcase of things that exist in rust that, AFAIK, don't (and probably _can't_) exist anywhere else.

- [Shuttle](https://www.shuttle.rs/)
	- Define infrastructure needs _in rust code_ as macros
	- Deploy to the cloud using `cargo`
	- Need a database? Reference it in code and it will get created on deploy 
- [Bevy](https://bevyengine.org/) auto [concurrency](https://bevy-cheatbook.github.io/programming/ecs-intro.html#performance)
	- Because Rust's type system can express so much, Bevy (a game engine) will _automatically_ detect what game logic has no dependencies on other elements and run them in parellel. This is decided at _compile time_.
- Sqlx and Diesel 
	- We'll work with both of these in the training.
	- [Diesel](https://diesel.rs/) is an ORM that knows and checks (again, _at compile time_) that every type you try to insert into a database is perfectly compatible with that datatype. E.g. is the SQL column nullable? Gotta be an `Option`. 
	- [SQLX](https://github.com/launchbadge/sqlx) can do the same thing, except it checks raw _queries_ at compile time to make sure they have valid types and structure. 
- [Tower](https://www.reddit.com/r/rust/comments/13386q1/comment/ji8hzy0/?utm_source=share&utm_medium=web2x&context=3)'s network services don't care what protocol they run on. 
- [RSX](https://github.com/victorporof/rsx) and [JSX](https://yew.rs/docs/next/concepts/html)
	- Write JSX-like syntax in rust. Try doing that in go (seriously, try)
```
	html! {
		<div>
			<span>{"This is valid Rust"}</span>
		</div
	}
```

