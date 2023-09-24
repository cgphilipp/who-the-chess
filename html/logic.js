class GameLogic {
    constructor() {
        this._currentGameId = 1337;
        this._currentHintId = 0;
        this._appStartTime = 0;
        this._gameStartTime = 0;
        this._correctAnswers = 0;
        this._totalAnswers = 0;
    }

    initApp() {
        this._appStartTime = Date.now();
    }

    generateGameId() {
        this._currentGameId = Math.floor(Math.random() * 16777215);
        this._currentHintId = 0;
        this._gameStartTime = Date.now();
        return this._currentGameId;
    }

    getGameId() {
        return this._currentGameId;
    }

    getHintId() {
        return ++this._currentHintId;
    }

    getGameTime() {
        const timeInSec = Math.floor((Date.now() - this._gameStartTime) / 1000.0);
        return timeInSec + "s";
    }

    getAppTime() {
        const timeInSec = Math.floor((Date.now() - this._appStartTime) / 1000.0);
        return timeInSec + "s";
    }

    addCorrectAnswer() {
        this._correctAnswers++;
        this._totalAnswers++;
    }

    addWrongAnswer() {
        this._totalAnswers++;
    }

    getCorrectAnswers() {
        return this._correctAnswers;
    }

    getTotalAnswers() {
        return this._totalAnswers;
    }
}

const gameLogic = new GameLogic();

function getCurrentAnswer() {
    var submitElement = document.getElementById("answerBox");
    if (!submitElement) {
        return "";
    }
    return submitElement.value;
}

function flashAnswer() {
    // add delay of 50ms in case the server determines a correct answer
    setTimeout(() => {
        var submitElement = document.getElementById("answerBox");
        if (submitElement) {
            submitElement.classList.add("flash-red");

            // remove the flashing after a timeout to allow it to be reapplied
            setTimeout(() => {
                submitElement.classList.remove("flash-red");
            }, 500);
        }
    }, 50);

}

function copyPrediction() {
    var submitElement = document.getElementById("answerBox");
    var predictionElement = document.getElementById("predictionBox");
    if (submitElement && predictionElement) {
        submitElement.value = predictionElement.value;

        var prediction = document.getElementById("prediction");
        if (prediction) {
            prediction.style.display = "none";
        }
    }
}

// handle tab autocomplete
htmx.on("keydown", function (evt) {
    if (evt.key === 'Tab') {
        copyPrediction();
        evt.preventDefault();
        evt.stopPropagation();
    }
});

// handle resize and communicate height changes to CSS
const documentHeight = () => {
    var doc = document.documentElement;
    doc.style.setProperty('--doc-height', `${window.innerHeight}px`);
}
window.addEventListener('resize', documentHeight);
documentHeight();
