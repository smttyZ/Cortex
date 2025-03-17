![Cortex](https://socialify.git.ci/smttyZ/Cortex/image?custom_description=A+server+built+using+rust+to+breathe+some+new+life+into+older+systems.&description=1&font=Inter&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Light)

# DISCLAIMER
I am not a good programmer (yet), and this entire repo is probably filled with bugs, contradictions, and lots of imperfections. It's a work in progress, but I hope you'll stick around!

# Getting Started
=======
### Prerequisites
* [Rust](https://www.rust-lang.org) (Recommended: Latest stable version)
* [npm](https://www.npmjs.com) (Installed with [Node.js](https://nodejs.org) - Recommended: v18+)

## Instructions for Linux

> **Warning!** This project is not even close to having an official release, but I tried my best here. Once an official release is out, I promise these will get better.

1. Clone the repository.
   ```bash
   git clone https://github.com/smttyZ/Cortex.git
   ```

2. Navigate to the "web-client" folder.
   ```bash
   cd ./Cortex/web-client
   ```

3. Install Node.js dependencies.
   ```bash
   npm install
   ```

4. Set up the frontend build:
   ```bash
   # Create the dist directory if it doesn't exist
   mkdir -p dist
   
   # Copy the index.html file to the dist folder
   cp src/index.html dist/index.html
   
   # Build the React application
   npx esbuild src/index.tsx --bundle --outfile=dist/index.js --format=esm
   ```
   > Note: This will generate a large index.js file in the dist directory. This file is gitignored and needs to be generated locally.

5. Navigate to the 'mentis' directory.
   ```bash
   cd ../mentis
   ```

6. Compile and run the Rust server.
   ```bash
   cargo run
   ```

7. Server configuration options:
   - Default: `cargo run` will use localhost (127.0.0.1) on port 3001
   - Custom host: `cargo run 0.0.0.0` will bind to all interfaces on port 3001
   - Custom host and port: `cargo run 127.0.0.1 8080` will use localhost with port 8080

## Q&A

### Why?
To anyone who's ever had a decade-old computer they just can't get rid of, this is your solution. Whether it's because you have memories with it, or you're just too lazy, this project aims to breathe some new life into that hardware!

### Ok, but why Rust?
I personally think Rust is the new C. With memory management and a compiler that refuses to let you write unsafe code, it's relatively easy to write highly optimized programs. Which is exactly what these older systems need.

### Why does it suck so much, and when will it get better?
Soon™.
I'm just a student, and I can't work on this full time. But the hope is that someone sees this and I can save at least one computer from getting tossed out. The less e-waste, the better.

### How can I help?
I'll admit, I'm not great at working with others. But if you make a pull request, I'll *try* to look at it. No promises. You can also reach out to me for just about any reason on Discord: 
* smittyzzz
=======
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
8. Running `cargo run` will use localhost on port 3001, to change this you can add arguments like this.
   ```
   cargo run 0.0.0.0
   ```
   This will bind to all interfaces on port 3001 which is useful if you have a static IP and forwarded port.

# Q&A
### Why?
To anyone who's ever had a decade old computer they just can't get rid of, this is your solution. Whether it's because you have memories with it, or you're just too lazy this project aims to breathe some new life into that hardware!

### Ok, but why Rust?
I personally think Rust is the new C. With memory management and a compiler that refuses to let you write unsafe code, it's relatively easy to write highly optimized programs. Which is exactly what's these older systems need.

### Why does it suck so much, and when will it get better?
Soon™.
I'm just a student, and i can't work on this full time. But the hope is that someone sees this and i can save at least one computer from getting tossed out. The less E-waste, the better.

### How can i help?
I'll admit, I'm not great at working with others. But if you make a pull request, i'll *try* to look at it. No promises. You can also reach out to me for just about any reason on discord: 
* smittyzzz