# html/rendering/widgets/input-number-text-size-ref.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-number-text-size-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<input type=number><br>
<input type=number><br>
<input type=number><br>
<input type=number size=10><br>
<input type=number><br>
<input type=number size=1><br>
<input type=number size=2><br>
<input type=number size=3><br>
<input type=number size=4><br>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.size.disallowed_for_type",
      "message": "Attribute “size” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 115,
        "byte_start": 88,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.input.size.disallowed_for_type",
      "message": "Attribute “size” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 170,
        "byte_start": 144,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.input.size.disallowed_for_type",
      "message": "Attribute “size” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 201,
        "byte_start": 175,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.input.size.disallowed_for_type",
      "message": "Attribute “size” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 232,
        "byte_start": 206,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.input.size.disallowed_for_type",
      "message": "Attribute “size” not allowed on element “input” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 263,
        "byte_start": 237,
        "col": 1,
        "line": 10
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
  "source_name": "html/rendering/widgets/input-number-text-size-ref.html"
}
```
