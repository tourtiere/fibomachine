import { foldNodeProp, foldInside, indentNodeProp } from "@codemirror/language";
import { styleTags, tags as t } from "@codemirror/highlight";
import { LezerLanguage } from "@codemirror/language";
import { parser } from "./parser";
import { HighlightStyle, tags } from "@codemirror/highlight";

let parserWithMetadata = parser.configure({
    props: [
        styleTags({
            Identifier: t.variableName,
            Boolean: t.bool,
            String: t.string,
            Int: t.number,
            Application: t.strong,
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
    { tag: tags.number, color: theme.blue2 },
    { tag: tags.variableName, color: theme.orange },
    { tag: tags.paren, color: theme.green },
]);

export const language = LezerLanguage.define({
    parser: parserWithMetadata,
});
