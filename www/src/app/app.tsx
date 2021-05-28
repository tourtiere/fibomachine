import { Component, h, JSX } from "preact";
import { FiboProvider } from "../providers/fibo";
import "./app.scss";
import { router } from "./router";

export class App extends Component {
    render(): JSX.Element {
        return <FiboProvider>{router(location)}</FiboProvider>;
    }
}
