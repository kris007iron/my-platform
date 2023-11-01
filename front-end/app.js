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
