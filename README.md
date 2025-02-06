# Tailfrick

Tailfrick brings [Brainf*uck](https://brainfuck.org/brainfuck.html) to [Tailwindcss](https://tailwindcss.com/). Now you can enjoy the simplicity and power of a Turing-complete language right in your classes! Oh, and Tailfrick is written in Rust, so obviously it's 🔥 blazing fast.

Tailfrick, once installed, adds the `frick` utility to Tailwind. Simply write a Brainf*ck program that outputs the css properties you want, and place it between the square brackets like so:

```html
<div class="frick-[<your program>] hover:frick-[<your hover program>]">
```

Good luck!

[**See Tailfrick in action now on Tailwind play**](https://play.tailwindcss.com/wLRTvrOTp6)

## Installation

### Tailwind 4
1. Get Tailwind working
2. Install the Tailfrick plugin
    ```shell
    [npm|yarn|pnpm|bun|duno] [install|add|gimme] @tailfrick/tailwind
    ```
3. Load the plugin
    ```css
    /* near wherever you import tailwind */
    @import "tailwindcss";

    /* load the plugin */
    @plugin "@tailfrick/tailwind";
    ```

### Tailwind 3

~~TODO~~ left as an exercise for you, dear reader


## Contributing

Tailfrick is organized as a monorepo. The `core` is written in rust and can easily be extended or incorporated into future projects.

If someone knows how `package.json` `exports` are meant to work, please kindly keep that to yourself. It's perfect already.

## Notes

This is a very serious enterprise-grade project with extensive testing. It is ready for production use.

<details>
<summary>\s</summary>

Yes, it's a joke. No, it's not intended to be a commentary on tailwind, webdev, or anything like that. I happen to rather like tailwind, development, & the web. I had a silly idea and wanted to try wasm. That's all. 

</details