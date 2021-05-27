import { h, JSX } from "preact";
import Home from "./components/home/Home";
import { Nested } from "./components/nested/Nested";
import { Page } from "./components/page/Page";
import { NotFound } from "./NotFound";

type TRoute = {
    content: () => JSX.Element;
};

export function router(location: Location): JSX.Element {
    const path = location.pathname.replace(/\/$/, "");

    const routes: Record<string, TRoute> = {
        "": {
            content: () => <Home />,
        },
        "/page": {
            content: () => <Page />,
        },
        "/page/nested": {
            content: () => <Nested />,
        },
    };

    const route: { content: () => JSX.Element } = routes[path];
    return route ? route.content() : <NotFound />;
}
