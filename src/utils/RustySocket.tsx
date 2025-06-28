import { appWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";


interface RustyPipe {
    payload: string;
  }

 
type OnMessageCallback = (message: string) => void;

export const RustySocketConnection = (onMessage: OnMessageCallback) => {
  useEffect(() => {
    
    const rustyPipe = appWindow.listen(
      "rustysocket-message",
      (event: RustyPipe) => onMessage(event.payload));

    return () => {
      rustyPipe.then((dispose) => dispose());
    };
  }, []);
};

export const RustySocketConnection2 = (onMessage: OnMessageCallback) => {
  useEffect(() => {
    
    const rustyPipe = appWindow.listen(
      "rustysocket-message2",
      (event: RustyPipe) => onMessage(event.payload));

    return () => {
      rustyPipe.then((dispose) => dispose());
    };
  }, []);
};

export const RustySocketConnection3 = (onMessage: OnMessageCallback) => {
  useEffect(() => {
    
    const rustyPipe = appWindow.listen(
      "rustysocket-message3",
      (event: RustyPipe) => onMessage(event.payload));

    return () => {
      rustyPipe.then((dispose) => dispose());
    };
  }, []);
};

export default RustySocketConnection;
