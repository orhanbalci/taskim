import React, { useState, useEffect, useRef } from 'react';

const modalOverlayStyle = {
  position: 'fixed',
  top: 0,
  left: 0,
  right: 0,
  bottom: 0,
  background: 'rgba(0,0,0,0.7)',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  zIndex: 1000,
};

const modalContentStyle = {
  background: '#1e1e1e',
  padding: '1rem',
  borderRadius: '8px',
  width: '80%',
  maxWidth: '800px',
  maxHeight: '90vh',
  display: 'flex',
  flexDirection: 'column',
};

const panelContainerStyle = {
  display: 'flex',
  flex: 1,
  gap: '1rem',
  marginTop: '1rem',
};

const panelStyle = {
  flex: 1,
  background: '#2e2e2e',
  borderRadius: '4px',
  padding: '0.5rem',
  overflowY: 'auto',
};

const inputStyle = {
  width: '100%',
  padding: '0.5rem',
  background: '#444',
  border: 'none',
  outline: 'none',
  color: '#fff',
  borderRadius: '4px',
  marginTop: '0.5rem',
};

const TaskView = ({ task, onClose, onUpdateTask, onDeleteTask }) => {
  const [subtasks, setSubtasks] = useState(task.subtasks || []);
  const [comments, setComments] = useState(task.comments || []);
  const [subtaskInput, setSubtaskInput] = useState('');
  const [commentInput, setCommentInput] = useState('');
  const [animateComplete, setAnimateComplete] = useState(false);
  const [animateUrgent, setAnimateUrgent] = useState(false);
  const [animateUndo, setAnimateUndo] = useState(false);
  const modalRef = useRef(null);

  useEffect(() => {
    setSubtasks(task.subtasks || []);
    setComments(task.comments || []);
  }, [task]);

  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  const handleOverlayClick = (e) => {
    if (modalRef.current && !modalRef.current.contains(e.target)) {
      onClose();
    }
  };

  const addSubtask = () => {
    if (!subtaskInput.trim()) return;
    const newSubtask = { id: Date.now(), title: subtaskInput, completed: false };
    const newSubtasks = [...subtasks, newSubtask];
    setSubtasks(newSubtasks);
    onUpdateTask({ ...task, subtasks: newSubtasks, comments });
    setSubtaskInput('');
  };

  const toggleSubtask = (id) => {
    const newSubtasks = subtasks.map((st) =>
      st.id === id ? { ...st, completed: !st.completed } : st
    );
    setSubtasks(newSubtasks);
    onUpdateTask({ ...task, subtasks: newSubtasks, comments });
  };

  const addComment = () => {
    if (!commentInput.trim()) return;
    const command = commentInput.trim().toLowerCase();
  
    if (command === 'delete') {
      onDeleteTask(task.id);
      onClose();
      return;
    }
  
    const newComment = { id: Date.now(), text: commentInput.trim() };
    const newComments = [...comments, newComment];
    
    let updatedTask = { ...task, subtasks, comments: newComments };
  
    if (command === 'done') {
      updatedTask.completed = true;
      setAnimateComplete(true);
      setTimeout(() => setAnimateComplete(false), 1000);
    } else if (command === 'undo') {
      updatedTask.completed = false;
      setAnimateUndo(true);
      setTimeout(() => setAnimateUndo(false), 1000);
    } else if (command === 'urgent') {
      updatedTask.urgent = true;
      setAnimateUrgent(true);
      setTimeout(() => setAnimateUrgent(false), 1000);
    } else if (command === 'not urgent') {
      updatedTask.urgent = false;
      setAnimateUndo(true);
      setTimeout(() => setAnimateUndo(false), 1000);
    }
    
    setComments(updatedTask.comments);
    onUpdateTask(updatedTask);
    setCommentInput('');
  };  

  return (
    <>
      <style>
        {`
          @keyframes completeFlash {
            0% { background-color: #1e1e1e; }
            50% { background-color: #2e7d32; }
            100% { background-color: #1e1e1e; }
          }
          @keyframes urgentFlash {
            0% { background-color: #1e1e1e; }
            50% { background-color: #b71c1c; }
            100% { background-color: #1e1e1e; }
          }
          @keyframes updateFlash {
            0% { opacity: 1; }
            50% { opacity: 0.5; }
            100% { opacity: 1; }
          }
        `}
      </style>
      <div style={modalOverlayStyle} onClick={handleOverlayClick}>
        <div style={modalContentStyle} ref={modalRef}>
          <h2 
            style={{ 
              margin: 0, 
              color: '#fff',
              ...(animateComplete && { animation: 'completeFlash 1s ease-out' }),
              ...(animateUrgent && { animation: 'urgentFlash 1s ease-out' }),
              ...(animateUndo && { animation: 'updateFlash 1s ease-out' }),
              // Apply urgent border only if task is urgent and not completed.
              ...(task.urgent && !task.completed ? { border: '2px solid red', padding: '0.5rem' } : {})
            }}
          >
            {task.title}
          </h2>
          <div style={panelContainerStyle}>
            <div style={panelStyle}>
              {subtasks.map((st) => (
                <div
                  key={st.id}
                  onClick={() => toggleSubtask(st.id)}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    padding: '0.25rem 0',
                    borderBottom: '1px solid #333',
                    cursor: 'pointer',
                    color: '#fff'
                  }}
                >
                  <span
                    style={{
                      marginRight: '0.5rem',
                      fontSize: '1.2rem',
                      color: st.completed ? 'green' : '#fff',
                    }}
                  >
                    {st.completed ? '✔︎' : '○'}
                  </span>
                  <span>{st.title}</span>
                </div>
              ))}
              <input
                type="text"
                placeholder="Add subtask"
                value={subtaskInput}
                onChange={(e) => setSubtaskInput(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter') addSubtask(); }}
                style={inputStyle}
              />
            </div>
            <div style={panelStyle}>
              {comments.map((c) => (
                <div key={c.id} style={{ padding: '0.25rem 0', borderBottom: '1px solid #333', color: '#fff' }}>
                  {c.text}
                </div>
              ))}
              <input
                type="text"
                placeholder="Add comment (type 'done', 'undo', 'urgent', or 'not urgent')"
                value={commentInput}
                onChange={(e) => setCommentInput(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter') addComment(); }}
                style={inputStyle}
              />
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default TaskView;
