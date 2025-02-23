function reloadTasks() {
    const tasks_frame = document.getElementById("tasks-frame"); // Pegando o elemento correto

    fetch("/api/todos")
        .then(response => response.json())
        .then(data => {
            tasks_frame.innerHTML = ''; // Limpa a div antes de adicionar novas tasks
            data.forEach(task => {
                let task_div = document.createElement('div');
                task_div.className = 'task-item';
                let completed = task.completed ? 'Sim' : 'Não';
                task_div.innerHTML = `
                    <div class="task-id">${task.id}</div>
                    <div class="task-title">${task.title}</div>
                    <div class="task-content">${task.content}</div>
                    <div class="task-completed">${completed}</div>
                `;
                tasks_frame.appendChild(task_div);
            });
        })
        .catch(error => console.error('Error fetching tasks:', error));
}


// Call the function to load tasks when the script is loaded
reloadTasks();
let reload_button = document.getElementById('reload-tasks-button');
reload_button.addEventListener('click', reloadTasks);

