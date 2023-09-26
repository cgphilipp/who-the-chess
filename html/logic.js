// this is all stuff that should be handled on the server :)
// ideally using a database so requests for the same game can be handled
// on multiple servers
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
        const array = new Uint32Array(1);
        crypto.getRandomValues(array);

        this._currentGameId = array[0];
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
    // add delay in case the server determines a correct answer
    setTimeout(() => {
        var submitElement = document.getElementById("answerBox");
        if (submitElement) {
            submitElement.classList.add("flash-red");

            // remove the flashing after a timeout to allow it to be reapplied
            setTimeout(() => {
                submitElement.classList.remove("flash-red");
            }, 1000);
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
            prediction.innerHTML = "";
        }
    }
}

// handle tab autocomplete and enter to start games
htmx.on("keydown", function (evt) {
    if (evt.key === 'Tab') {
        copyPrediction();
        evt.preventDefault();
        evt.stopPropagation();
    }

    if (evt.key === 'Enter') {
        const startButtons = document.getElementsByClassName("start-button");
        if (startButtons.length > 0) {
            startButtons[0].click();
            evt.preventDefault();
            evt.stopPropagation();
        }
    }
});

// handle resize and communicate height changes to CSS
const documentHeight = () => {
    var doc = document.documentElement;
    doc.style.setProperty('--doc-height', `${window.innerHeight}px`);
}
window.addEventListener('resize', documentHeight);
documentHeight();
