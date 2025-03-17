// Select the greeting div
const greetingDiv = document.querySelector("#greeting p");

// Function to set text inside the div
function setGreetingText(text: string): void {
    if (greetingDiv) {
        greetingDiv.textContent = text;
    } else {
        console.error("Greeting element not found");
    }
}

// Call the function with your desired text
setGreetingText("Welcome to the Oracle GUI!");
