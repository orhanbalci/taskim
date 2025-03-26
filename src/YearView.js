import React, { useMemo } from 'react';
import moment from 'moment';

const YearView = ({ events, currentDate }) => {
  const year = moment(currentDate).year();
  
  // Generate all days for the year
  const yearData = useMemo(() => {
    const firstDay = moment([year, 0, 1]);
    const lastDay = moment([year, 11, 31]);
    
    // Start from the first Sunday before or on January 1st
    const startDate = moment(firstDay).startOf('week');
    // End on the last Saturday after or on December 31st
    const endDate = moment(lastDay).endOf('week');

    const days = [];
    const currentDay = startDate.clone();
    
    // Initialize weeks and days
    while (currentDay.isSameOrBefore(endDate, 'day')) {
      days.push({
        date: currentDay.clone(),
        isCurrentMonth: currentDay.month() === moment(currentDate).month(),
        isCurrentYear: currentDay.year() === year,
      });
      currentDay.add(1, 'day');
    }

    return days;
  }, [year, currentDate]);
  
  // Count completed tasks per day
  const taskCounts = useMemo(() => {
    const counts = {};
    events.forEach(event => {
      if (event.completed) {
        const dateKey = moment(event.start).format('YYYY-MM-DD');
        counts[dateKey] = (counts[dateKey] || 0) + 1;
      }
    });
    return counts;
  }, [events]);
  
  // Find max tasks for color scaling
  const maxTaskCount = useMemo(() => {
    return Math.max(1, ...Object.values(taskCounts));
  }, [taskCounts]);
  
  // Group days by week number
  const weeks = useMemo(() => {
    const result = [];
    let currentWeek = [];
    
    yearData.forEach((day, index) => {
      currentWeek.push(day);
      
      // Start a new week after every 7 days
      if (currentWeek.length === 7) {
        result.push(currentWeek);
        currentWeek = [];
      }
    });
    
    return result;
  }, [yearData]);
  
  // Function to determine cell color based on task count
  const getCellColor = (date) => {
    const dateKey = date.format('YYYY-MM-DD');
    const count = taskCounts[dateKey] || 0;
    
    if (count === 0) return 'transparent';
    
    // Calculate color intensity (0-1)
    const intensity = count / maxTaskCount;
    
    if (intensity < 0.2) return 'rgba(0, 100, 0, 0.2)';
    if (intensity < 0.4) return 'rgba(0, 130, 0, 0.4)';
    if (intensity < 0.6) return 'rgba(0, 160, 0, 0.6)';
    if (intensity < 0.8) return 'rgba(0, 190, 0, 0.8)';
    return 'rgba(0, 230, 0, 1.0)';
  };
  
  // Month labels for the top of the graph
  const monthLabels = useMemo(() => {
    const months = [];
    for (let i = 0; i < 12; i++) {
      months.push(moment([year, i, 1]).format('MMM'));
    }
    return months;
  }, [year]);

  return (
    <div style={{
      padding: '1rem',
      background: '#121212',
      color: '#fff',
      fontFamily: 'sans-serif'
    }}>
      <h2 style={{ textAlign: 'center', marginBottom: '1.5rem' }}>
        {year} Activity
      </h2>
      
      {/* Month labels */}
      <div style={{ display: 'flex', marginBottom: '0.5rem', marginLeft: '2rem' }}>
        {monthLabels.map((month, idx) => (
          <div 
            key={month} 
            style={{ 
              flex: '1', 
              textAlign: 'center',
              fontWeight: moment(currentDate).month() === idx ? 'bold' : 'normal'
            }}
          >
            {month}
          </div>
        ))}
      </div>
      
      <div style={{ display: 'flex' }}>
        {/* Day of week labels */}
        <div style={{ marginRight: '0.5rem', display: 'flex', flexDirection: 'column', justifyContent: 'space-around' }}>
          {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map(day => (
            <div key={day} style={{ height: '14px', fontSize: '10px', textAlign: 'right' }}>{day}</div>
          ))}
        </div>
        
        {/* Calendar grid */}
        <div style={{ display: 'flex', flexWrap: 'nowrap', overflowX: 'auto' }}>
          {weeks.map((week, weekIndex) => (
            <div key={weekIndex} style={{ display: 'flex', flexDirection: 'column', margin: '0 1px' }}>
              {week.map((day) => {
                const dateKey = day.date.format('YYYY-MM-DD');
                const taskCount = taskCounts[dateKey] || 0;
                
                return (
                  <div 
                    key={dateKey}
                    title={`${day.date.format('YYYY-MM-DD')}: ${taskCount} completed tasks`}
                    style={{
                      width: '14px',
                      height: '14px',
                      margin: '1px',
                      backgroundColor: getCellColor(day.date),
                      border: day.isCurrentYear ? 
                        (day.isCurrentMonth ? '1px solid #444' : '1px solid #333') : 
                        '1px solid #222',
                      borderRadius: '2px',
                      opacity: day.isCurrentYear ? 1 : 0.3
                    }}
                  />
                );
              })}
            </div>
          ))}
        </div>
      </div>
      
      {/* Legend */}
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', marginTop: '1.5rem', gap: '0.5rem' }}>
        <span>Less</span>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'transparent', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 100, 0, 0.2)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 130, 0, 0.4)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 160, 0, 0.6)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 190, 0, 0.8)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <div style={{ width: '14px', height: '14px', backgroundColor: 'rgba(0, 230, 0, 1.0)', border: '1px solid #333', borderRadius: '2px' }}></div>
        <span>More</span>
      </div>
      
      <div style={{ textAlign: 'center', marginTop: '1rem', fontSize: '0.9rem', color: '#aaa' }}>
        {events.filter(event => event.completed && moment(event.start).year() === year).length} tasks completed in {year}
      </div>
    </div>
  );
};

export default YearView;
