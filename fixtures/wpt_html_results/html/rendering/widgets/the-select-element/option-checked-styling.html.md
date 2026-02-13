# html/rendering/widgets/the-select-element/option-checked-styling.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/the-select-element/option-checked-styling.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Stylability of select>option with :checked pseudo</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help"  href="https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element">
<link rel="match"  href="option-checked-styling-ref.html">

<style>
  option {
    color: black;
    background-color: red;
  }
  option:checked {
    background: green;
  }
</style>

<select id=select size=3 multiple>
  <option id=option1 selected>Option #1 (should be green)</option>
  <option id=option2>Option #2 (should be red)</option>
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
  "source_name": "html/rendering/widgets/the-select-element/option-checked-styling.html"
}
```
