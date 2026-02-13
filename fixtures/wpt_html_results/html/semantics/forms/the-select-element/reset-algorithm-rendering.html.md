# html/semantics/forms/the-select-element/reset-algorithm-rendering.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/reset-algorithm-rendering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Invalidation test on resetting &lt;select></title>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-select-element:concept-form-reset-control">
<link rel="help" href="http://crbug.com/1087031">
<link rel="match" href="reset-algorithm-rendering-ref.html">
<body>

<form>
<select>
<option>Default</option>
<option>Another</option>
</select>

<select>
<option>Another</option>
<option selected>Default</option>
</select>

<select multiple>
<option>option 1</option>
<option>option 2</option>
</select>
</form>

<script>
const selects = document.querySelectorAll('select');
selects[0].selectedIndex = 1;
selects[1].selectedIndex = 0;
selects[2].options[1].selected = true;

document.documentElement.addEventListener('TestRendered', e => {
  document.querySelector('form').reset();
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
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/reset-algorithm-rendering.html"
}
```
