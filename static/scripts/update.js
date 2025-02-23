let update_button = document.getElementById('update-task-button');
let id_input = document.getElementById('task-id');
let title_input = document.getElementById('task-title');
let content_input = document.getElementById('task-content');
let completed_input = document.getElementById('task-completed');

update_button.addEventListener('click', () => {
    let id = id_input.value;
    let title = title_input.value;
    let content = content_input.value;
    let completed = completed_input.checked;
    
    if (id === '' || Number.isNaN(Number(id))) {
        alert('Please fill the id field with a valid id');
        return;
    }

    let json_to_send = {}
    if (title !== '') {
        json_to_send.title = title;
    }
    if (content !== '') {
        json_to_send.content = content;
    }
    if (completed !== '') {
        json_to_send.completed = completed;
    }
    
    fetch(`/api/todos/${id}`, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(json_to_send),
    })
        .then(response => response.json().catch(() => null))
        .then(data => {
            if (data) {
                alert(`Task updated!`);
                id_input.value = '';
                title_input.value = '';
                content_input.value = '';
                completed_input.checked = false;
            } else {
                alert('Failed to update task: Invalid JSON response');
            }
        })
        .catch(error => console.error('Error updating task:', error));
});
