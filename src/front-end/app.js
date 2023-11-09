ones = [];
let hiddenElements = document.querySelectorAll('.hidden');
const observer = new IntersectionObserver(entries =>
{
    entries.forEach((entry) =>
    {
        // console.log(entry);
        if (entry.isIntersecting)
        {
            entry.target.classList.add('show');

            if (entry.target.classList.contains('one') && !ones.includes(entry.target))
            {
                ones.push(entry.target);
                if (ones.length == 3)
                {
                    setTimeout(() =>
                    {
                        entry.target.parentElement.classList.add('hidden');
                        entry.target.parentElement.classList.add('show');
                        observer.observe(entry.target.parentElement);
                        hiddenElements = document.querySelectorAll('.hidden');
                        for (let i = 0; i < ones.length; i++)
                        {
                            ones[i].classList.remove('hidden');
                            //hiddenElements.delete(ones[i]);
                            observer.unobserve(ones[i]);
                        }
                    }, 1000);

                }
            }
        } else
        {
            entry.target.classList.remove('show');
        }
    });
});
let i = 0;
hiddenElements.forEach((element) =>
{
    setTimeout(() =>
    {
        observer.observe(element);
    }, i == 1 ? 300 : 20);
    i == 0 ? i++ : i--;
});
// get request to the server
const get = (url) =>
{
    return fetch(url)
        .then(response => response.json())
        .catch(error => console.log(error));
};

const textOptions = ['Freelancer', 'BackEndDev', 'Programmer'];
const textElement = document.getElementById('text');
const cursorElement = document.getElementById('cursor');
let textIndex = 0;
let charIndex = 0;

function typeText()
{
    if (charIndex < textOptions[textIndex].length)
    {
        //remove blink animation
        cursorElement.classList.remove('blink');
        textElement.textContent += textOptions[textIndex][charIndex];
        charIndex++;
        setTimeout(typeText, 100); // Typing speed
    } else
    {
        //add blink animation
        cursorElement.classList.add('blink');
        setTimeout(eraseText, 1000); // Delay before erasing
    }
}

function eraseText()
{
    if (charIndex > 0)
    {
        cursorElement.classList.remove('blink');
        textElement.textContent = textOptions[textIndex].substring(0, charIndex - 1);
        charIndex--;
        setTimeout(eraseText, 50); // Erasing speed
    } else
    {
        cursorElement.classList.add('blink');
        textIndex = (textIndex + 1) % textOptions.length; // Loop through the options
        setTimeout(typeText, 1000); // Delay before typing the next option
    }
}

setTimeout(typeText, 1000); // Start typing after 1 second

let data = get('http://localhost:8000/api/v1/projects');
if (data == undefined)
{
    data = get('https://kris007iron.shuttle.rs/api/v1/projects');
}

data.then((data) =>
{
    console.log(data[0]);
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
renderBlogs = (blogs) =>
{
    if (blogs.items)
    {
        const columns = [];
        for (let i = 0; i < blogs.items.length; i += 3)
        {
            const columnContent = blogs.items.slice(i, i + 3).map(post =>
            {
                return `<div class="card">
                    <img src=${post.thumbnail} class="Img" />
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

fetch('https://api.rss2json.com/v1/api.json?rss_url=https://medium.com/feed/@kris007.iron')
    .then(resp => resp.json())
    .then(blogs =>
    {
        blogs_medium = blogs;
        // blogs_medium.items.push(blogs_medium.items[0]);
        let blogsDiv = document.querySelector('#posts');
        blogsDiv.innerHTML = renderBlogs(blogs_medium);
    });