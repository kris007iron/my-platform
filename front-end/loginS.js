// let url = 'http://localhost:8000/api/v1';
let url = 'https://kris007iron-o9ms.shuttle.app/api/v1';

let token = 'Bearer '
let deleteTargetId = null
let deleteType = null
let currentEditProjectId = null
let currentEditPostId = null

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
        await generateProjectsList();
        await generatePostsList();
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
        <img src="${project.images[0]}" alt="${project.title}">
        <p>${project.tags}</p>
        <button onclick="showDeleteModal('${project._id}', 'project')">Delete</button>
        <button onclick="updateProject('${project._id}')">Edit</button>
        `;
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
        console.log(result.map(res =>
        {
            console.log(res._id)
        }))
        return result;
    } catch (error)
    {
        console.error("Failed to get projects:", error.message);
    }
}

async function deleteProject(id)
{
    try
    {
        const response = await fetch(url + '/projects/' + id, {
            method: "DELETE",
            headers: {
                "Authorization": token,
            },
        });

        if (!response.ok)
        {
            if (response.status === 401)
            {
                alert("Unauthorized: Please log in.");
            } else if (response.status === 404)
            {
                alert("Not Found: Project not found.");
            } else if (response.status === 500)
            {
                alert("Server Error: Please try again later.");
            } else
            {
                alert(`Unexpected Error: ${response.statusText}`);
            }
        }

        alert("Project deleted successfully.");
    } catch (error)
    {
        console.error("Failed to delete project:", error.message);
    }
}

//TODO: test it
async function updateProject(id)
{
    currentEditProjectId = id;

    const project = [...document.querySelector('#projects-list li')].find(li => li.innerHTML.includes(id))
    if (!project) return;

    document.getElementById('edit-project-title').value = project.querySelector('h3')?.textContent || '';
    document.getElementById('edit-project-description').value = project.querySelector('p')?.textContent || '';
    document.getElementById('edit-project-link').value = project.querySelector('a')?.href || '';
    document.getElementById('edit-project-tags').value = project.querySelector('p')[1]?.textContent || '';

    document.getElementById('update-project-modal').classList.remove('hidden');
}

async function submitProjectUpdate()
{
    const updatedData = {
        title: document.getElementById('edit-project-title').valuem,
        description: document.getElementById('edit-project-description').value,
        link: document.getElementById('edit-project-link').value,
        tags: document.getElementById('edit-project-tags').value.split(','),
        image: document.getElementById('edit-project-image').files[0] ? document.getElementById('edit-project-image').files[0] : ""
    };

    try
    {
        const response = await fetch(`${url}/projects/${currentEditProjectId}`, {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json",
                "Authorization": token,
            },
            body: JSON.stringify(updatedData)
        });

        if (!response.ok)
        {
            alert("Failed to update project")
        } else
        {
            alert("Project updated successfully")
            await generateProjectsList()
        }
    }
    catch (err)
    {
        console.err("UPDATE failed: ", err.message)
    }
    closeModal()
}

async function generatePostsList()
{
    let postList = document.getElementById('posts-list');
    postList.innerHTML = '';
    let posts = await getPosts();
    for (let post of posts)
    {
        let postItem = document.createElement('li');
        postItem.innerHTML = `<h3>${post.title}</h3>
        <p>${post.pub_date}</p>
        <a href="${post.link}">Link</a>
        <img src="${post.thumbnail}" alt="${post.title}">
        <button onclick="showDeleteModal('${post._id}', 'post')">Delete</button>
        <button onclick="updatePost(${post._id})">Patch</button>`;
        postList.appendChild(postItem);
    }
}

async function getPosts()
{
    try
    {
        const response = await fetch(url + '/posts', {
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
        console.log(result);
        return result;
    } catch (error)
    {
        console.error("Failed to get posts:", error.message);
    }
}

async function deletePost(id)
{
    try
    {
        const response = await fetch(url + '/posts/' + id, {
            method: "DELETE",
            headers: {
                "Authorization": token,
            },
        });

        if (!response.ok)
        {
            if (response.status === 401)
            {
                alert("Unauthorized: Please log in.");
            } else if (response.status === 404)
            {
                alert("Not Found: Post not found.");
            } else if (response.status === 500)
            {
                alert("Server Error: Please try again later.");
            } else
            {
                alert(`Unexpected Error: ${response.statusText}`);
            }
        }

        alert("Post deleted successfully.");
    } catch (error)
    {
        console.error("Failed to delete post:", error.message);
    }
}

async function updatePost(id)
{
    //TODO: add photo
    currentEditPostId = id;

    const post = [...document.querySelector('#posts-list li')].find(li => li.innerHTML.includes(id))
    if (!project) return;

    document.getElementById('edit-post-title').value = project.querySelector('h3')?.textContent || '';
    document.getElementById('edit-post-description').value = project.querySelector('p')?.textContent || '';
    document.getElementById('edit-post-link').value = project.querySelector('a')?.href || '';
    document.getElementById('edit-post-tags').value = project.querySelector('p')[1]?.textContent || '';

    document.getElementById('update-project-modal').classList.remove('hidden');
}

async function submitPostUpdate()
{
    const updatedData = {
        title: document.getElementById('edit-post-title').valuem,
        pub_date: document.getElementById('edit-post-pubDate').value,
        link: document.getElementById('edit-post-link').value,
        thumbnail: document.getElementById('edit-post-thumbnail').value.split(',')
        //TODO: add photo
    };

    try
    {
        const response = await fetch(`${url}/projects/${currentEditPostId}`, {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json",
                "Authorization": token,
            },
            body: JSON.stringify(updatedData)
        });

        if (!response.ok)
        {
            alert("Failed to update project")
        } else
        {
            alert("Project updated successfully")
            await generateProjectsList()
        }
    }
    catch (err)
    {
        console.err("UPDATE failed: ", err.message)
    }
    closeModal()
}

function showDeleteModal(id, type)
{
    deleteTargetId = id;
    deleteType = type;
    document.getElementById('confirm-delete-modal').classList.remove('hidden');
}

function confirmDelete()
{
    if (deleteType === 'project')
    {
        deleteProject(deleteTargetId);
    } else if (deleteType === 'post')
    {
        deletePost(deleteTargetId);
    }
}

function closeModal()
{
    document.querySelector('.modal').forEach(modal =>
    {
        modal.classList.add('hidden')
    });
    deleteTargetId = null;
    deleteType = null;
    currentEditPostId = null;
    currentEditProjectId = null;
}