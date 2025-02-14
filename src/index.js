import React from 'react';
import ReactDOM from 'react-dom/client'; // Note the new import for React 18
import App from './App';
import './index.css';
import PouchDB from 'pouchdb';

import { DndProvider } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);
root.render(
  <DndProvider backend={HTML5Backend}>
    <App />
  </DndProvider>
);

window.clearPouchDBData = () => {
  const db = new PouchDB('task_manager_data');
  db.destroy()
    .then(() => {
      console.log("Database destroyed. All data has been deleted.");
      window.location.reload(); // Reload to initialize a fresh database
    })
    .catch((err) => {
      console.error("Error deleting database:", err);
    });
};