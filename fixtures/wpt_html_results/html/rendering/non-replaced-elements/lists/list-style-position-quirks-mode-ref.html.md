# html/rendering/non-replaced-elements/lists/list-style-position-quirks-mode-ref.html

Counts:
- errors: 0
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/list-style-position-quirks-mode-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
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
  <li style="list-style-position: inside">unspecified</li>
  <li style="list-style-position: outside">outside</li>
  <li style="list-style-position: outside">initial</li>
  <li style="list-style-position: outside">inherit</li>
  <li style="list-style-position: outside">unset</li>
  <li style="list-style-position: inside">revert</li>
  <li style="list-style-position: inside">revert-layer</li>
</div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 302,
        "byte_start": 262,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 362,
        "byte_start": 321,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 418,
        "byte_start": 377,
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
        "byte_end": 474,
        "byte_start": 433,
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
        "byte_end": 530,
        "byte_start": 489,
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
        "byte_end": 583,
        "byte_start": 543,
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
        "byte_end": 637,
        "byte_start": 597,
        "col": 3,
        "line": 20
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
  "source_name": "html/rendering/non-replaced-elements/lists/list-style-position-quirks-mode-ref.html"
}
```
