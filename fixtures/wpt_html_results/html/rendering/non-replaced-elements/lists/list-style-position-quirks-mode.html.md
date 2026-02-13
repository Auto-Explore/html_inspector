# html/rendering/non-replaced-elements/lists/list-style-position-quirks-mode.html

Counts:
- errors: 1
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/list-style-position-quirks-mode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- quirks mode -->
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#lists">
<link rel="match" href="list-style-position-quirks-mode-ref.html">
<style>
  body {
    /* Increase left margin to ensure we can see the list item's marker. */
    margin-left: 50px;
  }
  .wrapper {
    border: 2px solid teal;
    width: max-content;
    margin-bottom: 2px;
  }
</style>
<div class="wrapper">
  <li>unspecified</li>
  <li style="list-style-position: outside">outside</li>
  <li style="list-style-position: initial">initial</li>
  <li style="list-style-position: inherit">inherit</li>
  <li style="list-style-position: unset">unset</li>
  <li style="list-style-position: revert">revert</li>
  <li style="list-style-position: revert-layer">revert-layer</li>
</div>
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
        "byte_end": 105,
        "byte_start": 21,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 423,
        "byte_start": 419,
        "col": 3,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 483,
        "byte_start": 442,
        "col": 3,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 539,
        "byte_start": 498,
        "col": 3,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 595,
        "byte_start": 554,
        "col": 3,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 649,
        "byte_start": 610,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 702,
        "byte_start": 662,
        "col": 3,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 762,
        "byte_start": 716,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/lists/list-style-position-quirks-mode.html"
}
```
