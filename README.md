# EasySSG

EasySSG is a static site generator that prioritizes simplicity and convenience. Built in Rust, it allows users to create fast, secure, and scalable websites using markdown files. With EasySSG, your site's structure mirrors your folder structure, making it intuitive to organize content and build your website.

## Features

- **Markdown Support:** Write your content in Markdown—a lightweight markup language with plain-text formatting syntax.
- **Folder Structure as Site Structure:** The hierarchy of your markdown files determines the structure of your website, simplifying navigation and organization.
- **Customizable:** Though designed for simplicity, EasySSG provides options for customization to suit your site's specific needs.
- **Fast and Lightweight:** Leveraging Rust's performance, EasySSG generates your site quickly, making it ideal for all sizes of projects, from small blogs to large documentation sites.
- **Cross-Platform:** Works on Linux, macOS, and Windows, ensuring your site can be built from any platform.

## Getting Started

To get started with EasySSG, you will need to have Rust and Cargo installed on your system. EasySSG can be easily integrated into your project to start generating your static site from markdown files.

### Creating Your Site

1. **Organize Your Content:** Arrange your markdown files in a folder structure that reflects your desired site structure.
2. **Configuration:** Customize your site's settings through a simple configuration file (optional).
3. **Build Your Site:** Run EasySSG to generate your site. Your static site will be ready to be served from any web server.

### Example Structure

```
/site
    /blog
        01_intro.md
        02_features.md
    /about
        index.md
```

This structure will result in a site with `/blog/01_intro`, `/blog/02_features`, and `/about` as navigable URLs.

## Usage

For detailed usage instructions and advanced configurations, please refer to the documentation.

## License

EasySSG is open-sourced software licensed under the AGPLv3 license. This ensures that all modifications and extensions remain free and open, contributing to the community-driven development of the project.

For more information on the license, please see [LICENSE](LICENSE).
