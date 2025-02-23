let search_button = document.getElementById('search-task-button');
let search_task_result_div = document.getElementById('search-task-result');
search_button.addEventListener('click', () => {
    let id_input = document.getElementById('task-id');
    let id = id_input.value;
    if (id === '') {
        alert('Please fill the id field');
        return;
    }

    fetch(`/api/todos/${id}`)
        .then(response => response.json())
        .then(data => {
            let completed = data.completed ? 'Sim' : 'Não';
            id_input.value = '';
            search_task_result_div.innerHTML = `
                <div class="task-item">
                    <div class="task-id"> ${data.id} </div>
                    <div class="task-title"> ${data.title} </div>
                    <div class="task-content"> ${data.content} </div>
                    <div class="task-completed"> ${completed} </div>
                </div>
                `;
            
        })
        .catch(error => {
            search_task_result_div.innerHTML = ""
            setTimeout(() => alert('Task not found'), 100);
            
        });
})