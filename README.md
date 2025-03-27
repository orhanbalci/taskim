https://github.com/user-attachments/assets/c4d54664-8b0c-4fa0-af11-f96d618edd04

I hopped around different task managers so I decided to make my own so that I can customize it.

## Features
- Import Tasks
  - The code is written to seamlessly import from ClickUp as that's what I was migrating from but it also provides a default schema for imports.
- Month, Week, and Quarter Views
- Create, complete, prioritize, delete, comment on, and add subtasks to tasks.
- Drag and Drop interface for both Month and Week views
- Search tasks and goals
- Persistant database even in localhost using PouchDB
- Github style Year View
<img width="1020" alt="image" src="https://github.com/user-attachments/assets/6932e4ec-99b1-48cd-ac32-da7009d61d9a" />


### Testing
```
npm run build
npx serve -s build
```

## Building the App
```
npm run build
npm run make
```
