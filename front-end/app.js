const observer = new IntersectionObserver(entries => {
    entries.forEach((entry) => {
        // console.log(entry);
        if (entry.isIntersecting) {
            entry.target.classList.add('show');
        } else {
            entry.target.classList.remove('show');
        }
    });
});
const hiddenElements = document.querySelectorAll('.hidden');
hiddenElements.forEach((element) => {
    observer.observe(element);
});
// get request to the server
const get = (url) => {
    return fetch(url)
        .then(response => response.json())
        .catch(error => console.log(error));
};

const text = document.querySelector('.sec-text');

let lastUpdateTime = new Date();


const textOptions = ['Freelancer', 'BackEndDev', 'Programmer'];
let currentIndex = 1;

const updateText = () => {
    const currentTime = new Date();
    const elapsedTime = currentTime - lastUpdateTime;

    if (elapsedTime >= 4000) {
        text.innerText = textOptions[currentIndex];
        currentIndex = (currentIndex + 1) % textOptions.length;
        lastUpdateTime = currentTime;
    }
};

setInterval(updateText, 10); // Check every second for updates

let data = get('http://localhost:8000/api/v1/projects');

data.then((data) => {
    console.log(data[0]);
    let projects = document.querySelector('#projects');
    let projectsData = data;
    projectsData.forEach((project) => {
        let tagsHTML = '';
        for (let i = 0; i < project.tags.length; i++) {
            tagsHTML += `<span class="tag">${project.tags[i]}</span>`;
        }
        projects.innerHTML += `        
        <a class="project" href="${project.link}">
        <div class="project-img">
            <img src="${project.images[0]}" alt="">
        </div>
        <div class="project-text">
            <h3>${project.title}</h3>
            <p>${project.description}</p>
            <div class="extras">
            ${tagsHTML}
            </div>                
            </div>            
        </a>
        `;
    });
});
let blogs_medium;
renderBlogs = (blogs) => {
    if (blogs.items) {
        return blogs.items.map(post => {
            return `<div class="column">
                <div class="card">
                    <img src=${post.thumbnail} class="Img" />
                    <h1 class="cardHeader">${post.title}</h1>
                    <p class="cardText">Posted on: ${post.pubDate}</p>
                    <a href=${post.link} class="Link"> Read the Full Blog Here!</a>
                </div>
            </div>`;
        })
    }
}
fetch('https://api.rss2json.com/v1/api.json?rss_url=https://medium.com/feed/@kris007.iron')
    .then(resp => resp.json())
    .then(blogs => {
        blogs_medium = blogs;
        let blogsDiv = document.querySelector('#posts');
        blogsDiv.innerHTML = renderBlogs(blogs_medium);
    });