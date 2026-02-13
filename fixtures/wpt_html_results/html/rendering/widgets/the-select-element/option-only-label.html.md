# html/rendering/widgets/the-select-element/option-only-label.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/the-select-element/option-only-label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>OPTION's label attribute in SELECT -- Only a label</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
<link rel="match" href="option-label-ref.html">
<meta name="assert" content="An option element is expected to be rendered by displaying the element's label.">

<select>
  <option label="Label Text"></option>
</select>
<br/>
<select size="4">
  <option label="Label Text"></option>
</select>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/widgets/the-select-element/option-only-label.html"
}
```
