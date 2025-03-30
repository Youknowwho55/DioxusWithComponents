<!-- @format -->

# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

### Tailwind

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform desktop
```

dx serve --platform web

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```

# DioxusWithComponents

## Tailwing

npm install tailwindcss @tailwindcss/cli
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss -i ./assets/tailwind.css -o ./assets/tailwind-output.css --watch
const TAILWIND_CSS: Asset = asset!("/assets/tailwind-output.css"); // Updated path

## Prompt

"Please provide the props for a [XXXX] component, including:

1. Core Functionality (Essential props for basic operation)
2. Visual Customization (Styling, variants, sizes)
3. Behavior (Event handlers, interaction logic)
4. Composition (Slots, children, compound components)
5. Accessibility (ARIA attributes, keyboard nav, labels)
6. DOM Extensions (Class, style, ID, data-\* attributes)
7. Documentation Examples (3-5 common usage examples)

- [ ] Props documented using standard template
- [ ] Uses shared design tokens
- [ ] Includes basic examples
- [ ] Accessibility attributes considered
