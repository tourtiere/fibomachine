import { h } from "preact";
import { useContext, useEffect } from "preact/hooks";
import { FiboContext } from "providers/fibo";
import "./OutputPanel.scss";

export default () => {
    const { result } = useContext(FiboContext);

    useEffect(() => {
        console.log("output panel", result);
    }, [result]);

    if (result === undefined) return null;
    if (!result.success) return null;

    return (
        <div class="output">
            <div class="output-row" key={"headline"}>
                <div class="output-index">Index</div>
                <div class="output-value">Number</div>
            </div>
            {result.values.map((value, idx) => (
                <div class="output-row" key={idx + value}>
                    <div class="output-index">{idx.toString()}</div>
                    <div class="output-value">{value}</div>
                </div>
            ))}
        </div>
    );
};
