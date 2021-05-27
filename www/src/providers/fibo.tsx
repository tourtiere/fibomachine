import { createContext, FunctionalComponent, h } from "preact";
import { useEffect, useState } from "preact/hooks";

interface Success {
    success: true;
    values: string[];
}
interface Fail {
    success: false;
    errorType: "OutOfBounds" | "Type" | "Count" | "Undefined" | "Other";
    range?: number[];
}

export type Result = Success | Fail;

interface FiboContext {
    setResult: (result: Result) => void;
    result?: Result;
}
export const FiboContext = createContext<FiboContext>({
    setResult: (_: Result) => ({}),
});

export const FiboProvider: FunctionalComponent = (props) => {
    const [result, setResult] = useState<Result | undefined>(undefined);
    useEffect(() => {
        console.log("changed", result);
    }, [result]);
    return (
        <FiboContext.Provider
            value={{
                result,
                setResult,
            }}
        >
            {props.children}
        </FiboContext.Provider>
    );
};
