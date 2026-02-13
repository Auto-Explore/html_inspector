# html/rendering/non-replaced-elements/lists/TODO-lists.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/TODO-lists.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<ol><div><li>A</div></ol>
<ol><div><li>A</div> <li>B</ol>
<ol><div><li>A</div><div><li>B</div></ol>
<ol reversed><div><li>A</div> <li>B</ol>
<ol><div style=display:list-item>A</div><li>B</ol>
<ol reversed><div style=display:list-item>A</div><li>B</ol>
<ol reversed>
  <div><li>Two</li></div>
  <li>One</li>
  <li>Zero</li>
</ol>
<ol reversed>
  <li>Three</li>
  <li style="display: none"></li>
  <li>Two</li>
</ol>
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
        "byte_end": 29,
        "byte_start": 25,
        "col": 10,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 55,
        "byte_start": 51,
        "col": 10,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 87,
        "byte_start": 83,
        "col": 10,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 103,
        "byte_start": 99,
        "col": 26,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 138,
        "byte_start": 134,
        "col": 19,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 293,
        "byte_start": 289,
        "col": 8,
        "line": 9
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
  "source_name": "html/rendering/non-replaced-elements/lists/TODO-lists.html"
}
```
