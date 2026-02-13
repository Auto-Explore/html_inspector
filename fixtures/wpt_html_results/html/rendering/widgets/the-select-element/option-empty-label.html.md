# html/rendering/widgets/the-select-element/option-empty-label.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/the-select-element/option-empty-label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>OPTION's label attribute in SELECT -- Empty label uses Element text</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
<link rel="match" href="option-label-ref.html">
<meta name="assert" content="An option element is expected to be rendered by displaying the element's label.">

<select>
  <option label>Label Text</option>
</select>
<br/>
<select size="4">
  <option label>Label Text</option>
</select>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.label.empty",
      "message": "Bad value “” for attribute “label” on element “option”.",
      "severity": "Warning",
      "span": {
        "byte_end": 407,
        "byte_start": 393,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.option.label.empty",
      "message": "Bad value “” for attribute “label” on element “option”.",
      "severity": "Warning",
      "span": {
        "byte_end": 477,
        "byte_start": 463,
        "col": 3,
        "line": 13
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
  "source_name": "html/rendering/widgets/the-select-element/option-empty-label.html"
}
```
