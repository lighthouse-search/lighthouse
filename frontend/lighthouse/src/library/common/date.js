function time_since(since) {
    // Get the current date and time
    let now = new Date();

    // Get the UNIX timestamp in milliseconds
    let nowMs = now.getTime();

    // Convert the UNIX timestamp to milliseconds
    let targetMs = since;

    // Calculate the difference in milliseconds
    let diffMs = nowMs - targetMs;

    let years,
    months,
    days,
    hours,
    minutes,
    seconds;

    // Check if the difference is positive
    if (diffMs > 0) {
        // Convert the difference to years, days, hours, minutes, seconds
        years = Math.floor(diffMs / (1000 * 60 * 60 * 24 * 365));
        months = Math.floor((diffMs % (1000 * 60 * 60 * 24 * 365)) / (1000 * 60 * 60 * 24 * 30));
        days = Math.floor((diffMs % (1000 * 60 * 60 * 24 * 30)) / (1000 * 60 * 60 * 24));
        hours = Math.floor((diffMs % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
        minutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));
        seconds = Math.floor((diffMs % (1000 * 60)) / 1000);

        // if (days < 10) {
        //     days = "0" + days;
        // }
        // if (hours < 10) {
        //     hours = "0" + hours;
        // }
        // if (minutes < 10) {
        //     minutes = "0" + minutes;
        // }
        // if (seconds < 10) {
        //     seconds = "0" + seconds;
        // }
    } else {
        // Display a message if the target time has passed
        console.log("The target time has passed.");
        years = -1;
        months = -1;
        days = -1;
        hours = -1;
        minutes = -1;
        seconds = -1;
    }

    return { years: years, months: months, days: days, hours: hours, minutes: minutes, seconds: seconds };
}
  
function get_ago(since) {
    let timeframe;
    let timevalue;
  
    const times = time_since(new Date(since).getTime());
  
    let years = times.years;
    let months = times.months;
    let days = times.days;
    let hours = times.hours;
    let minutes = times.minutes
    let seconds = times.seconds
  
    if (years) {
        timeframe = "years";
        timevalue = years;
    } else if (months) {
        if (months == 1) {
            timeframe = "month";
        } else {
            timeframe = "months";
        }
        timevalue = months;
    } else if (days) {
        if (days == 1) {
            timeframe = "day";
        } else {
            timeframe = "days";
        }
        timevalue = days;
    } else if (hours) {
        if (hours == 1) {
            timeframe = "hour";
        } else {
            timeframe = "hours";
        }
        timevalue = hours;
    } else if (minutes) {
        if (minutes == 1) {
            timeframe = "minute";
        } else {
            timeframe = "minutes";
        }
        timevalue = minutes;
    } else {
        if (seconds == 1) {
            timeframe = "second";
        } else {
            timeframe = "seconds";
        }
        timevalue = seconds;
    }
  
    return { timevalue: timevalue, timeframe: timeframe, string: `${timevalue} ${timeframe}`, raw: times };
  }
  
function number_to_month(num) {
    let month_list = ["January", "Feburary", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    return month_list[num];
}

export { time_since, get_ago, number_to_month };