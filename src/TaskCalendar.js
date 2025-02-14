import React, { useState, useRef, useEffect } from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import withDragAndDrop from 'react-big-calendar/lib/addons/dragAndDrop';

import 'react-big-calendar/lib/css/react-big-calendar.css';
import 'react-big-calendar/lib/addons/dragAndDrop/styles.css';

const localizer = momentLocalizer(moment);
const DnDCalendar = withDragAndDrop(Calendar);

const CanvasPrompt = ({ message, onSubmit, onCancel }) => {
  const [input, setInput] = useState('');
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas) {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
      const ctx = canvas.getContext('2d');
      ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    }
  }, []);

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      onSubmit(input);
    } else if (e.key === 'Escape') {
      onCancel();
    }
  };

  return (
    <div style={{ position: 'fixed', top: 0, left: 0, width: '100%', height: '100%', zIndex: 1000 }}>
      <canvas ref={canvasRef} style={{ position: 'absolute', top: 0, left: 0 }} />
      <input
        type="text"
        autoFocus
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder={message}
        style={{
          position: 'absolute',
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          background: 'rgba(255, 255, 255, 0.2)',
          color: '#fff',
          border: 'none',
          outline: 'none',
          padding: '1rem',
          fontSize: '1.2rem',
          width: '80%',
        }}
      />
    </div>
  );
};

const TaskCalendar = () => {
  const [events, setEvents] = useState([
    {
      id: 0,
      title: 'Initial Task',
      start: new Date(),
      end: new Date(new Date().getTime() + 60 * 60 * 1000),
    },
  ]);
  const [promptData, setPromptData] = useState(null);

  const onEventDrop = ({ event, start, end, isAllDay: droppedOnAllDaySlot }) => {
    const updatedEvent = { ...event, start, end, allDay: droppedOnAllDaySlot };
    setEvents((prevEvents) =>
      prevEvents.map((existingEvent) =>
        existingEvent.id === event.id ? updatedEvent : existingEvent
      )
    );
  };

  const onEventResize = ({ event, start, end }) => {
    const updatedEvent = { ...event, start, end };
    setEvents((prevEvents) =>
      prevEvents.map((existingEvent) =>
        existingEvent.id === event.id ? updatedEvent : existingEvent
      )
    );
  };

  const handleSelectSlot = (slotInfo) => {
    setPromptData({ slotInfo, message: 'Enter task title:' });
  };

  const submitPrompt = (title) => {
    if (title && promptData) {
      const { slotInfo } = promptData;
      const newEvent = {
        id: events.length,
        title,
        start: slotInfo.start,
        end: slotInfo.end,
      };
      setEvents([...events, newEvent]);
    }
    setPromptData(null);
  };

  return (
    <div style={{ height: '100vh', padding: '1rem', position: 'relative' }}>
      {promptData && (
        <CanvasPrompt
          message={promptData.message}
          onSubmit={submitPrompt}
          onCancel={() => setPromptData(null)}
        />
      )}
      <DnDCalendar
        localizer={localizer}
        events={events}
        defaultView="month"
        views={['month', 'week', 'day']}
        onEventDrop={onEventDrop}
        onEventResize={onEventResize}
        resizable
        selectable
        onSelectSlot={handleSelectSlot}
        style={{ height: '100%' }}
      />
    </div>
  );
};

export default TaskCalendar;
