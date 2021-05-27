import { h } from "preact";
import { useContext, useEffect } from "preact/hooks";
import { FiboContext } from "providers/fibo";

export default () => {
    const { result } = useContext(FiboContext);
    useEffect(() => {
        console.log("output panel", result);
    }, [result]);
    if (result === undefined) return null;
    if (!result.success) {
        return null;
    }
    return (
        <div>
            <p>test</p>
            <div>
                {result.values.map((value, idx) => (
                    <p key={idx + value}>{value}</p>
                ))}
            </div>
        </div>
    );
};
