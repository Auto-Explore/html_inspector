# html/semantics/forms/the-label-element/iframe-label-attributes.html

Counts:
- errors: 2
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-label-element/iframe-label-attributes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
  <body>
    <label>
      <div id="div1"></div>
    </label>
    <label for="test13"></label>
  </body>
</html>
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
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “label” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 49,
        "byte_start": 34,
        "col": 7,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.label.for.must_reference_non_hidden_control",
      "message": "The value of the “for” attribute of the “label” element must be the ID of a non-hidden form control.",
      "severity": "Error",
      "span": {
        "byte_end": 93,
        "byte_start": 73,
        "col": 5,
        "line": 6
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
  "source_name": "html/semantics/forms/the-label-element/iframe-label-attributes.html"
}
```
