// src/CustomMonthView.js
import React from 'react';
import moment from 'moment';

const CustomMonthView = ({ events, weeklyGoals, currentDate, setEvents }) => {
  // Calculate start and end dates for the grid:
  const startOfMonth = moment(currentDate).startOf('month');
  const endOfMonth = moment(currentDate).endOf('month');
  const startDate = moment(startOfMonth).startOf('week'); // beginning of the week that contains the 1st
  const endDate = moment(endOfMonth).endOf('week'); // end of the week that contains the last day

  // Build an array of weeks; each week is an array of 7 days (moment objects)
  let day = startDate.clone();
  const weeks = [];
  while (day.isBefore(endDate)) {
    const week = [];
    for (let i = 0; i < 7; i++) {
      week.push(day.clone());
      day.add(1, 'day');
    }
    weeks.push(week);
  }

  // Handler to add a task for a given day
  const handleAddTask = (day) => {
    const title = prompt(`Enter task title for ${day.format('MMM D, YYYY')}:`);
    if (title) {
      const newEvent = {
        id: Date.now(), // or any unique ID
        title,
        start: day.toDate(),
        end: moment(day).add(1, 'hour').toDate(),
      };
      setEvents((prevEvents) => [...prevEvents, newEvent]);
    }
  };

  return (
    <div style={{ background: '#121212', padding: '1rem', color: '#fff' }}>
      <h2 style={{ textAlign: 'center', marginBottom: '1rem' }}>
        {moment(currentDate).format('MMMM YYYY')}
      </h2>
      <table
        style={{
          width: '100%',
          borderCollapse: 'collapse',
          tableLayout: 'fixed',
          fontFamily:
            "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif",
        }}
      >
        <thead>
          <tr>
            {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map((dayName) => (
              <th
                key={dayName}
                style={{
                  border: '1px solid #333',
                  padding: '0.5rem',
                  background: '#1e1e1e',
                  color: '#fff',
                }}
              >
                {dayName}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {weeks.map((week, wi) => {
            // Use the first day of the week to look up the weekly goal.
            const weekKey = week[0].format('GGGG-ww'); // ISO week key (e.g., "2025-07")
            const weekGoal = weeklyGoals[weekKey] || 'No goal set';
            return (
              <React.Fragment key={`${weekKey}-${wi}`}>
                {/* Week goal row */}
                <tr>
                  <td
                    colSpan={7}
                    style={{
                      background: '#2e2e2e',
                      padding: '0.5rem',
                      textAlign: 'left',
                      fontStyle: 'italic',
                      border: '1px solid #333',
                    }}
                  >
                    Week {week[0].isoWeek()} Goal: {weekGoal}
                  </td>
                </tr>
                {/* Dates row */}
                <tr>
                  {week.map((day, di) => {
                    // Filter events for this day:
                    const dayEvents = events.filter((event) =>
                      moment(event.start).isSame(day, 'day')
                    );
                    return (
                      <td
                        key={di}
                        style={{
                          border: '1px solid #333',
                          verticalAlign: 'top',
                          height: '100px',
                          padding: '0.5rem',
                          background: '#1e1e1e',
                          cursor: 'pointer',
                        }}
                        onClick={() => handleAddTask(day)}
                      >
                        <div style={{ fontWeight: 'bold', marginBottom: '0.25rem' }}>
                          {day.format('D')}
                        </div>
                        <div style={{ fontSize: '0.8rem' }}>
                          {dayEvents.map((ev) => (
                            <div
                              key={ev.id}
                              style={{
                                background: '#444',
                                marginBottom: '0.25rem',
                                padding: '0.25rem',
                                borderRadius: '4px',
                                overflow: 'hidden',
                                whiteSpace: 'nowrap',
                                textOverflow: 'ellipsis',
                              }}
                            >
                              {ev.title}
                            </div>
                          ))}
                        </div>
                      </td>
                    );
                  })}
                </tr>
              </React.Fragment>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

export default CustomMonthView;
