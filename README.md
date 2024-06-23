# Package Checker

## Overview
The Package Checker is a Rust application designed to manage npm package dependencies by checking current project versions against the latest versions available on npm. <br><br>

It reads the `package.json` file from a given project, extracts the dependencies, and compares each dependency version with its latest available version on npm. Additionally, the application plans to introduce functionality to recommend newer packages with similar capabilities.

## Features

- **Current Version vs. Latest Version Comparison**: The application fetches the latest version information for each package listed in the `package.json` dependencies and compares it with the current version used in the project.
- **Future Feature - Similar Package Recommendations**: In future updates, the application will also be able to recommend newer packages that offer similar functionality but may not be in the user's current package list.

## Requirements

- Rust programming environment.
- Access to the internet to fetch package data from npm.
- An API key from Libraries.io for accessing detailed package data.

## Installation

1. Clone the repository:
   ``bash
   git clone <repository_url>
   ``
2. Navigate to the cloned directory:
   ``bash
   cd <directory_name>
   ``
3. Set your API key from Libraries.io in a `.env` file:
   ``bash
   echo "API_KEY=your_api_key_here" > .env
   ``
4. Build the project with Cargo:
   ``bash
   cargo build
   ``

## Usage

Run the application using Cargo:
``bash
cargo run
``

The application expects a `package.json` file at the specified location (`./package.json`). Ensure that this file is present in the root directory or modify the `file_location` in the source code as needed.

## Contributing

Contributions are welcome! If you have suggestions for improving the application or adding new features, please open an issue or a pull request.

## License

Specify your license or state that the project is licensed under the MIT License, allowing others freely to use, modify, and distribute the software.

## Acknowledgments

Thank the contributors and mention any third-party libraries or APIs you are using.
