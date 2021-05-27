import { Component, JSX, h } from "preact";
import { FiboProvider } from "../providers/fibo";
import { router } from "./router";

export class App extends Component {
    render(): JSX.Element {
        return <FiboProvider>{router(location)}</FiboProvider>;
    }
}
