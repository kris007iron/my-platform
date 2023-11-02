const observer = new IntersectionObserver(entries =>
{
    entries.forEach((entry) =>
    {
        console.log(entry);
        if (entry.isIntersecting)
        {
            entry.target.classList.add('show');
        } else
        {
            entry.target.classList.remove('show');
        }
    });
});
const hiddenElements = document.querySelectorAll('.hidden');
hiddenElements.forEach((element) =>
{
    observer.observe(element);
});
// get request to the server
const get = (url) =>
{
    return fetch(url)
        .then(response => response.json())
        .catch(error => console.log(error));
};

const text = document.querySelector('.sec-text');

let lastUpdateTime = new Date();


const textOptions = ['Freelancer', 'BackEndDev', 'Programmer'];
let currentIndex = 1;

const updateText = () =>
{
    const currentTime = new Date();
    const elapsedTime = currentTime - lastUpdateTime;

    if (elapsedTime >= 4000)
    {
        text.innerText = textOptions[currentIndex];
        currentIndex = (currentIndex + 1) % textOptions.length;
        lastUpdateTime = currentTime;
    }
};

setInterval(updateText, 100); // Check every second for updates
