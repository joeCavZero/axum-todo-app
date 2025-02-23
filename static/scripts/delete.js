let delete_task_button = document.getElementById('delete-task-button');
let id_input = document.getElementById('task-id');

delete_task_button.addEventListener('click', () => {
    let id = id_input.value;
    if (id === '' || Number.isNaN(Number(id))) {
        alert('Please fill the id field with a valid id');
        return;
    }

    fetch(`/api/todos/${id}`, {
        method: 'DELETE',
    })
        .then(response => {
            if (response.status === 404) {
                alert('Task not found!');
                return;
            }
            return response.json();
        })
        .then(data => {
            if (data) {
                alert('Task deleted!');
                id_input.value = '';
            }
        })
        .catch(error => console.error('Error deleting task:', error));
});