# html/rendering/non-replaced-elements/tables/table-border-3q.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-border-3q.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- Intentionally omitting doctype, to test quirks mode. -->
<head>
  <title>Testing default 'border-color' on table (with 'color' set), in quirks mode</title>
  <meta charset="utf-8">
  <link rel="author" title="Daniel Holbert" href="mailto:dholbert@mozilla.com">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
  <link rel="match" href="table-border-3-ref.html">
  <style>
    * {
      /* This sets the used value of 'currentColor', which is what should be
         used for all border-coloring in this test: */
      color: teal;
      /* This only affects the elements that we specify 'border-style' on: */
      border-width: 6px;
    }

    table {
      height: 30px;
      width: 30px;
      border-spacing: 0;

      /* To test in "rows": */
      float: left;
      margin: 1px;
    }
    br {
      clear: both;
    }

    .dotted {
      border-style: dotted;
    }
    .dashed {
      border-style: dashed;
    }
    .solid {
      border-style: solid;
    }
    .double {
      border-style: double;
    }
    .groove {
      border-style: groove;
    }
    .ridge {
      border-style: ridge;
    }
    .inset {
      border-style: inset;
    }
    .outset {
      border-style: outset;
    }
  </style>
</head>

<table class="dotted"><td></td></table>
<table><th class="dotted"></th></table>
<table><td class="dotted"></td></table>
<br>

<table class="dashed"><td></td></table>
<table><th class="dashed"></th></table>
<table><td class="dashed"></td></table>
<br>

<table class="solid"><td></td></table>
<table><th class="solid"></th></table>
<table><td class="solid"></td></table>
<br>

<table class="double"><td></td></table>
<table><th class="double"></th></table>
<table><td class="double"></td></table>
<br>

<table class="groove"><td></td></table>
<table><th class="groove"></th></table>
<table><td class="groove"></td></table>
<br>

<table class="ridge"><td></td></table>
<table><th class="ridge"></th></table>
<table><td class="ridge"></td></table>
<br>

<table class="inset"><td></td></table>
<table><th class="inset"></th></table>
<table><td class="inset"></td></table>
<br>

<table class="outset"><td></td></table>
<table><th class="outset"></th></table>
<table><td class="outset"></td></table>
<br>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 68,
        "byte_start": 62,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/tables/table-border-3q.html"
}
```
