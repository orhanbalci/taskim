import React from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import withDragAndDrop from 'react-big-calendar/lib/addons/dragAndDrop';

import 'react-big-calendar/lib/css/react-big-calendar.css';
import 'react-big-calendar/lib/addons/dragAndDrop/styles.css';

const localizer = momentLocalizer(moment);
const DnDCalendar = withDragAndDrop(Calendar);

const WeekView = ({ events, currentDate, setEvents, onTaskShiftClick }) => {
  const onEventDrop = ({ event, start, end, isAllDay: droppedOnAllDaySlot }) => {
    const updatedEvent = { ...event, start, end, allDay: droppedOnAllDaySlot };
    setEvents((prev) => prev.map((ev) => (ev.id === event.id ? updatedEvent : ev)));
  };

  const onEventResize = ({ event, start, end }) => {
    const updatedEvent = { ...event, start, end };
    setEvents((prev) => prev.map((ev) => (ev.id === event.id ? updatedEvent : ev)));
  };

  const dayPropGetter = (date) => ({ style: { backgroundColor: '#1e1e1e', color: '#fff' } });

  return (
    <div>
      <DnDCalendar
        localizer={localizer}
        events={events}
        defaultView="week"
        view="week"
        date={new Date(currentDate)}
        onEventDrop={onEventDrop}
        onEventResize={onEventResize}
        resizable
        selectable
        onSelectEvent={(event, e) => {
          if (e.shiftKey) {
            onTaskShiftClick && onTaskShiftClick(event);
          }
        }}
        dayPropGetter={dayPropGetter}
        style={{ height: '70vh', backgroundColor: '#1e1e1e', color: '#fff' }}
        toolbar={false}
      />
    </div>
  );
};

export default WeekView;
