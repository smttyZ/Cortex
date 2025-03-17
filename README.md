![Cortex](https://socialify.git.ci/smttyZ/Cortex/image?custom_description=A+server+built+using+rust+to+breathe+some+new+life+into+older+systems.&description=1&font=Inter&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Light)

# DISCLAIMER
I am not a good programmer (yet), and this entire repo is probably filled with bugs, contradictions, and lots of imperfections. It's a work in progress, but I hope you'll stick around!

# Getting Started
### Prerequisites
* [Rust](https://www.rust-lang.org) (Recommended: Latest stable version)
* [npm](https://www.npmjs.com) (Installed with [Node.js](https://nodejs.org) - Recommended: v18+)

---

# Instructions for Linux
* Warning! This project is not even close to having an official release, but i tried my best here. Once an official release is out, i promise these will get better.
1. Clone the repository.
   ```
   git clone https://github.com/smttyZ/Cortex.git
   ```
2. Cd into the "web-client" folder.
   ```
   cd ./Cortex/web-client
   ```
3. Run the ```npm install``` command.
4. Copy the ```index.html``` file from "/web-client/src/" into the "/web-client/dist/" folder.
   ```
   cp src/index.html dist/index.html
   ```
5. Build the frontend.
   ```
   npx esbuild src/index.tsx --bundle --outfile=dist/index.js --format=esm
   ```
6. Head to the 'mentis' directory.
   ```
   cd ../mentis
   ```
7. Compile and run the Rust server.
   ```
   cargo run
   ```
8. Done!


# Q&A
### Why?
To anyone who's ever had a decade old computer they just can't get rid of, this is your solution. Whether it's because you have memories with it, or you're just too lazy this project aims to breathe some new life into that hardware!

### Ok, but why Rust?
I personally think Rust is the new C. With memory management and a compiler that refuses to let you write unsafe code, it's relatively easy to write highly optimized programs. Which is exactly what's these older systems need.

### Why does it suck so much, and when will it get better?
Soon™.
I'm just a student, and i can't work on this full time. But the hope is that someone sees this and i can save at least one computer from getting tossed out. The less E-waste, the better.

### How can i help?
I'll admit, I'm not great at working with others. But if you make a pull request, i'll *try* to look at it. No promises. You can also reach out to me for just about any reason here!
