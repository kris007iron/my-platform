let hiddenElements = document.querySelectorAll('.hidden');
const observer = new IntersectionObserver(entries =>
{
    entries.forEach(entry =>
    {
        if (entry.isIntersecting)
        {
            entry.target.classList.add('show');

            if (entry.target.classList.contains('one') && !entry.target.classList.contains('shown'))
            {
                entry.target.classList.add('shown');
                if (document.querySelectorAll('.one.shown').length === 3)
                {
                    setTimeout(() =>
                    {
                        const parent = entry.target.parentElement;
                        parent.classList.add('hidden', 'show');
                        observer.observe(parent);
                        hiddenElements = document.querySelectorAll('.hidden');
                        document.querySelectorAll('.one.shown').forEach(el =>
                        {
                            el.classList.remove('hidden');
                            observer.unobserve(el);
                        });
                    }, 1000);
                }
            }
        } else
        {
            entry.target.classList.remove('show');
        }
    });
});

hiddenElements.forEach((element, index) =>
{
    setTimeout(() =>
    {
        observer.observe(element);
    }, index === 1 ? 300 : 20);
});

// Improved typing animation using async/await
const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));

const typeText = async () =>
{
    const textOptions = ['Freelancer', 'BackEndDev', 'Programmer'];
    const textElement = document.getElementById('text');
    const cursorElement = document.getElementById('cursor');

    for (let textIndex = 0; ; textIndex = (textIndex + 1) % textOptions.length)
    {
        cursorElement.classList.remove('blink');
        let charIndex = 0;
        while (charIndex < textOptions[textIndex].length)
        {
            textElement.textContent += textOptions[textIndex][charIndex];
            charIndex++;
            await sleep(100); // Typing speed
        }

        cursorElement.classList.add('blink');
        await sleep(1000); // Delay before erasing

        while (charIndex > 0)
        {
            cursorElement.classList.remove('blink');
            textElement.textContent = textOptions[textIndex].substring(0, charIndex - 1);
            charIndex--;
            await sleep(50); // Erasing speed
        }

        cursorElement.classList.add('blink');
        await sleep(1000); // Delay before typing the next option
    }
};

typeText(); // Start typing

let url = 'http://localhost:8000/api/v1';
// let url = 'https://kris007iron.shuttleapp.rs/api/v1';
let data = fetch(url + "/projects");//'https://kris007iron.shuttleapp.rs/api/v1/'

data.then((data) =>
{
    data.json().then((data) =>
    {
        let projects = document.querySelector('#projects-inner');
        let projectsData = data;
        projectsData.forEach((project) =>
        {
            let tagsHTML = '';
            for (let i = 0; i < project.tags.length; i++)
            {
                tagsHTML += `<span class="tag">${project.tags[i]}</span>`;
            }
            projects.innerHTML += `        
            <a class="project" href="${project.link}">
            <div class="project-img">
                <img src="${project.images[0]}" alt="" loading="lazy">
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
});
let blogs_medium;
renderBlogs = (blogs) =>
{
    console.log('blogs');
    if (blogs.items)
    {
        const columns = [];
        for (let i = 0; i < blogs.items.length; i += 3)
        {
            const columnContent = blogs.items.slice(i, i + 3).map(post =>
            {
                return `<div class="card">
                    <img src=${post.thumbnail} class="Img" loading="lazy"/>
                    <h1 class="cardHeader">${post.title}</h1>
                    <p class="cardText">Posted on: ${post.pubDate}</p>
                    <a href=${post.link} class="Link"> Read the Full Blog Here!</a>
                </div>`;
            });

            columns.push(`<div class="column">${columnContent.join('')}</div>`);
        }

        return columns.join('');
    }
}


//document.querySelector('#posts').innerHTML = renderBlogs({ "items": blogs_medium });
fetch(url + '/posts')
    .then(resp => resp.json())
    .then(blogs =>
    {
        blogs_medium = blogs;
        // blogs_medium.items.push(blogs_medium.items[0]);
        let blogsDiv = document.querySelector('#posts');
        blogsDiv.innerHTML = renderBlogs({ "items": blogs_medium });
    });