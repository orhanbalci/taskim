// src/WeekView.js
import React from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import withDragAndDrop from 'react-big-calendar/lib/addons/dragAndDrop';

import 'react-big-calendar/lib/css/react-big-calendar.css';
import 'react-big-calendar/lib/addons/dragAndDrop/styles.css';

const localizer = momentLocalizer(moment);
const DnDCalendar = withDragAndDrop(Calendar);

const WeekView = ({ events, dailyGoals, currentDate, onNavigate, setEvents }) => {
  const startOfWeek = moment(currentDate).startOf('week');
  const daysOfWeek = [];
  for (let i = 0; i < 7; i++) {
    daysOfWeek.push(startOfWeek.clone().add(i, 'days'));
  }

  const onEventDrop = ({ event, start, end, isAllDay }) => {
    const updatedEvent = { ...event, start, end, allDay: isAllDay };
    setEvents((prev) => prev.map((ev) => (ev.id === event.id ? updatedEvent : ev)));
  };

  const onEventResize = ({ event, start, end }) => {
    const updatedEvent = { ...event, start, end };
    setEvents((prev) => prev.map((ev) => (ev.id === event.id ? updatedEvent : ev)));
  };

  const handleSelectSlot = (slotInfo) => {
    const title = prompt('Enter task title:');
    if (title) {
      const newEvent = {
        id: events.length,
        title,
        start: slotInfo.start,
        end: slotInfo.end,
        completed: false,
      };
      setEvents([...events, newEvent]);
    }
  };

  // Custom toolbar (similar to MonthView)
  const customToolbar = (toolbar) => (
    <div style={{ color: '#fff', marginBottom: '1rem' }}>
      <button onClick={() => toolbar.onNavigate('PREV')} style={{ marginRight: '1rem' }}>
        Prev
      </button>
      <button onClick={() => toolbar.onNavigate('TODAY')} style={{ marginRight: '1rem' }}>
        Today
      </button>
      <button onClick={() => toolbar.onNavigate('NEXT')} style={{ marginRight: '1rem' }}>
        Next
      </button>
      <span>{toolbar.label}</span>
    </div>
  );

  return (
    <div>
      {/* Header row with each day’s goal */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          background: '#2e2e2e',
          padding: '0.5rem',
          borderRadius: '4px',
          marginBottom: '1rem',
        }}
      >
        {daysOfWeek.map((day) => {
          const dayKey = day.format('YYYY-MM-DD');
          const goal = dailyGoals[dayKey] || 'No goal set';
          return (
            <div key={dayKey} style={{ flex: 1, textAlign: 'center' }}>
              <div>{day.format('ddd, MMM D')}</div>
              <div style={{ fontSize: '0.8rem', color: '#aaa' }}>{goal}</div>
            </div>
          );
        })}
      </div>
      <DnDCalendar
        localizer={localizer}
        events={events}
        defaultView="week"
        view="week"
        date={currentDate}
        onNavigate={onNavigate}
        onEventDrop={onEventDrop}
        onEventResize={onEventResize}
        resizable
        selectable
        onSelectSlot={handleSelectSlot}
        components={{
          toolbar: customToolbar,
        }}
        style={{ height: '70vh', backgroundColor: '#1e1e1e', color: '#fff' }}
      />
    </div>
  );
};

export default WeekView;
