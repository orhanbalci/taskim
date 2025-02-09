// src/QuarterView.js
import React from 'react';
import moment from 'moment';

const QuarterView = ({ events, weeklyGoals, currentDate }) => {
  const quarterStart = moment(currentDate).startOf('quarter');
  const quarterEnd = moment(currentDate).endOf('quarter');

  let weeks = [];
  let weekStart = quarterStart.clone().startOf('week');
  while (weekStart.isBefore(quarterEnd)) {
    weeks.push(weekStart.clone());
    weekStart.add(1, 'week');
  }

  return (
    <div
      style={{
        padding: '1rem',
        background: '#121212',
        color: '#fff',
        fontFamily: 'sans-serif',
      }}
    >
      <h2 style={{ textAlign: 'center' }}>
        Quarter View – {moment(currentDate).format('Q')} Quarter {moment(currentDate).year()}
      </h2>
      <div>
        {weeks.map((weekStart) => {
          const weekKey = weekStart.format('GGGG-ww');
          const goal = weeklyGoals[weekKey] || 'No goal set';
          const weekEnd = moment(weekStart).endOf('week');
          const taskCount = events.filter((event) =>
            moment(event.start).isBetween(weekStart, weekEnd, null, '[]')
          ).length;
          return (
            <div key={weekKey} style={{ padding: '0.5rem 0', borderBottom: '1px solid #333' }}>
              <strong>Week {weekStart.isoWeek()}:</strong> {goal} ({taskCount} tasks scheduled)
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default QuarterView;
