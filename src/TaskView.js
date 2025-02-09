import React, { useState } from 'react';

const modalOverlayStyle = {
  position: 'fixed',
  top: 0,
  left: 0,
  width: '100%',
  height: '100%',
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
  marginTop: '1rem',
};

const panelStyle = {
  flex: 1,
  margin: '0 0.5rem',
  background: '#2e2e2e',
  borderRadius: '4px',
  padding: '0.5rem',
  overflowY: 'auto',
  maxHeight: '300px',
};

const TaskView = ({ task, onClose, onUpdateTask }) => {
  const [subtaskInput, setSubtaskInput] = useState('');
  const [commentInput, setCommentInput] = useState('');

  // Add a subtask to the task.
  const addSubtask = () => {
    if (!subtaskInput.trim()) return;
    const newSubtask = { id: Date.now(), title: subtaskInput };
    const updatedTask = {
      ...task,
      subtasks: [...(task.subtasks || []), newSubtask],
    };
    onUpdateTask(updatedTask);
    setSubtaskInput('');
  };

  // Add a comment to the task.
  const addComment = () => {
    if (!commentInput.trim()) return;
    const newComment = { id: Date.now(), text: commentInput };
    const updatedTask = {
      ...task,
      comments: [...(task.comments || []), newComment],
    };
    onUpdateTask(updatedTask);
    setCommentInput('');
  };

  return (
    <div style={modalOverlayStyle} onClick={onClose}>
      <div style={modalContentStyle} onClick={(e) => e.stopPropagation()}>
        <h2>{task.title}</h2>
        <button
          style={{ alignSelf: 'flex-end', background: '#444', color: '#fff', border: 'none', padding: '0.5rem 1rem', borderRadius: '4px' }}
          onClick={onClose}
        >
          Close
        </button>
        <div style={panelContainerStyle}>
          {/* Subtasks Panel */}
          <div style={panelStyle}>
            <h3>Subtasks</h3>
            {(task.subtasks || []).map((st) => (
              <div key={st.id} style={{ padding: '0.25rem 0', borderBottom: '1px solid #333' }}>
                {st.title}
              </div>
            ))}
            <div style={{ marginTop: '0.5rem' }}>
              <input
                type="text"
                placeholder="Add subtask..."
                value={subtaskInput}
                onChange={(e) => setSubtaskInput(e.target.value)}
                style={{ width: '80%', padding: '0.25rem', borderRadius: '4px', border: 'none', outline: 'none', background: '#444', color: '#fff' }}
              />
              <button onClick={addSubtask} style={{ marginLeft: '0.5rem', background: '#333', color: '#fff', border: 'none', padding: '0.25rem 0.5rem', borderRadius: '4px' }}>Add</button>
            </div>
          </div>
          {/* Comments Panel */}
          <div style={panelStyle}>
            <h3>Comments</h3>
            {(task.comments || []).map((c) => (
              <div key={c.id} style={{ padding: '0.25rem 0', borderBottom: '1px solid #333' }}>
                {c.text}
              </div>
            ))}
            <div style={{ marginTop: '0.5rem' }}>
              <input
                type="text"
                placeholder="Add comment..."
                value={commentInput}
                onChange={(e) => setCommentInput(e.target.value)}
                style={{ width: '80%', padding: '0.25rem', borderRadius: '4px', border: 'none', outline: 'none', background: '#444', color: '#fff' }}
              />
              <button onClick={addComment} style={{ marginLeft: '0.5rem', background: '#333', color: '#fff', border: 'none', padding: '0.25rem 0.5rem', borderRadius: '4px' }}>Send</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TaskView;
