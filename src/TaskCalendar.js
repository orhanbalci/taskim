import React, { useState } from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import withDragAndDrop from 'react-big-calendar/lib/addons/dragAndDrop';

import 'react-big-calendar/lib/css/react-big-calendar.css';
import 'react-big-calendar/lib/addons/dragAndDrop/styles.css';

import CanvasPrompt from './CanvasPrompt'; // <-- Import the updated CanvasPrompt

const localizer = momentLocalizer(moment);
const DnDCalendar = withDragAndDrop(Calendar);

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
