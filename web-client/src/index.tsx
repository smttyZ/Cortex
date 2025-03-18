import React from "react";
import ReactDOM from "react-dom/client";

let date = new Date();

const App = () => {
    return (
        <div>
            <h1>Oracle GUI</h1>
            <p>Current time: {date.toLocaleTimeString()}</p>
        </div>
    )
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(<App />);
