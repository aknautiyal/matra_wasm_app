# Matra Analysis Web App (`matra_wasm_app`)

## Overview

The **matra_wasm_app** is a web-based application that uses WebAssembly (Wasm) to analyze Devanagari text. Built on top of the [**matra_lib**](https://github.com/aknautiyal/matra_lib) Rust library, this app enables users to analyze syllabic patterns (matra), decompose text into its constituent components, and display structured insights in real time. It is particularly useful for those studying or composing poetry in the classical Indian tradition, such as Matrik Chhand.

## Features

- **Matra Analysis in the Browser**: Leverages WebAssembly to run Rust code directly in the browser for fast and efficient analysis.
- **Detailed Insights**: Breaks down input lines into their Varns (characters) and their respective Matra values, showing comprehensive line-level statistics.
- **Interactive Interface**: Provides an easy-to-use text box for entering Devanagari lines, with results displayed dynamically.
- **Cross-Platform**: Runs seamlessly on any modern browser without the need for additional installations.
- **Powered by matra_lib**: Uses the core functionalities of matra_lib for accurate matra analysis and text decomposition.

## Demo

You can try the live demo hosted on GitHub Pages: [matra_wasm_app](https://aknautiyal.github.io/matra_wasm_app/) Demo

## Getting Started

### Prerequisites

To set up and run the app locally:

- Install [Rust](https://www.rust-lang.org/) and the `wasm-pack` tool:
    
    ```bash
    cargo install wasm-pack
    ```
    
- A basic understanding of WebAssembly and how Rust integrates with it.

### Running Locally

1. Clone the repository:
    
    ```bash
    git clone <https://github.com/aknautiyal/matra_wasm_app.git
    cd matra_wasm_app
    ```
    
2. Build the WebAssembly package:
    
    ```bash
    wasm-pack build --target web
    ```
    
3. Serve the app locally:
    
    ```bash
    python -m http.server 8000
    ```
    
4. Open your browser and navigate to `http://localhost:8000` to test the app.

### Deployment to GitHub Pages

1. Switch to the `gh-pages` branch:
    
    ```bash
    git checkout gh-pages
    ```
    
2. Copy the contents of the `pkg` folder and the `index.html` file into the branch.
3. Push the changes to GitHub:
    
    ```bash
    git push origin gh-pages
    ```
    
4. Enable GitHub Pages in your repository settings and set the branch to `gh-pages`.

## Usage

1. Enter a Devanagari text line into the provided text box on the webpage.
2. Press the "Analyze" button to view:
    - **Varns**: The characters in the line, grouped by words.
    - **Matra**: The corresponding syllabic values.
    - **Total Counts**: Statistics like the total number of Varns and Matra for the entire line.
3. Explore results displayed in a structured and formatted manner.

## Built With

- **Rust**: Core logic and analysis.
- **wasm-pack**: To compile Rust code to WebAssembly.
- **JavaScript**: For interacting with the compiled WebAssembly module.
- **HTML/CSS**: For the user interface.
- **matra_lib**: The Rust library powering the matra analysis.
