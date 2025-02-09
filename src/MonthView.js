import React from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import withDragAndDrop from 'react-big-calendar/lib/addons/dragAndDrop';

import 'react-big-calendar/lib/css/react-big-calendar.css';
import 'react-big-calendar/lib/addons/dragAndDrop/styles.css';

const localizer = momentLocalizer(moment);
const DnDCalendar = withDragAndDrop(Calendar);

const MonthView = ({ events, weeklyGoals, currentDate, onNavigate, setEvents }) => {
  // Compute the visible date range for the month view.
  const startOfMonth = moment(currentDate).startOf('month').startOf('week');
  const endOfMonth = moment(currentDate).endOf('month').endOf('week');

  const weeks = [];
  let weekStart = startOfMonth.clone();
  while (weekStart.isBefore(endOfMonth)) {
    weeks.push(weekStart.clone());
    weekStart.add(1, 'week');
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

  // A simple custom toolbar for navigation
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
      <DnDCalendar
        localizer={localizer}
        events={events}
        defaultView="month"
        view="month"
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
      {/* Render a row for each week with its goal */}
      <div style={{ marginTop: '1rem' }}>
        {weeks.map((weekStart) => {
          // Use ISO week format for the key (e.g., "2025-07")
          const weekKey = weekStart.format('GGGG-ww');
          const goal = weeklyGoals[weekKey] || 'No goal set';
          return (
            <div
              key={weekKey}
              style={{
                padding: '0.5rem',
                background: '#2e2e2e',
                marginBottom: '0.5rem',
                borderRadius: '4px',
              }}
            >
              <strong>
                Week {weekStart.format('ww')} ({weekStart.format('MMM D')} –{' '}
                {moment(weekStart).add(6, 'days').format('MMM D')}):
              </strong>{' '}
              <span>{goal}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default MonthView;
