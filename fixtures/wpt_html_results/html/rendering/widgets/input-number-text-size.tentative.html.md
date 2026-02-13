# html/rendering/widgets/input-number-text-size.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-number-text-size.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-input-element-as-a-text-entry-widget">
<link rel="help" href="https://github.com/whatwg/html/issues/10390">
<title>Test `size` attribute behavior on number input</title>
<link rel="match" href="input-number-text-size-ref.html">
<input type=number><br>
<input type=number min=0><br>
<input type=number max=1><br>
<input type=number size=10 min=0 max=1><br>
<input type=number min=0 max=1 step=any><br>
<input type=number min=0 max=1><br>
<input type=number min=-1 max=1><br>
<input type=number min=0 max=1 step=0.5><br>
<input type=number min=-1 max=1 step=0.5><br>
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
        "byte_end": 426,
        "byte_start": 387,
        "col": 1,
        "line": 9
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
  "source_name": "html/rendering/widgets/input-number-text-size.tentative.html"
}
```
