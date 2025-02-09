import React, { useState } from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import withDragAndDrop from 'react-big-calendar/lib/addons/dragAndDrop';

import 'react-big-calendar/lib/css/react-big-calendar.css';
import 'react-big-calendar/lib/addons/dragAndDrop/styles.css';

const localizer = momentLocalizer(moment);
const DnDCalendar = withDragAndDrop(Calendar);

const TaskCalendar = () => {
  const [events, setEvents] = useState([
    // Example event: a starting task for today
    {
      id: 0,
      title: 'Initial Task',
      start: new Date(),
      end: new Date(new Date().getTime() + 60 * 60 * 1000), // 1 hour later
    },
  ]);

  // Handle dragging events to new times/dates.
  const onEventDrop = ({ event, start, end, isAllDay: droppedOnAllDaySlot }) => {
    const updatedEvent = { ...event, start, end, allDay: droppedOnAllDaySlot };
    setEvents((prevEvents) =>
      prevEvents.map((existingEvent) =>
        existingEvent.id === event.id ? updatedEvent : existingEvent
      )
    );
  };

  // Handle resizing events
  const onEventResize = ({ event, start, end }) => {
    const updatedEvent = { ...event, start, end };
    setEvents((prevEvents) =>
      prevEvents.map((existingEvent) =>
        existingEvent.id === event.id ? updatedEvent : existingEvent
      )
    );
  };

  // When a user clicks on a slot, prompt for a new task
  const handleSelectSlot = (slotInfo) => {
    const title = prompt('Enter task title:');
    if (title) {
      const newEvent = {
        id: events.length, // simple id generation – consider using UUIDs for production
        title,
        start: slotInfo.start,
        end: slotInfo.end,
      };
      setEvents([...events, newEvent]);
    }
  };

  return (
    <div style={{ height: '100vh', padding: '1rem' }}>
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
