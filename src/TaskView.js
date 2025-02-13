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

const TaskView = ({ task, onClose, onUpdateTask }) => {
  // Use local state so the modal updates immediately.
  const [subtasks, setSubtasks] = useState(task.subtasks || []);
  const [comments, setComments] = useState(task.comments || []);
  const [subtaskInput, setSubtaskInput] = useState('');
  const [commentInput, setCommentInput] = useState('');
  const modalRef = useRef(null);

  // Sync local state when task prop changes.
  useEffect(() => {
    setSubtasks(task.subtasks || []);
    setComments(task.comments || []);
  }, [task]);

  // Close on Escape key.
  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  // Close on clicking outside.
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
    const commentText = commentInput.trim();
    const newComment = { id: Date.now(), text: commentText };
    const newComments = [...comments, newComment];

    let updatedTask = { ...task, subtasks, comments: newComments };
    // Check if the comment text is "done" (ignoring case).
    if (commentText.toLowerCase() === 'done') {
      updatedTask.completed = true;
    }
    
    setComments(updatedTask.comments);
    onUpdateTask(updatedTask);
    setCommentInput('');
  };

  return (
    <div style={modalOverlayStyle} onClick={handleOverlayClick}>
      <div style={modalContentStyle} ref={modalRef}>
        <h2 style={{ margin: 0, color: '#fff' }}>{task.title}</h2>
        <div style={panelContainerStyle}>
          {/* Subtasks panel */}
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
                <span style={{ color: '#fff' }}>{st.title}</span>
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
          {/* Comments panel */}
          <div style={panelStyle}>
            {comments.map((c) => (
              <div key={c.id} style={{ padding: '0.25rem 0', borderBottom: '1px solid #333', color: '#fff' }}>
                {c.text}
              </div>
            ))}
            <input
              type="text"
              placeholder="Add comment (type 'done' to complete)"
              value={commentInput}
              onChange={(e) => setCommentInput(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter') addComment(); }}
              style={inputStyle}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default TaskView;
