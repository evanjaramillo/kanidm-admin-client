import React from "react";
import ReactDOM from "react-dom/client";
import '@mantine/core/styles.css';
import { createTheme, MantineProvider } from '@mantine/core';
import LoginWindow from "./LoginWindow";
import { ErrorBoundary } from "react-error-boundary";
import { error as err } from "../logger";

const theme = createTheme({
  /** Put your mantine theme override here */
});

const handler = (error, stack) => {
  err("Error boundary tripped in login window. error: {} stack: {}", error, stack);
};

ReactDOM.createRoot(document.getElementById("login-root")).render(
  <React.StrictMode>
    <MantineProvider theme={theme} defaultColorScheme="dark">
      <ErrorBoundary errorHandler={handler}>
        <LoginWindow />
      </ErrorBoundary>
    </MantineProvider>
  </React.StrictMode >
);
