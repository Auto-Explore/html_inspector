# html/semantics/forms/the-option-element/dynamic-content-change-rendering.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-option-element/dynamic-content-change-rendering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Invalidation test on resetting &lt;select></title>
<link rel="help" href="https://html.spec.whatwg.org/C/#concept-option-label">
<link rel="help" href="http://crbug.com/1090806">
<link rel="match" href="dynamic-content-change-rendering-ref.html">
<meta name=fuzzy content="maxDifference=0-3;totalPixels=0-20">
<body>

<select id="dropdown">
<option></option>
</select>

<select id="listbox" multiple>
<option></option>
</select>

<script>
const selects = document.querySelectorAll('select');

const span0 = document.createElement('span');
selects[0].options[0].appendChild(span0);

const span1 = document.createElement('span');
selects[1].options[0].appendChild(span1);

document.documentElement.addEventListener('TestRendered', e => {
  span0.textContent = 'foo';
  span1.textContent = 'bar';
  e.target.removeAttribute('class');
});
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 409,
        "byte_start": 400,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 469,
        "byte_start": 460,
        "col": 9,
        "line": 15
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
  "source_name": "html/semantics/forms/the-option-element/dynamic-content-change-rendering.html"
}
```
