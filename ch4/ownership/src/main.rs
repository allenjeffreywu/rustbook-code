fn main() {
    // ownership model, completely different from garbage collection and memory management you may be used to
    // Stack and Heap comparch review
    // Stack = stores values in the order it gets them (hence the name) LIFO. All data here has a fixed size
    // Heap = all data here has an unknown size. malloc for space, returns a pointer. You can store this pointer on the stack, but the actual data is on the heap
    // as usual, heap is slower than the stack

    // ownership rules
    // each value in rust has an owner
    // There can only be one owner at a time
    // when the owner goes out of scope, the value will be dropped

    // variable scope
    // this variable is in scope for the entire main function after it is declared here
    let s = "hello";

    // the String type
} // s is now no longer in scope.
