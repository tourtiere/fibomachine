import { HighlightStyle, styleTags, tags as t, tags } from "@codemirror/highlight";
import { foldInside, foldNodeProp, indentNodeProp, LezerLanguage } from "@codemirror/language";
import { parser } from "./parser";

let parserWithMetadata = parser.configure({
    props: [
        styleTags({
            Index: t.variableName,
            Function: t.float,
            Boolean: t.bool,
            Int: t.number,
            Application: t.strong,
            RightSide: t.list,
            "( )": t.paren,
        }),
        indentNodeProp.add({
            Application: (context) => context.column(context.node.from) + context.unit,
        }),
        foldNodeProp.add({
            Application: foldInside,
        }),
    ],
});

// Atome one light theme
const theme = {
    purple: "rgb(156,45,154)",
    green: "rgb(83,160,84)",
    orange: "rgb(227,93,84)",
    brown: "rgb(153,107,24)",
    blue1: "rgb(68,123,239)",
    blue2: "rgb(27,138,189)",
};

export const highlight = HighlightStyle.define([
    { tag: tags.number, color: theme.blue1 },
    { tag: tags.variableName, color: theme.green },
    { tag: tags.float, color: theme.green },
    { tag: tags.paren, color: theme.blue2 },
    { tag: tags.list, color: theme.purple },
]);

export const language = LezerLanguage.define({
    parser: parserWithMetadata,
});
