function updateDurationOptions() {
    const frequency = document.getElementById('frequency').value;
    const duration = document.getElementById('duration');

    while (duration.firstChild) {
        duration.removeChild(duration.firstChild);
    }

    if (frequency === 'daily') {
        for (let i = 1; i <= 24; i++) {
            duration.add(new Option(`${i} hour(s) per day`, i));
        }
    } else if (frequency === 'weekly') {
        for (let i = 1; i <= 7; i++) {
            duration.add(new Option(`${i} day(s) per week`, i));
        }
    } else if (frequency === 'biweekly') {
        for (let i = 1; i <= 14; i++) {
            duration.add(new Option(`${i} day(s) per two weeks`, i));
        }
    } else if (frequency === 'monthly') {
        for (let i = 1; i <= 31; i++) {
            duration.add(new Option(`${i} day(s) per month`, i));
        }
    } else if (frequency === 'annually') {
        for (let i = 1; i <= 12; i++) {
            duration.add(new Option(`${i} month(s) per year`, i));
        }
    }
}

function predictImpact() {
    const behavior = document.getElementById('behavior').value;
    const frequency = document.getElementById('frequency').value;
    const duration = document.getElementById('duration').value;

    const resultText = `Predicting the climate impact of ${behavior}, with a frequency of ${frequency} and a duration of ${duration}.`;

    document.getElementById('result').innerText = resultText;
}

document.addEventListener('DOMContentLoaded', () => {
    updateDurationOptions();
});
