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
            <div class="row header" key={"headline"}>
                <div class="index">n</div>
                <div class="value">a(n)</div>
            </div>
            {result.values.map((value, idx) => (
                <div class="row" key={idx + value}>
                    <div class="index">{idx.toString()}</div>
                    <div class="value">{value}</div>
                </div>
            ))}
        </div>
    );
};
