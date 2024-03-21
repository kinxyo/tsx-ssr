# I'm making a framework not a library.

## AI suggestions

1. Inversion of Control:  Ensure your framework dictates the program flow and calls the developer's code at designated points. This can be achieved through traits, callbacks, or plugins.

2. Extensibility: Aim to design your framework for extensibility. Allow developers to add custom functionalities without modifying the core framework code.

3. The key is to find the right balance between structure and extensibility for your specific use case.

---

## My way

1. I need to let user decide which file or component to serve at which request. Initially, I was literally configuring a setup, that was trying to automatically buiding and serving files based on configurations. It was like a library for starting up React frontend. That's why I was clueless regarding serving different files to different routes OR what more features could possibly be added. **I'm providing a 'way of writing', not a collection of functions**. This naturally requires more planning and designing.
