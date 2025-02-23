
let send_button = document.getElementById('send-task-button');
let title_input = document.getElementById('task-title');
let content_input = document.getElementById('task-content');

send_button.addEventListener('click', () => {
    let title = title_input.value;
    let content = content_input.value;
    if (title === '' || content === '') {
        alert('Please fill all fields');
        return;
    }

    fetch('/api/todos', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            title: title,
            content: content,
        }),
    })
        .then(response => response.json())
        .then(data => {
            alert(`Task created: \n${data.title} \n${data.content}`);
            title_input.value = '';
            content_input.value = '';
        })
        .catch(error => console.error('Error creating task:', error));
})