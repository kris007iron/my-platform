# My-Portfolio Project

This repository contains the code for my portfolio, showcasing my skills as a backend developer. The frontend is created using vanilla JavaScript, with additional styling and animations using CSS. The backend is built with Rust using the Rocket framework and MongoDB for data storage.

## Project Structure

```plaintext
│   cors.rs
│   main.rs
│   routes.rs
│
├───cors
│       cors.rs
│
├───front-end
│   │   app.js
│   │   index.html
│   │   style.css
│   │
│   └───imgs
│           githublogo.png
│
└───routes
        projects.rs
```

- **cors.rs**: Contains the CORS (Cross-Origin Resource Sharing) implementation for adding headers to responses.

- **main.rs**: The main entry point for the Rocket web server.

- **routes.rs**: Defines the routes for the web server.

- **front-end**: Contains the frontend code, including HTML, CSS, and JavaScript.

  - **app.js**: JavaScript code for handling animations, typing effects, and fetching data from the backend.

  - **index.html**: The main HTML file for the frontend.

  - **style.css**: Cascading Style Sheets for styling the frontend.

  - **imgs**: Directory for images used in the frontend.

- **routes**: Contains route-specific Rust files.

  - **projects.rs**: Rust code defining routes related to projects.

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/my-portfolio.git
   ```

2. Configure MongoDB connection string:

   - Open `main.rs` and replace the placeholder in the `db_connection` function with your MongoDB connection string.

3. Install dependencies:

   ```bash
   cargo build
   ```

4. Run the server:

   ```bash
   cargo run
   ```

   The server will start at `http://localhost:8000`.

5. Open `http://localhost:8000` in your browser to view the portfolio.

## Frontend Code Highlights

### Animation on Scroll

The JavaScript code in `app.js` uses the Intersection Observer API to trigger animations when elements come into view. The CSS classes `hidden` and `show` control the visibility and animation of elements.

```javascript
// Intersection Observer for animation on scroll
let hiddenElements = document.querySelectorAll('.hidden');
const observer = new IntersectionObserver(entries => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('show');
            // Additional logic for specific elements
        } else {
            entry.target.classList.remove('show');
        }
    });
});

// Observe hidden elements with a delay
hiddenElements.forEach((element, index) => {
    setTimeout(() => {
        observer.observe(element);
    }, index === 1 ? 300 : 20);
});
```

### Typing Animation

The typing animation uses async/await and simulates typing and erasing text.

```javascript
// Improved typing animation using async/await
const typeText = async () => {
    // Text options to type
    const textOptions = ['Freelancer', 'BackEndDev', 'Programmer'];
    const textElement = document.getElementById('text');
    const cursorElement = document.getElementById('cursor');

    // Loop through text options
    for (let textIndex = 0; ; textIndex = (textIndex + 1) % textOptions.length) {
        cursorElement.classList.remove('blink');
        let charIndex = 0;

        // Type text
        while (charIndex < textOptions[textIndex].length) {
            textElement.textContent += textOptions[textIndex][charIndex];
            charIndex++;
            await sleep(100); // Typing speed
        }

        // Add blinking cursor
        cursorElement.classList.add('blink');
        await sleep(1000); // Delay before erasing

        // Erase text
        while (charIndex > 0) {
            cursorElement.classList.remove('blink');
            textElement.textContent = textOptions[textIndex].substring(0, charIndex - 1);
            charIndex--;
            await sleep(50); // Erasing speed
        }

        // Add blinking cursor
        cursorElement.classList.add('blink');
        await sleep(1000); // Delay before typing the next option
    }
};

// Start typing animation
typeText();
```

### Fetching Data from Backend

The code fetches project data from the backend and dynamically populates the projects section on the webpage.

```javascript
let data = fetch('https://kris007iron.shuttleapp.rs/api/v1/projects');

data.then((data) => {
    data.json().then((data) => {
        let projects = document.querySelector('#projects-inner');
        let projectsData = data;
        projectsData.forEach((project) => {
            // Additional logic for populating project data
            projects.innerHTML += `<a class="project" href="${project.link}">${project.title}</a>`;
        });
    });
});
```

## Additional Configurations

- **MongoDB Connection String**: Open `main.rs` and replace the placeholder in the `db_connection` function with your MongoDB connection string.

- **CORS Configuration**: The CORS headers are configured in `cors.rs`. Update the "Access-Control-Allow-Origin" header with the origin of your frontend.

- **Data Fetching in app.js**: Update the URL in the `fetch` function to match the endpoint of your backend.

-  **Handling email form endpoint**: Remember to update this field to get emails from people who will contact you.

## Conclusion

Feel free to explore and modify the code to suit your needs. If you have any feedback or suggestions, please comment and share them with your fellow programmers.

Happy coding! 🚀
