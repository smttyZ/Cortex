import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom/client";

const ClockWidget: React.FC = () => {
    const [date, setDate] = useState(new Date());

    useEffect(() => {
        const interval = setInterval(() => {
            setDate(new Date()); // Update time every second
        }, 1000);

        return () => clearInterval(interval); // Cleanup when component unmounts
    }, []);

    return (
        <div style={{ 
            border: "1px solid #ccc", 
            padding: "10px", 
            borderRadius: "8px", 
            width: "250px",
            backgroundColor: "#282c34", 
            color: "#ffffff",
            fontFamily: "Arial, sans-serif",
            textAlign: "center"
        }}>
            <h2>Clock Widget</h2>
            <p>Local Time: {date.toLocaleTimeString()}</p>
            <p>UTC Time: {date.toUTCString()}</p>
            <p>ISO Format: {date.toISOString()}</p>
            <p>Timezone Offset: {date.getTimezoneOffset()} minutes</p>
            <p>Milliseconds: {date.getMilliseconds()}</p>
        </div>
    );
};

const App = () => {
    return (
        <div>
            <h1>Oracle GUI</h1>
            <ClockWidget />
        </div>
    );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(<App />);