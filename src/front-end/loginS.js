let url = 'http://localhost:8000/api/v1';
// let url = 'https://kris007iron.shuttleapp.rs/api/v1';

let token = 'Bearer '
async function hash()
{
    const password = document.getElementById('password').value
    const username = document.getElementById('username').value
    if (!username && !password)
    {
        alert('Please enter a username and password.');
        return;
    }
    const encoder = new TextEncoder();
    const data = encoder.encode(password);
    const hash = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hash));
    const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    const postData = {
        "username": username,
        "hashed_password": hashHex
    }
    let result = await loginUser(url, postData)
    if (result === true)
    {
        verified();
    }
}

function verified()
{
    document.getElementById('projects-form').style.display = 'block';
    document.getElementById('posts-form').style.display = 'block';
    document.getElementById('login-panel').style.display = 'none';

}

function unVerified()
{
    document.getElementById('projects-form').style.display = 'none';
    document.getElementById('posts-form').style.display = 'none';
    document.getElementById('login-panel').style.display = 'block';

}

async function addProject()
{
    const formData = new FormData();
    const projectTitle = document.getElementById('projects-form-title').value;
    const projectDescription = document.getElementById('projects-form-description').value;
    const projectLink = document.getElementById('projects-form-link').value;
    const projectImage = document.getElementById('projects-form-images').files[0];
    const projectTags = document.getElementById('projects-form-tags').value.split(',');

    formData.append('title', projectTitle);
    formData.append('description', projectDescription);
    formData.append('link', projectLink);
    formData.append('image', projectImage);
    formData.append('tags', projectTags);

    const response = await fetch(url + '/projects', {
        method: "POST",
        headers: {
            "Authorization": token,
        },
        body: formData,
    });

    if (!response.ok)
    {
        if (response.status === 401)
        {
            alert("Unauthorized: Please log in.");
        } else if (response.status === 400)
        {
            alert("Bad Request: Please check your input.");
        } else if (response.status === 500)
        {
            alert("Server Error: Please try again later.");
        } else
        {
            alert(`Unexpected Error: ${response.statusText}`);
        }
    } else
    {
        alert("Project added successfully.");
    }
}

async function addPost()
{
    const formData = new FormData();
    const postTitle = document.getElementById('posts-form-title').value;
    const postPubDate = document.getElementById('posts-form-pubDate').value;
    const postLink = document.getElementById('posts-form-link').value;
    const postThumbnail = document.getElementById('posts-form-thumbnail').files[0];

    formData.append('title', postTitle);
    formData.append('pub_date', postPubDate);
    formData.append('link', postLink);
    formData.append('thumbnail', postThumbnail);

    const response = await fetch(url + '/posts', {
        method: "POST",
        headers: {
            "Authorization": token,
        },
        body: formData,
    });

    if (!response.ok)
    {
        if (response.status === 401)
        {
            alert("Unauthorized: Please log in.");
        } else if (response.status === 400)
        {
            alert("Bad Request: Please check your input.");
        } else if (response.status === 500)
        {
            alert("Server Error: Please try again later.");
        } else
        {
            alert(`Unexpected Error: ${response.statusText}`);
        }
    }
    else
    {
        alert("Post added successfully.");
    }
}

async function loginUser(url, postData)
{
    try
    {
        const response = await fetch(url + '/login', {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(postData),
        });

        // Check if the response status is not OK (status 200-299)
        if (!response.ok)
        {
            // Handle specific HTTP error status codes
            if (response.status === 401)
            {
                throw new Error("Unauthorized: Incorrect username or password.");
            } else if (response.status === 400)
            {
                throw new Error("Bad Request: Please check your input.");
            } else if (response.status === 500)
            {
                throw new Error("Server Error: Please try again later.");
            } else
            {
                throw new Error(`Unexpected Error: ${response.statusText}`);
            }
        }

        // Attempt to parse the JSON response
        let result;
        try
        {
            result = await response.json();
        } catch (jsonError)
        {
            throw new Error("Failed to parse JSON response: " + jsonError.message);
        }

        // Successfully obtained and parsed the responsez        
        token = 'Bearer ' + result;
        console.log(token);
        return true;
    } catch (error)
    {
        // General error handler
        console.error("Login failed:", error.message);
        // Optionally, you can return a message or object to indicate failure
        return { success: false, message: error.message };
    }
}

async function generateProjectsList()
{
    let projectList = document.getElementById('projects-list');
    projectList.innerHTML = '';
    let projects = await getProjects();
    for (let project of projects)
    {
        //_id: {$oid: "6543ea5d875bc6bcda7d9218"}
        let projectItem = document.createElement('li');
        projectItem.innerHTML = `<h3>${project.title}</h3>
        <p>${project.description}</p>
        <a href="${project.link}">Link</a>
        <img src="${project.image}" alt="${project.title}">
        <p>${project.tags}</p>
        <button onclick="deleteProject(${project._id})">Delete</button>
        <button onclick="updateProject(${project._id})">Patch</button>`;
        projectList.appendChild(projectItem);
    }
}

async function getProjects()
{
    try
    {
        const response = await fetch(url + '/projects', {
            method: "GET",
        });

        if (!response.ok)
        {
            if (response.status === 401)
            {
                alert("Unauthorized: Please log in.");
            } else if (response.status === 500)
            {
                alert("Server Error: Please try again later.");
            } else
            {
                alert(`Unexpected Error: ${response.statusText}`);
            }
        }

        let result = await response.json();
        let projects = result.projects;
        return projects;
    } catch (error)
    {
        console.error("Failed to get projects:", error.message);
    }
}

