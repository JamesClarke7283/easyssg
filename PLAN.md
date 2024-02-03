# Lib Easy SSG

## Introduction

`libeasyssg` is the core library powering EasySSG, a static site generator designed for simplicity and convenience. This library will handle parsing Markdown files, generating HTML pages, and creating a Table of Contents (ToC) for site navigation. The goal is to enable users to easily create static websites with structured content and simple navigation.

## Core Features

1. **Markdown to HTML Conversion**
   - Implement functionality to parse Markdown files and convert them into HTML pages.
   - Ensure support for standard Markdown syntax and common extensions.

2. **Folder-Based Site Structure**
   - Develop a system to read and interpret the project's folder structure as the website's navigation hierarchy.
   - Generate HTML pages that reflect this structure, preserving folder names as URL paths.

3. **Table of Contents Generation**
   - Automatically generate a ToC for each page, enabling easy navigation across the site.
   - The ToC should dynamically reflect the site's structure, showing pages and subpages.

4. **Static HTML Generation**
   - Output the generated HTML pages into a specified directory, mirroring the original folder structure of the Markdown source files.

## Development Plan

### Phase 1: Setup and Markdown Processing

- **Task 1.1:** Set up the Rust project and integrate a Markdown parsing library.
- **Task 1.2:** Create a function to convert Markdown files to HTML.

### Phase 2: Site Structure and ToC

- **Task 2.1:** Implement functionality to read the folder structure and map it to site URLs.
- **Task 2.2:** Design and implement the logic for ToC generation based on the folder structure.

### Phase 3: HTML Output and Site Generation

- **Task 3.1:** Develop the system to output generated HTML files to the correct directories.
- **Task 3.2:** Ensure that the ToC is included on each page, providing navigation across the site.

### Phase 4: Testing and Refinement

- **Task 4.1:** Write tests for Markdown to HTML conversion, ToC generation, and site structure integrity.
- **Task 4.2:** Refine and optimize the code based on test results and initial user feedback.

### Phase 5: Documentation

- **Task 5.1:** Document the library's API, focusing on how users can generate their sites and customize the ToC.
- **Task 5.2:** Provide examples and best practices for organizing Markdown files to achieve the desired site structure.

## Future Considerations

- **Extensibility:** Plan for future features, such as support for custom templates, plugins, and additional Markdown extensions.
- **Performance Optimization:** Assess and optimize the performance for large sites with numerous Markdown files.
- **Community Feedback:** Open the project to community feedback to guide further development and improvements.
