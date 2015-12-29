# advent-of-rust
My solutions to Advent of Code challenges, written in Rust.

## Methodology
I found these challenges to be a good opportunity for getting acquainted with a new language. Therefore I decided to do all of them using Rust, which is a modern alternative to my currently favourite language, C++.

As the primary goal is on learning as many new concepts as possible, I have decided to follow two main principles:
* Whenever the problem is solvable in a better way than bruteforce, I will strive to implement the better approach.
* My goal is for the final revision to these solutions to reference nothing but the Rust standard library, as I would like to try and implement as many of the algorithms myself without relying on external APIs.

With that in mind, here is a list of the challenges where I have particularly applied the above:
* **Day 6**, where I have implemented the Quadtree data structure with lazy propagation for more efficient range updates.
* **Day 7**, where I have first implemented the graph data structure. This might sound simple, but Rust's borrows checker has made it a very demanding task, requiring usage of dynamic borrowing structures such as RefCell.
* **Day 9**, where I have implemented a dynamic programming approach to solving the shortest/longest Hamiltonian path problems, reducing the complexity from O(n!) to O(2^n n^2).
* **Day 12**, where I have implemented a custom parser (taking into account just the data required for solving the problem), rather than using an external JSON library.
* **Day 13**, where I have implemented a dynamic programming approach to solving the Travelling Salesman Problem (similarly to Day 9).

## License
MIT
